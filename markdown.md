# API OpenAI Client

## Bibliothèques utilisées
- **reqwest::blocking::Client** : Permet d'effectuer des requêtes HTTP, utile pour interagir avec l'API OpenAI en envoyant des données et recevant des réponses synchrones.
- **serde::{Deserialize, Serialize}** : Utilisée pour la sérialisation et désérialisation de données JSON, notamment pour les messages et réponses de l'API.

## Structure LLMApi
- La structure `LLMApi` contient la clé API OpenAI pour authentifier les requêtes.
- La méthode `new` crée une nouvelle instance de cette structure.

## Structures principales
- **Message** : Représente les messages envoyés à l'API. Les rôles peuvent être `system`, `user`, ou `assistant`, permettant de structurer les échanges.
- **OpenAIRequest** : Gère la requête API, contenant le modèle, les messages et le nombre maximal de tokens.
- **OpenAIResponse** et **Choice** : Désérialisent la réponse JSON de l'API, permettant d’accéder aux choix de messages générés.

## Gestion des erreurs
- **OpenAIError** : Cette structure extrait et affiche les messages d'erreur retournés par l'API, facilitant la gestion des erreurs et l'information des utilisateurs.

## Fonctionnalité clé (`request`)
- La méthode `request` envoie une requête avec un prompt utilisateur à l'API OpenAI.
- Elle crée et envoie un corps de requête structuré avec les messages.
- Si une erreur survient, elle est capturée et affichée. En cas de succès, la réponse est désérialisée et le message généré par l'API est renvoyé à l'utilisateur.

## Installation et Utilisation
1. Clonez le projet et ajoutez vos dépendances dans `Cargo.toml` (`reqwest` et `serde`).
2. Créez une instance de `LLMApi` avec votre clé API OpenAI.
3. Utilisez la méthode `request` pour interagir avec l'API en passant un prompt.

## Exemple
```rust
let api = LLMApi::new("votre_cle_api".to_string());
match api.request("Dites-moi une blague") {
    Ok(response) => println!("Réponse de l'API: {}", response),
    Err(e) => println!("Erreur: {}", e),
}

# Gestionnaire de Templates pour Prompts

Ce projet fournit une structure `Prompt` pour gérer des templates de prompts depuis un fichier et créer des prompts en remplaçant des placeholders par des valeurs spécifiques.

## Bibliothèques utilisées

- **std::collections::HashMap** : Utilisé pour stocker les templates de prompts avec des noms uniques comme clés.

## Fonctionnalités

### `Prompt::new(file_name: &str) -> Prompt`

- Cette fonction crée une nouvelle instance de `Prompt` en chargeant des templates à partir d'un fichier.
- Les templates sont définis dans le fichier texte avec la syntaxe `[[[nom]]]` pour nommer chaque prompt.
- Le contenu entre les noms de prompts est chargé dans un `HashMap`.

### `Prompt::create(&self, key: &str, params: &Vec<String>) -> String`

- Permet de créer un prompt en remplaçant des placeholders (`{{0}}`, `{{1}}`, etc.) par les paramètres donnés dans `params`.
- Cette méthode récupère le template correspondant au nom (`key`) et remplace chaque placeholder par les valeurs respectives fournies dans la liste.

# Générateur de Projet Python

Cet outil génère des fonctions Python à partir de descriptions textuelles, crée les fichiers associés, vérifie la syntaxe, et exécute des tests unitaires.

## Bibliothèques utilisées

- **std::fs** : Gestion des fichiers et répertoires.
- **std::io** : Entrée/sortie pour lire les descriptions et écrire des fichiers.
- **uuid::Uuid** : Génération d'ID unique pour les conversations.
- **serde_json::json** : Génération de fichiers JSON pour sauvegarder les résultats.

## Fonctionnalités principales

### `PythonProject::new(api_key: String) -> Self`
- Initialise une nouvelle instance avec l'API OpenAI et gère les répertoires pour les fichiers Python générés.

### `PythonProject::run()`
- Demande une description de la fonction Python à générer, puis appelle `generate_code` pour créer le fichier Python et tester le code.

### `PythonProject::generate_code(description: &str, python_path: &str)`
- Génère du code Python à partir de la description, vérifie la syntaxe et crée un fichier de tests.

### `PythonProject::handle_folder(function_name: String)`
- Gère la création et la suppression des répertoires pour les fichiers générés.

### `PythonProject::extract_python_code(response: &str) -> String`
- Extrait le code Python d'une réponse OpenAI.

### `PythonProject::check_python_syntax(file_path: &str, python_path: &str) -> bool`
- Vérifie la syntaxe du fichier Python généré.

### `PythonProject::execute_python_script(file_path: &str, python_path: &str)`
- Exécute le fichier Python généré et affiche les résultats.

### `PythonProject::save_conversation(prompt: &str, response: &str)`
- Sauvegarde le prompt et la réponse dans un fichier `resultats.txt`.

### `PythonProject::generate_json_file(prompt: &str, response: &str, file_path: &str) -> std::io::Result<()>`
- Génère un fichier JSON contenant le code généré et les informations associées.

### `PythonProject::generate_test_file(function_name: &str)`
- Génère un fichier de tests Python pour la fonction créée.

### `PythonProject::run_python_tests(python_path: &str) -> bool`
- Exécute les tests unitaires Python pour la fonction générée.

## Installation et Utilisation

1. Clonez ce projet.
2. Ajoutez les dépendances nécessaires.
3. Lancez le programme avec un chemin Python valide et une clé OpenAI API.
4. Décrivez la fonction à générer, et les fichiers seront créés et testés automatiquement.




