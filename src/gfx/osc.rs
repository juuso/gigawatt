use crate::gfx::Srgb;
use nix::libc;
use nix::sys::termios::{SetArg, Termios, cfmakeraw, tcgetattr, tcsetattr};
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::AsFd;

const MAX_OSC_LEN: usize = 64;
const BEL: u8 = 0x07;
const ESC: u8 = 0x1b;
const ST: &[u8] = &[ESC, b'\\'];
const BEL_CHAR: char = BEL as char;
const ESC_CHAR: char = ESC as char;

pub fn term_background() -> Option<Srgb> {
    let mut tty = File::options()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .ok()?;
    let fd = tty.as_fd();

    let original = tcgetattr(fd).ok()?;
    let _guard = TermiosGuard {
        tty: tty.try_clone().ok()?,
        termios: original.clone(),
    };

    let mut raw = original.clone();
    cfmakeraw(&mut raw);

    raw.control_chars[libc::VMIN] = 0;
    raw.control_chars[libc::VTIME] = 10;

    tcsetattr(fd, SetArg::TCSANOW, &raw).ok()?;
    tty.write_all(b"\x1b]11;?\x07").ok()?;

    let reply = read_osc_reply(&mut tty)?;
    parse_osc_reply(&reply)
}

fn read_osc_reply(tty: &mut File) -> Option<String> {
    let mut buf: Vec<u8> = Vec::with_capacity(MAX_OSC_LEN);
    let mut byte = [0u8; 1];

    loop {
        match tty.read(&mut byte).ok()? {
            1 => {
                buf.extend(byte);

                if buf.ends_with(&[BEL]) || buf.ends_with(ST) {
                    return String::from_utf8(buf).ok();
                }

                if buf.len() >= MAX_OSC_LEN {
                    return None;
                }
            }
            _ => return None,
        }
    }
}

fn parse_osc_reply(reply: &str) -> Option<Srgb> {
    let (r, g, b) = hex_rgb_from_osc_reply(reply)?;
    Some((hex_to_u8(r)?, hex_to_u8(g)?, hex_to_u8(b)?).into())
}

fn hex_rgb_from_osc_reply(reply: &str) -> Option<(&str, &str, &str)> {
    let reply = reply.trim_end_matches(|c| matches!(c, BEL_CHAR | ESC_CHAR | '\\'));
    let (_, rgb) = reply.split_once(':')?;
    let mut it = rgb.split('/');
    Some((it.next()?, it.next()?, it.next()?))
}

fn hex_to_u8(hex: &str) -> Option<u8> {
    let max = match hex.len() {
        1 => 0xf,
        2 => 0xff,
        3 => 0xfff,
        4 => 0xffff,
        _ => return None,
    };

    let v = u32::from_str_radix(hex, 16).ok()?;
    let r = (0xff * v + max / 2) / max;
    Some(r as u8)
}

struct TermiosGuard {
    tty: File,
    termios: Termios,
}

impl Drop for TermiosGuard {
    fn drop(&mut self) {
        _ = tcsetattr(&self.tty, SetArg::TCSADRAIN, &self.termios);
    }
}

#[test]
fn test_hex_to_u8() {
    assert_eq!(hex_to_u8("1"), Some(0x11));
    assert_eq!(hex_to_u8("a"), Some(0xaa));
    assert_eq!(hex_to_u8("f"), Some(0xff));
    assert_eq!(hex_to_u8("01"), Some(1));
    assert_eq!(hex_to_u8("ffff"), Some(0xff));
    assert_eq!(hex_to_u8("00001"), None);
    assert_eq!(hex_to_u8("1x"), None);
    assert_eq!(hex_to_u8(""), None);
}
