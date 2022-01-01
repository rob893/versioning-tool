use clap::{App, Arg};
use git2::Oid;
use git2::{BranchType, Repository};
use serde_json::{from_str, to_string_pretty, to_value, Value as JsonValue};
use std::{fs, str::FromStr};
use version::Version;

mod version;

fn main() {
    let matches = App::new("Versioning Tool")
        .version("1.0")
        .author("Robert Herber <rwherber@gmail.com>")
        .about("Versions various projects")
        .arg(
            Arg::with_name("project-path")
                .short("pp")
                .long("project-path")
                .help("The path to the project file with the version")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("git-path")
                .short("gp")
                .long("git-path")
                .help("The path to the git repo")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let default_project_path = format!("{}/package.json", env!("CARGO_MANIFEST_DIR"));
    let default_git_path = env!("CARGO_MANIFEST_DIR");

    let project_file_path = matches
        .value_of("project-path")
        .unwrap_or(&default_project_path);

    let git_path = matches.value_of("git-path").unwrap_or(default_git_path);
    println!("Project file path: {}", project_file_path);
    println!("Git path: {}", git_path);

    let repo = match Repository::open(git_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open repo: {}", e),
    };

    let mut secs = 0;

    // Get last tag
    let tags = repo.tag_names(Some("*")).unwrap();

    let last_tag = tags.iter().filter_map(|x| x).last();

    if let Some(last_tag_name) = last_tag {
        let mut set_secs = |commit_id: Oid| {
            let commit = repo.find_commit(commit_id).unwrap();
            secs = commit.time().seconds()
        };
        let obj = repo.revparse_single(last_tag_name).unwrap();

        if let Some(tag) = obj.as_tag() {
            set_secs(tag.target_id());
        } else if let Some(tag_commit) = obj.as_commit() {
            set_secs(tag_commit.id());
        } else {
            dbg!(&obj);
        }
    }

    let mut revwalk = repo.revwalk().unwrap();

    revwalk.push_head().unwrap();

    let revwalk = revwalk.filter_map(|id| {
        let id = id.ok()?;
        let commit = repo.find_commit(id).ok()?;

        return Some(commit);
    });

    for commit in revwalk {
        if commit.time().seconds() >= secs {
            println!(
                "On or after last tag: {:?} {}",
                commit.time(),
                commit.message().unwrap()
            );
        } else {
            println!(
                "Before last tag: {:?} {}",
                commit.time(),
                commit.message().unwrap()
            );
        }
    }

    let branches = repo.branches(Some(BranchType::Local)).unwrap();

    for branch in branches {
        match branch {
            Ok((b, t)) => {
                println!("{} {:?}", b.name().unwrap().unwrap(), t);
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }

    let package_lock = load_package_lock(project_file_path);

    let mut data: JsonValue = from_str(&package_lock).unwrap();

    let mut version = Version::from_str(data["version"].as_str().unwrap()).unwrap();
    dbg!(&version);
    version.patch += 1;

    data["version"] = to_value(version.to_string()).unwrap();
    dbg!(&data);
    let json_string = to_string_pretty(&data).unwrap();
    fs::write(project_file_path, json_string).expect("Unable to write file.");
}

fn load_package_lock(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
