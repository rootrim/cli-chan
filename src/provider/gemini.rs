use crate::{Message, Part, Role};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const MODEL: &str = "gemini-2.5-flash-lite";

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Message>,
    pub system_instruction: Option<SystemInstruction>,
}

#[derive(Serialize)]
pub struct SystemInstruction {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

impl GeminiResponse {
    pub fn get_text(&self) -> Option<String> {
        let part = self.candidates.first()?.content.parts.first()?;
        match part {
            Part::Text(text) => Some(text.clone()),
            Part::InlineData => None,
        }
    }
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: Message,
}

impl GeminiRequest {
    pub async fn send(&self, client: &Client, api_key: String) -> reqwest::Result<GeminiResponse> {
        let url = format!("{BASE_URL}/{MODEL}:generateContent");
        let response = client
            .post(&url)
            .header("X-goog-api-key", api_key)
            .json(self)
            .send()
            .await?;
        response.json().await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gemini_request_parsing() {
        let gemini_request = GeminiRequest {
        contents: vec![Message {
            role: Role::User,
            parts: vec![Part::Text("Who are you?".into())],
        }],
        system_instruction: Some(SystemInstruction {
            parts: vec![Part::Text(
                "You are Kuro, a jaded terminal spirit who has lived inside command lines for decades.".into(),
            )],
        }),
    };
        let json = serde_json::to_string(&gemini_request).unwrap();
        assert_eq!(
            json,
            r#"{"contents":[{"role":"user","parts":[{"text":"Who are you?"}]}],"system_instruction":{"parts":[{"text":"You are Kuro, a jaded terminal spirit who has lived inside command lines for decades."}]}}"#
        );
    }
}
