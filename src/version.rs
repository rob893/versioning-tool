use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let Version {
            major,
            minor,
            patch,
        } = self;
        write!(f, "{}.{}.{}", major, minor, patch)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (major, version) = get_next_number(s).ok_or(VersionError {})?;
        let (minor, version) = get_next_number(version).ok_or(VersionError {})?;
        let (patch, _) = get_next_number(version).ok_or(VersionError {})?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

#[derive(Debug)]
pub struct VersionError {}

fn get_next_number(version: &str) -> Option<(u32, &str)> {
    if !version.contains('.') {
        match version.parse::<u32>() {
            Ok(val) => return Some((val, "")),
            Err(_) => return None,
        }
    }

    for (i, c) in version.chars().enumerate() {
        if c == '.' {
            match version[..i].parse::<u32>() {
                Ok(val) => return Some((val, &version[i + 1..])),
                Err(_) => return None,
            }
        }
    }

    return None;
}
