use regex::Regex;

pub fn apply_name_template(template: &str, filename: &str) -> String {
    match template {
        "__name__" | "{{name}}" => filename.to_string(),
        "__upperCase_name__" | "{{upperCasename}}" => filename.to_uppercase().to_string(),
        "__lowerCase_name__" | "{{lowerCasename}}" => filename.to_lowercase().to_string(),
        "__camelCase_name__" | "{{camelCasename}}" => parse_camel_case(filename),
        "__pascalCase_name__" | "{{pascalCasename}}" => parse_pascal_case(filename),
        "__snakeCase_name__" | "{{snakeCasename}}" => parse_snake_case(filename),
        "__upperSnakeCase_name__" | "{{upperSnakeCasename}}" => {
            parse_snake_case(filename).to_uppercase()
        }
        "__kebabCase_name__" | "{{kebabCasename}}" => parse_snake_case(filename).replace("_", "-"),
        "__lowerDotCase_name__" | "{{lowerDotCasename}}" => {
            parse_snake_case(filename).replace("_", ".")
        }
        _ => filename.to_string(),
    }
}

pub fn apply_all_templates_to_string(mut input: String, replacement: &str) -> String {
    let get_template_names_regex = Regex::new(r"(\{\{[\s]*(name|upperCase name|lowerCase name|camelCase name|pascalCase name|snakeCase name|upperSnakeCase name|kebabCase name|lowerDotCase name)[\s]*\}\})").unwrap();
    input = get_template_names_regex
        .replace_all(&input, |captured: &regex::Captures| {
            format!(
                "{}",
                apply_name_template(&captured[1].replace(" ", ""), replacement),
            )
        })
        .into_owned();

    input
}

fn parse_camel_case(filename: &str) -> String {
    let first_char_regex = Regex::new(r"^[A-Z]").unwrap();
    let filename = parse_pascal_case(filename);
    let filename = first_char_regex
        .replace_all(&filename, |captured: &regex::Captures| {
            captured[0].to_lowercase()
        })
        .into_owned();
    filename
}

fn parse_pascal_case(filename: &str) -> String {
    let char_after_space_regex = Regex::new(r" ([a-z])").unwrap();
    let first_char_regex = Regex::new(r"^[a-z]").unwrap();

    // Change all separators by " " to facilitate regex parsing
    let filename = filename.replace("-", " ").replace("_", " ");

    let filename = char_after_space_regex
        .replace_all(&filename, |captured: &regex::Captures| {
            format!(" {}", captured[1].to_uppercase())
        })
        .into_owned();
    let filename = first_char_regex
        .replace_all(&filename, |captured: &regex::Captures| {
            captured[0].to_uppercase()
        })
        .into_owned();

    let filename = filename.replace(" ", "");

    filename
}

fn parse_snake_case(filename: &str) -> String {
    let highlight_regex = Regex::new(r"[\s\_\-A-Z]([A-Za-z])").unwrap();

    let splited_filename = filename.split_at(1);
    let rest_filename = splited_filename.1.to_string();
    let rest_filename = highlight_regex
        .replace_all(&rest_filename, |captured: &regex::Captures| {
            let valid_char_regex = Regex::new(r"^[A-Za-z]$").unwrap();

            let discarded = captured[0].to_string().chars().nth(0);
            let mut left_side = "".to_string();

            if let Some(discarded_first) = discarded {
                if valid_char_regex.is_match(&discarded_first.to_string()) {
                    left_side = discarded_first.to_string();
                }
            }
            format!(
                "_{}{}",
                left_side.to_lowercase(),
                captured[1].to_lowercase()
            )
        })
        .into_owned();
    let filename = format!(
        "{}{}",
        splited_filename.0.to_lowercase(),
        rest_filename.to_lowercase()
    );

    filename.to_string()
}
