use core::fmt;

#[derive(PartialEq, Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim_start_matches('v');
        let mut parts = s.split('.').map(str::parse::<u32>);
        let major = parts.next().unwrap_or(Ok(0)).ok()?;
        let minor = parts.next().unwrap_or(Ok(0)).ok()?;
        let patch = parts.next().unwrap_or(Ok(0)).ok()?;
        Some(Version {
            major,
            minor,
            patch,
        })
    }
}

impl From<[u32; 3]> for Version {
    fn from(v: [u32; 3]) -> Self {
        Self {
            major: v[0],
            minor: v[1],
            patch: v[2],
        }
    }
}

impl From<Version> for [u32; 3] {
    fn from(v: Version) -> Self {
        [v.major, v.minor, v.patch]
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[test]
fn test_version_parse() {
    assert_eq!(Version::parse("1.2.3"), Some([1, 2, 3].into()));
    assert_eq!(Version::parse("1.2.3.4"), Some([1, 2, 3].into()));
    assert_eq!(Version::parse("100"), Some([100, 0, 0].into()));
    assert_eq!(Version::parse(""), None);
    assert_eq!(Version::parse("v1.2.3"), Some([1, 2, 3].into()));
    assert_eq!(Version::parse("v1"), Some([1, 0, 0].into()));
    assert_eq!(Version::parse("v.1"), None);
    assert_eq!(Version::parse("v"), None);
    assert_eq!(Version::parse("a.b.c"), None);
}
