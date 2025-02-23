mod files;
mod parsers;

use std::{
    fs::{self, File},
    io::Write,
};

use files::*;
use inquire::{Select, Text};
use parsers::{apply_all_templates_to_string, apply_name_template};
use regex::Regex;

#[cfg(test)]
#[path = "./parsers/test/parsers_tests.rs"]
mod test;

fn main() {
    println!(
        "╭──────────────────╮
│- Code Templates -│
╰──────────────────╯"
    );
    let base_paths = get_base_template_paths();
    let template_type_options = get_template_options(base_paths);
    let template_type_result =
        Select::new("Select a template variant:", template_type_options).prompt();

    if let Ok(template_path) = template_type_result {
        let target_name_result = Text::new("Insert the desired name:").prompt();

        if let Ok(target_name) = target_name_result {
            let mut target_path_heap = "./".to_string();
            const CONFIRM_OPTION: &str = "-- Current --";
            const CANCEL_OPTION: &str = "-- Cancel --";

            loop {
                let mut sub_directory_list = get_sub_dirs_paths(&target_path_heap);
                sub_directory_list.insert(0, CONFIRM_OPTION.to_string());
                sub_directory_list.push(CANCEL_OPTION.to_string());

                let target_path_result = Select::new(
                    &format!(
                        "Select a target directory\n Current is: {}",
                        &target_path_heap
                    ),
                    sub_directory_list,
                )
                .prompt();
                if let Ok(target_path) = target_path_result {
                    // TODO: also check if dir has no subdirs
                    if target_path == CANCEL_OPTION {
                        break;
                    }

                    if target_path == CONFIRM_OPTION {
                        create_template(
                            template_path.clone(),
                            target_name.clone(),
                            target_path_heap.clone(),
                        )
                        .unwrap();
                        break;
                    } else {
                        target_path_heap = target_path;
                    }
                } else {
                    // TODO: manage Error
                    break;
                }
            }
        }
    }
}

// TODO: clean up the mess, should encapsulate parsing and regex into specific functions to improve readability
fn create_template(
    template_path: String,
    target_name: String,
    target_path: String,
) -> Result<(), ()> {
    let paths = fs::read_dir(&template_path).unwrap();
    for _path in paths {
        let path = _path.unwrap().path();
        let mut to_create_path = path.display().to_string().clone();

        to_create_path.replace_range(0..template_path.len(), &target_path);

        // TODO: migrate to new "apply all templates to string" strategy (change the function to allow for filename type of expressions)
        let get_path_names_regex = Regex::new(r"\/(__.*__)").unwrap();
        to_create_path = get_path_names_regex
            .replace_all(&to_create_path, |captured: &regex::Captures| {
                format!("/{}", apply_name_template(&captured[1], &target_name),)
            })
            .into_owned();

        if path.is_dir() {
            let dir_create_result = fs::create_dir(&to_create_path);
            match dir_create_result {
                Ok(_) => {
                    create_template(
                        path.display().to_string(),
                        target_name.clone(),
                        to_create_path,
                    )
                    .unwrap();
                }
                Err(_) => return Err(()),
            }
        } else {
            let template_file_content = fs::read_to_string(path).unwrap();
            let target_file_content =
                apply_all_templates_to_string(template_file_content, &target_name);
            let file_result = File::create(to_create_path);
            match file_result {
                Ok(mut file) => {
                    let write_result = file.write_all(target_file_content.as_bytes());
                    if let Err(_) = write_result {
                        return Err(());
                    }
                }
                Err(_) => return Err(()),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::create_template;

    #[test]
    fn test_create_template() {
        create_template(
            "./.blueprints".to_string(),
            "this_is a-TeSt".to_string(),
            "./source/test".to_string(),
        )
        .unwrap();
    }
}
