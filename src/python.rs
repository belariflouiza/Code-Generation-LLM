use std::fs::{self, OpenOptions, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use uuid::Uuid;
use serde_json::json;
use crate::llm_api::LLMApi;

pub struct PythonProject {
    api: LLMApi,
    folder_exists: bool,
    current_folder: String,
}

impl PythonProject {
    pub fn new(api_key: String) -> Self {
        let api = LLMApi::new(api_key);
        Self {
            api,
            folder_exists: false,
            current_folder: "python".to_string(),
        }
    }

    pub fn run(&mut self) {
        let python_path = self.get_python_executable_path();

        loop {
            print!("Expliquer ce que le code devrait faire ( Tapez 'exit pour quitter' ): ");
            io::stdout().flush().unwrap();

            let mut description = String::new();
            io::stdin().read_line(&mut description).expect("Erreur de lecture");
            let description = description.trim();

            if description == "exit" {
                println!("Au revoir !");
                break;
            }

            self.generate_code(description, &python_path);
        }
    }

    pub fn get_python_executable_path(&self) -> String {
        print!("Entrez le chemin complet de l'exécutable Python : ");
        io::stdout().flush().unwrap();

        let mut python_path = String::new();
        io::stdin().read_line(&mut python_path).expect("Erreur de lecture");
        python_path.trim().to_string() // Supprime les espaces ou les sauts de ligne
    }

    pub fn generate_code(&mut self, description: &str, python_path: &str) {
        let prompt = format!("Écris une fonction Python qui fait cela : {}.", description);

        match self.api.request(&prompt) {
            Ok(response) => {
                println!("Generated function code:\n{}", response);

                let function_name = self.extract_function_name(&response);
                self.handle_folder(function_name.clone());

                // Générer le fichier solution.py avec le code Python généré
                let solution_path = format!("{}/solution.py", self.current_folder);
                let extracted_code = self.extract_python_code(&response);

                if extracted_code.is_empty() {
                    println!("Erreur : aucun code Python valide n'a été trouvé.");
                    return;
                }

                self.write_to_file(&solution_path, &extracted_code);
                if self.check_python_syntax(&solution_path, python_path) {
                    println!("Code compilation successful!");
                    self.execute_python_script(&solution_path, python_path);
                } else {
                    println!("Compilation failed! Vérifiez la syntaxe du code généré.");
                }

                self.save_conversation(&prompt, &response);
                self.generate_json_file(&prompt, &response, "result.json").expect("Erreur lors de la génération du fichier JSON");

                self.generate_test_file(&function_name);
                if self.run_python_tests(python_path) {
                    println!("All tests passed!");
                } else {
                    println!("Some tests failed. Please review the code and tests.");
                }
            }
            Err(error) => eprintln!("Erreur lors de la requête : {}", error),
        }
    }

    fn handle_folder(&mut self, function_name: String) {
        if self.folder_exists {
            println!("Un dossier précédent existe. Voulez-vous le sauvegarder ? (oui/non)");
            let mut save_choice = String::new();
            io::stdin().read_line(&mut save_choice).expect("Erreur de lecture");
            let save_choice = save_choice.trim().to_lowercase();
    
            if save_choice == "oui" {
                // Créer un nouveau dossier avec le nom de la fonction
                self.current_folder = format!("python_{}", function_name);
                if Path::new(&self.current_folder).exists() {
                    fs::remove_dir_all(&self.current_folder)
                        .expect("Erreur lors de la suppression du dossier.");
                }
                fs::create_dir(&self.current_folder).expect("Impossible de recréer le dossier python.");
            } else {
                // Utiliser le dossier par défaut sans le recréer
                println!("Le code généré sera sauvegardé dans le dossier par défaut 'python'.");
                self.current_folder = "python".to_string();
            }
        }
    
        // Si le dossier n'existe pas encore, on le crée
        if !Path::new(&self.current_folder).exists() {
            fs::create_dir(&self.current_folder).expect("Impossible de créer le dossier python");
        }
    
        self.folder_exists = true;
    }

    fn write_to_file(&self, file_path: &str, content: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Impossible de créer le fichier");

        writeln!(file, "{}", content).expect("Erreur lors de l'écriture du fichier");
    }

    fn extract_python_code(&self, response: &str) -> String {
        let parts: Vec<&str> = response.split("```python").collect();
        if parts.len() > 1 {
            if let Some(code_block) = parts.get(1) {
                if let Some(code) = code_block.split("```").next() {
                    return code.trim().to_string();
                }
            }
        }
        String::new()
    }

    fn extract_function_name(&self, code: &str) -> String {
        let start = code.find("def ").unwrap() + 4;
        let end = code.find('(').unwrap();
        code[start..end].to_string()
    }

    fn check_python_syntax(&self, file_path: &str, python_path: &str) -> bool {
        let output = Command::new(python_path)
            .arg("-m")
            .arg("py_compile")
            .arg(file_path)
            .output()
            .expect("Failed to execute Python syntax check");

        output.status.success()
    }

    fn execute_python_script(&self, file_path: &str, python_path: &str) {
        let output = Command::new(python_path)
            .arg(file_path)
            .output()
            .expect("Failed to execute Python script");

        if output.status.success() {
            println!("Python script executed successfully:");
            io::stdout().write_all(&output.stdout).unwrap();
        } else {
            println!("Python script execution failed:");
            io::stderr().write_all(&output.stderr).unwrap();
        }
    }

    fn save_conversation(&self, prompt: &str, response: &str) {
        let mut results_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("resultats.txt")
            .expect("Impossible d'ouvrir ou de créer resultats.txt");

        writeln!(results_file, "Prompt: {}\nRéponse: {}\n", prompt, response)
            .expect("Erreur lors de l'écriture dans resultats.txt");
    }

    fn generate_json_file(&self, prompt: &str, response: &str, file_path: &str) -> std::io::Result<()> {
        let prompt_id = Uuid::new_v4().to_string();
        let code_python = self.extract_python_code(response);

        let json_data = json!({
            "id": prompt_id,
            "prompt": prompt,
            "code_python": code_python,
        });

        let mut file = File::create(file_path)?;
        file.write_all(json_data.to_string().as_bytes())?;
        println!("Le fichier JSON a été généré : {}", file_path);

        Ok(())
    }

    fn generate_test_file(&self, function_name: &str) {
        let test_code = format!(r#"
from solution import {}

def test_{}():
    assert {}(2, 3) == 5  # Exemple de test

if __name__ == "__main__":
    test_{}()
    print("Test réussi !")
"#, function_name, function_name, function_name, function_name);

        let test_path = format!("{}/test.py", self.current_folder);
        self.write_to_file(&test_path, &test_code);
    }

    fn run_python_tests(&self, python_path: &str) -> bool {
        let test_path = format!("{}/test.py", self.current_folder);
        let output = Command::new(python_path)
            .arg("-m")
            .arg("pytest")
            .arg(test_path)
            .output()
            .expect("Failed to run tests");

        output.status.success()
    }
}
