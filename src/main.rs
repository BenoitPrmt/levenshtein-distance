mod levenshtein;
mod spellchecker;

use levenshtein::levenshtein;
use spellchecker::SpellChecker;

fn main() {
    let mot1 = "chat";
    let mot2 = "chats";

    let distance = levenshtein(mot1, mot2);

    println!(
        "Distance de Levenshtein entre \"{}\" et \"{}\" : {}",
        mot1, mot2, distance
    );

    let dictionary = vec![
        "bonjour".to_string(),
        "chat".to_string(),
        "chien".to_string(),
        "maison".to_string(),
        "voiture".to_string(),
    ];

    let spell_checker = SpellChecker::new(dictionary);

    let word = "chzt";

    let suggestions = spell_checker.suggest(word, 2);

    println!("Mot recherché: {}", word);

    if suggestions.is_empty() {
        println!("Aucune suggestion trouvée.");
    } else {
        println!("Suggestions:");
        for suggestion in suggestions {
            println!("- {}", suggestion);
        }
    }
}

