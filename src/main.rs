use clap::{Arg, Command};
use std::fmt::Display;
use std::io::{self, Write};
use std::str::FromStr;

mod python;
mod java;
mod llm_api;
mod llm_prompt;

#[derive(Debug, Clone)]
enum Lang {
    Python,
    Java,
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::Python => write!(f, "python"),
            Lang::Java => write!(f, "java"),
        }
    }
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "python" => Ok(Lang::Python),
            "java" => Ok(Lang::Java),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}

fn prompt_for_language() -> Lang {
    println!("Veuillez choisir un langage (python/java):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture de l'entrée utilisateur");

    match input.trim().to_lowercase().as_str() {
        "python" => Lang::Python,
        "java" => Lang::Java,
        _ => {
            println!("Langage non pris en charge. Par défaut, Python sera utilisé.");
            Lang::Python
        }
    }
}

fn main() {
    let matches = Command::new("codegen")
        .version("1.0.0")
        .author("Your Name <your.email@example.com>")
        .about("Code generator for Python and Java using LLMs")
        .arg(
            Arg::new("lang")
                .long("lang")
                .value_name("LANG")
                .help("Sets the programming language")
                .value_parser(clap::builder::PossibleValuesParser::new(["python", "java"])),
        )
        .get_matches();

    let lang = match matches.get_one::<String>("lang") {
        Some(lang) => lang.parse::<Lang>().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        }),
        None => prompt_for_language(),
    };

    match lang {
        Lang::Python => {
            let api_key = std::fs::read_to_string("token.txt")
                .expect("Impossible de lire le fichier token.txt. Assurez-vous qu'il existe à la racine.");
            let mut python_project = python::PythonProject::new(api_key.trim().to_string());
            python_project.run();
        }
        Lang::Java => {
            let api_key = std::fs::read_to_string("token.txt")
                .expect("Impossible de lire le fichier token.txt. Assurez-vous qu'il existe à la racine.");
            let mut java_project = java::JavaProject::new(api_key.trim().to_string());
            java_project.run();
        }
    }
}
