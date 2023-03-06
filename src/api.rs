use crate::{
    constant::OPENAI_ENDPOINT,
    model::{Message, MessageRequestPayload, MessageResponsePayload, MessageRole},
};

pub struct Api {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum ApiError {
    RequestError(reqwest::Error),
    InvalidResponseError,
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::RequestError(value)
    }
}

impl Api {
    #[allow(dead_code)]
    pub fn key_from_input(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn key_from_env() -> Self {
        let api_key = std::env::var("OPENAI_API_KEY")
            .ok()
            .expect("Failed to get OPENAI_API_KEY");

        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    fn create_default_commandline_params(&self, user_input: &str) -> MessageRequestPayload {
        MessageRequestPayload {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                Message {
                    role: MessageRole::System,
                    content: "Your job is give the bash commands I request. You should only output the codeblock without any extra description.".to_string()
                },
                Message {
                    role: MessageRole::User,
                    content: user_input.to_string()
                }
            ],
        }
    }

    fn parse_response_text<'a>(&'a self, text: &'a str) -> Result<String, ApiError> {
        let command: Vec<&str> = text
            .lines()
            .into_iter()
            .skip_while(|line| *line != "```")
            .skip(1)
            .take_while(|line| *line != "```")
            .map(|line| line.trim())
            .collect();

        let command = command
            .into_iter()
            .skip_while(|line| *line == "bash")
            .collect::<Vec<&str>>()
            .join("\\\n");

        if command.is_empty() {
            Err(ApiError::InvalidResponseError)
        } else {
            Ok(command)
        }
    }

    pub async fn get_commandline(&self, description: &str) -> Result<String, ApiError> {
        let payload = self.create_default_commandline_params(description);

        let body: MessageResponsePayload = self
            .client
            .post(OPENAI_ENDPOINT)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        let content = body.choices.into_iter().next();

        if let Some(content) = content {
            let command_line = content
                .message
                .content
                .trim();

            let command_line = self.parse_response_text(command_line)?;

            return Ok(command_line);
        }

        Err(ApiError::InvalidResponseError)
    }
}
