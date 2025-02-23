#[cfg(test)]
pub mod tests {
    use crate::parsers::{apply_all_templates_to_string, apply_name_template};

    pub const FILENAME_EXPRESSIONS: [&str; 9] = [
        "__name__",
        "__upperCase_name__",
        "__lowerCase_name__",
        "__camelCase_name__",
        "__pascalCase_name__",
        "__snakeCase_name__",
        "__upperSnakeCase_name__",
        "__kebabCase_name__",
        "__lowerDotCase_name__",
    ];

    pub const _TEMPLATE_EXPRESSIONS: [&str; 9] = [
        "{{name}}",
        "{{upperCase name}}",
        "{{lowerCase name}}",
        "{{camelCase name}}",
        "{{pascalCase name}}",
        "{{snakeCase name}}",
        "{{upperSnakeCase name}}",
        "{{kebabCase name}}",
        "{{lowerDotCase name}}",
    ];

    #[test]
    fn test_apply_filename_template() {
        const FILENAME: &str = "this_is a-TeSt";

        let expected_filename_output = [
            "this_is a-TeSt",  // normal
            "THIS_IS A-TEST",  // upper
            "this_is a-test",  // lower
            "thisIsATeSt",     // camel
            "ThisIsATeSt",     // Pascal
            "this_is_a_te_st", // snake
            "THIS_IS_A_TE_ST", // snake upper
            "this-is-a-te-st", // kebab
            "this.is.a.te.st", // lower dot
        ];

        for (i, expression) in FILENAME_EXPRESSIONS.into_iter().enumerate() {
            let output = apply_name_template(expression, FILENAME);
            assert_eq!(output, expected_filename_output[i])
        }
    }

    #[test]
    fn test_apply_all_templates_to_string() {
        const NAME: &str = "this_is a-TeSt";
        const INPUT: &str = "{{name}} {{upperCase name}}{{lowerCase name}} {{camelCase name}}{{pascalCase name}} {{snakeCase name}}{{upperSnakeCase name}} {{kebabCase name}}{{lowerDotCase name}}";
        const EXPECTED_OUTPUT: &str = "this_is a-TeSt THIS_IS A-TESTthis_is a-test thisIsATeStThisIsATeSt this_is_a_te_stTHIS_IS_A_TE_ST this-is-a-te-stthis.is.a.te.st";
        let output = apply_all_templates_to_string(INPUT.to_string(), NAME);
        assert_eq!(output, EXPECTED_OUTPUT);
    }
}
