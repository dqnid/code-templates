mod files;
mod parsers;

use std::fs;

use files::*;
use inquire::{Select, Text};
use parsers::apply_name_template;
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
            // TODO: decide if the path should be inserted automatically of with a loop of selections -> Maybe better the loop
            let target_path_result = Text::new("Insert the target path:").prompt();

            if let Ok(target_path) = target_path_result {
                create_template(template_path, target_name, target_path).unwrap();
            }
        }
    }
}

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

        let get_path_names_regex = Regex::new(r"\/(__.*__)").unwrap();
        to_create_path = get_path_names_regex
            .replace_all(&to_create_path, |captured: &regex::Captures| {
                format!("/{}", apply_name_template(&captured[1], &target_name),)
            })
            .into_owned();

        if path.is_dir() {
            // Create path
            create_template(
                path.display().to_string(),
                target_name.clone(),
                to_create_path,
            )
            .unwrap();
        } else {
            // Create file
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
