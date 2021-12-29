use serde_json::{from_str, Result as JsonResult, Value as JsonValue};
use std::{fs, str::FromStr};

fn main() {
    let package_lock = load_package_lock();

    let data: JsonValue = from_str(&package_lock).unwrap();

    let version = Version::from_str(data["version"].as_str().unwrap()).unwrap();
    dbg!(version);
    // println!("{}", version);
}

fn load_package_lock() -> String {
    let path = format!("{}/package.json", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(path).unwrap()
}

#[derive(Debug)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
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

#[derive(Debug)]
struct VersionError {}
