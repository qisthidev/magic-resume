use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessageResponse {
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrammarCheckRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrammarCheckResponse {
    pub corrected_text: String,
    pub suggestions: Vec<GrammarSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrammarSuggestion {
    pub original: String,
    pub suggestion: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolishRequest {
    pub text: String,
    pub style: Option<String>, // "professional", "casual", "academic"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolishResponse {
    pub polished_text: String,
    pub improvements: Vec<String>,
}

pub struct AIService {
    client: Client,
    api_key: Option<String>,
    base_url: String,
}

impl AIService {
    pub fn new(api_key: Option<String>, base_url: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
        }
    }

    pub async fn check_grammar(&self, request: GrammarCheckRequest) -> Result<GrammarCheckResponse> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenAI API key not configured"))?;

        let prompt = format!(
            "Please check the following text for grammar errors and provide corrections. \
             Return the response in JSON format with 'corrected_text' and 'suggestions' fields. \
             Each suggestion should have 'original', 'suggestion', and 'reason' fields.\n\nText: {}",
            request.text
        );

        let openai_request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            max_tokens: 1000,
            temperature: 0.3,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI API request failed: {}", response.status()));
        }

        let openai_response: OpenAIResponse = response.json().await?;
        let content = &openai_response.choices[0].message.content;

        // Try to parse as JSON, fallback to simple response
        if let Ok(parsed_response) = serde_json::from_str::<GrammarCheckResponse>(content) {
            Ok(parsed_response)
        } else {
            // Fallback: assume the entire response is the corrected text
            Ok(GrammarCheckResponse {
                corrected_text: content.clone(),
                suggestions: vec![],
            })
        }
    }

    pub async fn polish_text(&self, request: PolishRequest) -> Result<PolishResponse> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("OpenAI API key not configured"))?;

        let style = request.style.unwrap_or_else(|| "professional".to_string());
        let prompt = format!(
            "Please polish and improve the following text to make it more {} and engaging. \
             Return the response in JSON format with 'polished_text' and 'improvements' fields. \
             The improvements field should be an array of strings describing what was improved.\n\nStyle: {}\nText: {}",
            style, style, request.text
        );

        let openai_request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            max_tokens: 1000,
            temperature: 0.7,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI API request failed: {}", response.status()));
        }

        let openai_response: OpenAIResponse = response.json().await?;
        let content = &openai_response.choices[0].message.content;

        // Try to parse as JSON, fallback to simple response
        if let Ok(parsed_response) = serde_json::from_str::<PolishResponse>(content) {
            Ok(parsed_response)
        } else {
            // Fallback: assume the entire response is the polished text
            Ok(PolishResponse {
                polished_text: content.clone(),
                improvements: vec!["Text has been polished and improved".to_string()],
            })
        }
    }
}