use crate::ui::bridge::bridge::Recomendation;

#[derive(Default)]
pub struct LanguageToolClient;

impl LanguageToolClient {
    pub fn get_recomendation(input: impl AsRef<str>) -> Vec<Recomendation> {
        let input = input.as_ref();
        let len = input.len();
        let mut i = 0;
        let colors = ["#FF0000", "#00FF00", "#0000FF"];

        let mut results = Vec::new();

        while i < len {
            let char = input.chars().nth(i).unwrap();
            if !char.is_ascii_alphanumeric() {
                i += 1;
                continue;
            }

            let start = i;
            while i < len && !input.chars().nth(i).unwrap().is_ascii_whitespace() {
                i += 1;
            }

            if input[start..i].starts_with('a') || input[start..i].starts_with('A') {
                results.push(Recomendation {
                    start: start as i32,
                    end: i as i32,
                    value: "Bob".to_string(),
                    color: colors[start % 3].into(),
                });
                results.push(Recomendation {
                    start: start as i32,
                    end: i as i32,
                    value: input[start..i].to_string(),
                    color: colors[start % 3].into(),
                });
            }
        }

        results
    }
}
