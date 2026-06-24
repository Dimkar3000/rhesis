use crate::{
    interop::bridge::ffi::{Range, Recommendation},
    languatool::models::LanguageToolDto,
};

#[derive(Default)]
pub struct LanguageToolClient;

impl LanguageToolClient {
    pub async fn get_recommendation(input: impl AsRef<str>) -> Vec<Recommendation> {
        let input = input.as_ref();

        let mut results = Vec::new();

        let client = reqwest::Client::new();

        let form_data = [
            ("text", input),
            ("language", "auto"),
            ("enabledOnly", "false"),
        ];

        let response = client
            .post("http://localhost:2699/v2/check")
            .form(&form_data)
            .send()
            .await;

        if let Ok(response) = response {
            if response.status() == 200 {
                let body = response.json::<LanguageToolDto>().await;

                if let Err(e) = body {
                    dbg!("failed to get response: {:?}", e);
                    return vec![];
                }

                let body = body.unwrap();
                // dbg!(&body);
                results = body
                    .matches
                    .into_iter()
                    .flat_map(|x| {
                        x.replacements
                            .into_iter()
                            .map(move |replacement| Recommendation {
                                color: "#FF0000".to_string(),
                                range: Range {
                                    start: x.offset,
                                    length: x.length,
                                },
                                value: replacement.value.to_string(),
                            })
                    })
                    .collect::<Vec<_>>();
            }
        }

        results
    }
}
