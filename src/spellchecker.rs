use crate::levenshtein::levenshtein;

pub struct SpellChecker {
    dictionary: Vec<String>,
}

impl SpellChecker {
    pub fn new(dictionary: Vec<String>) -> Self {
        SpellChecker { dictionary }
    }

    pub fn suggest(&self, word: &str, max_distance: usize) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for dict_word in &self.dictionary {
            let distance = levenshtein(word, dict_word);
            if distance <= max_distance {
                suggestions.push(dict_word.clone());
            }
        }
        suggestions.sort_by(|a, b| levenshtein(word, a).cmp(&levenshtein(word, b)));
        suggestions
    }
}
