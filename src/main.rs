mod levenshtein;
use levenshtein::levenshtein;

fn main() {
    let mot1 = "chat";
    let mot2 = "chats";

    let distance = levenshtein(mot1, mot2);

    println!(
        "Distance de Levenshtein entre \"{}\" et \"{}\" : {}",
        mot1, mot2, distance
    );
}

