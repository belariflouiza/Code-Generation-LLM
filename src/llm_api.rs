use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Structure de l'API pour OpenAI uniquement
pub struct LLMApi {
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]  // Ajout de #[derive(Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: i32,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

// Structure pour gérer les erreurs de l'API OpenAI
#[derive(Deserialize, Debug)]
struct OpenAIError {
    error: OpenAIErrorMessage,
}

#[derive(Deserialize, Debug)]
struct OpenAIErrorMessage {
    message: String,
}

impl LLMApi {
    pub fn new(api_key: String) -> LLMApi {
        LLMApi { api_key }
    }

    // Fonction pour envoyer une requête à OpenAI et récupérer la réponse
    pub fn request(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::new();

        // Créer une liste de messages en utilisant le rôle 'user'
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }
        ];

        let request_body = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),  // Utilise un modèle de conversation comme gpt-3.5-turbo ou gpt-4
            messages,
            max_tokens: 1000,  // Limite de tokens
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()?;

        let response_text = response.text()?;

        // Vérifie si la réponse contient une erreur
        if let Ok(error_response) = serde_json::from_str::<OpenAIError>(&response_text) {
            eprintln!("Erreur de l'API : {}", error_response.error.message);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_response.error.message,
            )));
        }

        // Désérialise la réponse si pas d'erreur
        let parsed_response: OpenAIResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("Erreur lors de la désérialisation : {}", e))?;

        Ok(parsed_response.choices[0].message.content.clone())
    }
}
