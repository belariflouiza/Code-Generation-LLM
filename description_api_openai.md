
# Description de l'API OpenAI

## 1. Introduction
L'API OpenAI permet aux développeurs d'accéder aux puissants modèles d'IA d'OpenAI tels que GPT-3.5 et GPT-4. Ces modèles peuvent être utilisés pour générer du texte, écrire du code, effectuer des traductions, répondre à des questions, etc. Cette API utilise une approche de paiement à l'usage, facturant en fonction du nombre de **tokens** utilisés par requête.

## 2. Fonctionnement de l'API OpenAI

### 2.1 Requête à l'API
L'API OpenAI accepte des requêtes HTTP de type **POST** envoyées au serveur OpenAI. Chaque requête contient un **prompt** (texte que vous voulez compléter ou pour lequel vous voulez une réponse) et des **paramètres** optionnels tels que le nombre maximal de tokens à générer, la température, etc.

**Exemple de requête en cURL** :
```bash
curl https://api.openai.com/v1/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "gpt-3.5-turbo",
    "prompt": "Explique la physique quantique de manière simple.",
    "max_tokens": 100,
    "temperature": 0.7
  }'
```

### 2.2 Réponse de l'API
L'API renvoie une réponse au format **JSON** contenant le texte généré et des informations sur la requête. Voici un exemple de réponse :

**Exemple de réponse JSON** :
```json
{
  "id": "cmpl-6XBcC...",
  "object": "text_completion",
  "created": 1629986742,
  "model": "gpt-3.5-turbo",
  "choices": [
    {
      "text": "La physique quantique est une branche de la physique qui étudie les phénomènes...",
      "index": 0,
      "logprobs": null,
      "finish_reason": "length"
    }
  ],
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 90,
    "total_tokens": 100
  }
}
```

## 3. Limites de l'API OpenAI

### 3.1 Types de Limites
L'API OpenAI impose des **limites d'utilisation** pour contrôler la charge sur les serveurs et répartir les ressources de manière équitable. Ces limites sont imposées sur :
- Le nombre de **requêtes par minute (RPM)**.
- Le nombre de **tokens par minute (TPM)**.
- Le **nombre total de tokens** utilisables par mois, selon votre plan tarifaire.

### 3.2 Limites typiques par modèle

| **Modèle**              | **Requêtes par minute (RPM)** | **Tokens par minute (TPM)** | **Limite par jour (RPD)** |
|-------------------------|-------------------------------|-----------------------------|---------------------------|
| GPT-3.5 Turbo (gratuit)  | 3                             | 40,000                      | 200                       |
| GPT-3.5 Turbo (payant)   | 60                            | 150,000                     | 6000                      |
| GPT-4 (payant)           | 20                            | 40,000                      | 1000                      |
| Whisper (audio)          | 3                             | -                           | 200                       |

### 3.3 Gestion des limites
- **Requêtes par minute (RPM)** : Nombre maximal de requêtes que vous pouvez envoyer par minute. Si vous dépassez cette limite, l'API retournera une erreur `429 Too Many Requests`.
- **Tokens par minute (TPM)** : Nombre maximal de tokens que l'API peut traiter par minute. Chaque token est une unité de texte, représentant environ 4 caractères.
  
### 3.4 Méthodes de gestion des erreurs de limite
- **Exponential Backoff** : Si vous atteignez une limite de requête ou de tokens, il est recommandé de retenter la requête après un délai croissant. Cela réduit les risques de surcharge.
- **Batch Processing** : Si vous traitez des tâches massives, vous pouvez regrouper plusieurs tâches en une seule requête pour maximiser l'efficacité.

## 4. Exemple d'utilisation en Python

Voici un exemple d'utilisation de l'API OpenAI en Python pour envoyer une requête et récupérer la réponse.

```python
import openai

# Clé API
openai.api_key = "YOUR_API_KEY"

# Envoi de la requête
response = openai.Completion.create(
  model="gpt-3.5-turbo",
  prompt="Explique la physique quantique de manière simple.",
  max_tokens=100,
  temperature=0.7
)

# Afficher la réponse générée
print(response['choices'][0]['text'])
```

## 5. Coût de l'utilisation de l'API

Le coût de l'utilisation de l'API dépend du nombre de **tokens** traités, à la fois pour le prompt envoyé et la réponse générée. Les prix sont les suivants :

| **Modèle**              | **Coût par 1 000 tokens (entrée)** | **Coût par 1 000 tokens (sortie)** |
|-------------------------|-----------------------------------|------------------------------------|
| GPT-3.5 Turbo            | 0,0015 $                         | 0,002 $                            |
| GPT-4 (8k context)       | 0,03 $                           | 0,06 $                            |
| GPT-4 (32k context)      | 0,06 $                           | 0,12 $                            |

Les **tokens** incluent le texte que vous envoyez ainsi que le texte généré par l'IA.

## 6. Conclusion

L'API OpenAI est une solution puissante pour générer du texte, écrire du code, ou accomplir d'autres tâches complexes via l'intelligence artificielle. Cependant, il est important de comprendre et de surveiller les limites de requêtes pour éviter les erreurs de dépassement, et de gérer les coûts en optimisant le nombre de tokens utilisés.

