mod files;
mod parsers;

use files::*;
use inquire::{Select, Text};

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

    if let Ok(template_type) = template_type_result {
        let template_file_options = get_template_options(vec![template_type]);
        let template_file_result =
            Select::new("Select a template:", template_file_options).prompt();

        if let Ok(template_file) = template_file_result {
            let target_name_result = Text::new("Insert the desired name:").prompt();

            if let Ok(target_name) = target_name_result {
                // TODO: decide if the path should be inserted automatically of with a loop of selections -> Maybe better the loop
                let target_path_result = Text::new("Insert the target path:").prompt();

                if let Ok(target_path) = target_path_result {}
            }
        }
    }
}
