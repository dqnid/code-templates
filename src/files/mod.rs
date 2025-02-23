use std::{env, fs, path::PathBuf};

const ENV_KEY: &str = "BLUEPRINTS_PATHS";
const DEFAULT_PATH: &str = "./.blueprints";

// ╭────────────────────────╮
// │- Read from filesystem -│
// ╰────────────────────────╯
pub fn get_template_options(base_paths: Vec<String>) -> Vec<String> {
    let mut type_options: Vec<String> = vec![];

    for base_path in base_paths {
        let type_dirs = get_valid_dirs_paths(&base_path);
        // TODO: prettify this in one line
        for type_dir in type_dirs {
            type_options.push(type_dir);
        }
    }

    type_options
}

pub fn get_sub_dirs_paths(dir: &str) -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();

    let mut valid_dirs: Vec<String> = vec![];

    for path in paths {
        let _path = path.unwrap().path();
        if _path.is_dir() {
            valid_dirs.push(_path.display().to_string());
        }
    }
    return valid_dirs;
}

pub fn get_valid_dirs_paths(dir: &str) -> Vec<String> {
    // FIXME: this panics when the directory does not exist
    let paths = fs::read_dir(dir).unwrap();

    let mut valid_dirs: Vec<String> = vec![];

    for path in paths {
        let _path = path.unwrap().path();
        if is_template_path_valid(&_path) {
            valid_dirs.push(_path.display().to_string());
        }
    }
    return valid_dirs;
}

pub fn is_template_path_valid(path: &PathBuf) -> bool {
    path.is_dir()
}

pub fn get_base_template_paths() -> Vec<String> {
    let mut base_paths: Vec<String> = vec![];

    match env::var(ENV_KEY) {
        Ok(value) => {
            let env_path_list = value.split(";");
            for raw_path in env_path_list {
                base_paths.push(raw_path.to_string());
            }
        }
        Err(_) => {
            base_paths.push(DEFAULT_PATH.to_string());
            println!(
                "${} is not set. Falling back to default './.blueprints' route.",
                ENV_KEY
            );
        }
    }

    base_paths
}

// ╭───────────────────────╮
// │- Write to filesystem -│
// ╰───────────────────────╯
