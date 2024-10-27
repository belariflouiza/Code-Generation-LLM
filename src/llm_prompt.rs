use std::collections::HashMap;

pub struct Prompt {
    prompts: HashMap<String, String>,
}

impl Prompt {
    // Constructeur pour charger les templates depuis un fichier
    pub fn new(file_name: &str) -> Prompt {
        let mut prompts = Prompt {
            prompts: HashMap::new(),
        };
        let content = std::fs::read_to_string(file_name).unwrap();

        let mut prompt_name = String::new();
        let mut prompt_content = String::new();

        for line in content.lines() {
            if line.starts_with("[[[") {
                if !prompt_name.is_empty() {
                    prompts.prompts.insert(prompt_name.clone(), prompt_content.clone());
                }
                prompt_name = line[3..line.len()-3].to_string();
                prompt_content = String::new();
            } else {
                prompt_content.push_str(line);
                prompt_content.push('\n');
            }
        } 
        if !prompt_name.is_empty() {
            prompts.prompts.insert(prompt_name, prompt_content);
        }

        prompts
    }

    // Crée un prompt en remplaçant les placeholders
    pub fn create(&self, key: &str, params: &Vec<String>) -> String {
        let mut prompt = self.prompts.get(key).unwrap().clone();
        for (i, param) in params.iter().enumerate() {
            let placeholder = format!("{{{{{{{}}}}}}}", i);
            prompt = prompt.replace(&placeholder, param);
        }
        prompt
    }
}
