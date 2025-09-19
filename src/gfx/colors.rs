#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Srgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub type Rgb24 = (u8, u8, u8);

pub fn prompt_oklab_bg_colors(term_bg: Srgb) -> (Srgb, Srgb) {
    let bg: Oklab = term_bg.into();

    let tint = if bg.is_dark() {
        Oklab::from((198, 208, 245))
    } else {
        Oklab::from((65, 69, 89))
    };

    let opacity = if bg.is_dark() { 0.21 } else { 0.15 };

    (
        bg.with_safe_luminance().lerp(&tint, opacity).into(),
        bg.with_safe_luminance().lerp(&tint, opacity * 0.56).into(),
    )
}

pub fn prompt_256color_bg_colors(term_bg: Srgb) -> (u8, u8) {
    let bg: Oklab = term_bg.into();

    let adjusts = if bg.is_dark() {
        (0.24, 0.12)
    } else {
        (-0.15, -0.06)
    };

    fn luminance_to_256(l: f64) -> u8 {
        let c = (l * 24.0 + 232.0).round().clamp(232.0, 256.0) as u16;
        if c == 256 { 231 } else { c as u8 }
    }

    (
        luminance_to_256((bg.l + adjusts.0).clamp(0.0, 1.0)),
        luminance_to_256((bg.l + adjusts.1).clamp(0.0, 1.0)),
    )
}

impl Oklab {
    pub fn lerp(&self, other: &Oklab, t: f64) -> Self {
        Self {
            l: lerp(self.l, other.l, t),
            a: lerp(self.a, other.a, t),
            b: lerp(self.b, other.b, t),
        }
    }

    pub fn is_dark(&self) -> bool {
        self.l < 0.5
    }

    // Very dark colors don't interpolate nicely
    pub fn with_safe_luminance(&self) -> Self {
        Self {
            l: self.l.max(0.15),
            a: self.a,
            b: self.b,
        }
    }
}

impl Srgb {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn rgb24_to_srgb((r, g, b): Rgb24) -> Srgb {
    Srgb {
        r: u8_to_f(r),
        g: u8_to_f(g),
        b: u8_to_f(b),
    }
}

fn srgb_to_rgb24(s: Srgb) -> Rgb24 {
    (f_to_u8(s.r), f_to_u8(s.g), f_to_u8(s.b))
}

fn f_to_u8(x: f64) -> u8 {
    (x.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn u8_to_f(x: u8) -> f64 {
    x as f64 / 255.0
}

fn srgb_to_linear(c: f64) -> f64 {
    assert!((0.0..=1.0).contains(&c));
    if c <= 0.040_45 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn srgb_to_oklab(c: Srgb) -> Oklab {
    let r = srgb_to_linear(c.r);
    let g = srgb_to_linear(c.g);
    let b = srgb_to_linear(c.b);

    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    let l_ = l.max(0.0).cbrt();
    let m_ = m.max(0.0).cbrt();
    let s_ = s.max(0.0).cbrt();

    Oklab {
        l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
    }
}

fn oklab_to_srgb(c: Oklab) -> Srgb {
    let l_ = c.l + 0.3963377774 * c.a + 0.2158037573 * c.b;
    let m_ = c.l - 0.1055613458 * c.a - 0.0638541728 * c.b;
    let s_ = c.l - 0.0894841775 * c.a - 1.2914855480 * c.b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

    Srgb {
        r: linear_to_srgb(r),
        g: linear_to_srgb(g),
        b: linear_to_srgb(b),
    }
}

impl From<Oklab> for Srgb {
    fn from(c: Oklab) -> Self {
        oklab_to_srgb(c)
    }
}

impl From<Srgb> for Oklab {
    fn from(c: Srgb) -> Self {
        srgb_to_oklab(c)
    }
}

impl From<Rgb24> for Srgb {
    fn from(c: Rgb24) -> Srgb {
        rgb24_to_srgb(c)
    }
}

impl From<Srgb> for Rgb24 {
    fn from(c: Srgb) -> Rgb24 {
        srgb_to_rgb24(c)
    }
}

impl From<Rgb24> for Oklab {
    fn from(c: Rgb24) -> Oklab {
        srgb_to_oklab(rgb24_to_srgb(c))
    }
}

impl From<Oklab> for Rgb24 {
    fn from(c: Oklab) -> Rgb24 {
        srgb_to_rgb24(oklab_to_srgb(c))
    }
}

#[test]
fn test_linear_roundtrip() {
    for x in 0..=255 {
        let scaled = x as f64 / 255.0;
        let roundtrip = linear_to_srgb(srgb_to_linear(scaled));
        assert_eq!(x, f_to_u8(roundtrip));
    }
}

#[test]
fn test_oklab_roundtrip_all_one_bits() {
    for r in 0..8 {
        for g in 0..8 {
            for b in 0..8 {
                let color: Rgb24 = (1 << r, 1 << g, 1 << b);
                let oklab = Oklab::from(color);
                let rgb24 = Rgb24::from(oklab);
                assert_eq!(color, rgb24);
            }
        }
    }
}

#[test]
fn test_oklab_roundtrip() {
    let colors: Vec<Rgb24> = vec![
        (0, 0, 0),
        (1, 2, 3),
        (255, 255, 255),
        (254, 253, 252),
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (128, 128, 128),
    ];

    for color in colors {
        let oklab = Oklab::from(color);
        let rgb24 = Rgb24::from(oklab);
        assert_eq!(color, rgb24);
    }
}
