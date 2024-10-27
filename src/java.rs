use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process::Command;
use crate::llm_api::LLMApi;

pub struct JavaProject {
    api: LLMApi,
    folder_exists: bool,
    current_folder: String,
}

impl JavaProject {
    pub fn new(api_key: String) -> Self {
        let api = LLMApi::new(api_key);
        Self {
            api,
            folder_exists: false,
            current_folder: "java".to_string(),
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("Expliquer ce que le code Java devrait faire (Tapez 'exit' pour quitter): ");
            io::stdout().flush().unwrap();

            let mut description = String::new();
            io::stdin().read_line(&mut description).expect("Erreur de lecture");
            let description = description.trim();

            if description == "exit" {
                println!("Au revoir !");
                break;
            }

            self.generate_code(description);
        }
    }

    pub fn generate_code(&mut self, description: &str) {
        let prompt = format!("Écris une classe Java qui fait cela : {}.", description);

        match self.api.request(&prompt) {
            Ok(response) => {
                println!("Generated Java code:\n{}", response);

                let extracted_code = self.extract_java_code(&response);
                if extracted_code.is_empty() {
                    println!("Erreur : aucun code Java valide n'a été trouvé.");
                    return;
                }

                let class_name = self.extract_class_name(&extracted_code);
                self.handle_folder(class_name.clone());

                let solution_path = format!("{}/{}.java", self.current_folder, class_name);
                let mut solution_file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&solution_path)
                    .expect("Impossible de créer le fichier .java");

                writeln!(solution_file, "{}", extracted_code)
                    .expect("Erreur lors de l'écriture du fichier .java");

                self.compile_and_run_java(&class_name);
            }
            Err(error) => eprintln!("Erreur lors de la requête : {}", error),
        }
    }

    fn extract_java_code(&self, response: &str) -> String {
        // Extraction du code Java contenu entre les balises ```java et ```
        let parts: Vec<&str> = response.split("```java").collect();
        if parts.len() > 1 {
            if let Some(code_block) = parts.get(1) {
                if let Some(code) = code_block.split("```").next() {
                    return code.trim().to_string();
                }
            }
        }
        String::new()
    }

    fn extract_class_name(&self, code: &str) -> String {
        // Extraction du nom de la classe Java à partir du code généré
        let start = code.find("class ").unwrap() + 6;
        let end = code[start..].find(' ').unwrap() + start;
        code[start..end].trim().to_string()
    }

    fn handle_folder(&mut self, class_name: String) {
        if self.folder_exists {
            println!("Un dossier précédent existe. Voulez-vous le sauvegarder ? (oui/non)");
            let mut save_choice = String::new();
            io::stdin().read_line(&mut save_choice).expect("Erreur de lecture");
            let save_choice = save_choice.trim().to_lowercase();

            if save_choice == "oui" {
                self.current_folder = format!("java_{}", class_name);
                if std::path::Path::new(&self.current_folder).exists() {
                    fs::remove_dir_all(&self.current_folder)
                        .expect("Erreur lors de la suppression du dossier.");
                }
                fs::create_dir(&self.current_folder).expect("Impossible de recréer le dossier java.");
            } else {
                println!("Le code généré sera sauvegardé dans le dossier par défaut 'java'.");
                self.current_folder = "java".to_string();
            }
        }

        if !std::path::Path::new(&self.current_folder).exists() {
            fs::create_dir(&self.current_folder).expect("Impossible de créer le dossier java");
        }

        self.folder_exists = true;
    }

    fn compile_and_run_java(&self, class_name: &str) {
        let solution_path = format!("{}/{}.java", self.current_folder, class_name);

        // Compilation du fichier Java
        let compile_status = Command::new("javac")
            .arg(&solution_path)
            .status()
            .expect("Erreur lors de la compilation du fichier Java.");

        if !compile_status.success() {
            eprintln!("La compilation a échoué.");
            return;
        }

        // Exécution du fichier compilé
        let run_status = Command::new("java")
            .arg("-cp")
            .arg(&self.current_folder)
            .arg(class_name)
            .status()
            .expect("Erreur lors de l'exécution du fichier Java.");

        if run_status.success() {
            println!("Le programme Java a été exécuté avec succès.");
        } else {
            eprintln!("L'exécution du programme a échoué.");
        }
    }
}
