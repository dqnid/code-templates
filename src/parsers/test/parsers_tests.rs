#[cfg(test)]
pub mod tests {
    use crate::parsers::{apply_name_template, expressions::FILENAME_EXPRESSIONS};

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
}
