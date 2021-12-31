use clap::{App, Arg, SubCommand};
use git2::{BranchType, Repository};
use serde_json::{from_str, Value as JsonValue};
use std::{fs, str::FromStr};

//fart remove

fn main() {
    let matches = App::new("Versioning Tool")
        .version("1.0")
        .author("Robert Herber <rwherber@gmail.com>")
        .about("Versions various projects")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("The path to the project file with the version")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open repo: {}", e),
    };

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.push_head().unwrap();

    let revwalk = revwalk.filter_map(|id| {
        let id = id.ok()?;
        let commit = repo.find_commit(id).ok()?;

        return Some(commit);
    });

    for commit in revwalk {
        println!("{}", commit.message().unwrap());
    }

    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    for branch in branches {
        match branch {
            Ok((b, t)) => {
                println!("{} {:?}", b.name().unwrap().unwrap(), t);
            }
            Err(e) => {
                continue;
            }
        }
    }

    let default_path = format!("{}/package.json", env!("CARGO_MANIFEST_DIR"));

    let file_path = matches.value_of("path").unwrap_or(&default_path);
    println!("{}", file_path);
    let package_lock = load_package_lock(file_path);

    let data: JsonValue = from_str(&package_lock).unwrap();

    let version = Version::from_str(data["version"].as_str().unwrap()).unwrap();
    dbg!(version);
    // println!("{}", version);
}

fn load_package_lock(path: &str) -> String {
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
