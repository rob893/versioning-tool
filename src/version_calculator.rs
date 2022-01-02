use super::version_type::VersionType;

pub fn calculate_version_type(commit_messages: &Vec<String>) -> VersionType {
    let mut num_major = 0;
    let mut num_minor = 0;
    let mut num_patch = 0;

    for message in commit_messages {
        if message.starts_with("feat") {
            num_minor += 1;
        } else {
            num_patch += 1;
        }
    }

    if num_major > 0 {
        return VersionType::Major;
    } else if num_minor > 0 {
        return VersionType::Minor;
    }

    return VersionType::Patch;
}
