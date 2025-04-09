use std::cmp::min;

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();

    if a_len == 0 {
        return b_len;
    } else if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 1..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1]
            } else {
                matrix[i][j] = 1 + min(
                    min(
                        matrix[i - 1][j],
                        matrix[i][j - 1]
                    ),
                    matrix[i - 1][j - 1],
                )
            }
        }
    }
    matrix[a_len][b_len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_word() {
        assert_eq!(levenshtein("test", "test"), 0);
    }

    #[test]
    fn test_basic_case() {
        assert_eq!(levenshtein("chat", "chats"), 1);
        assert_eq!(levenshtein("chien", "niche"), 4);
        assert_eq!(levenshtein("bonjour", "bonjout"), 1);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("chat", ""), 4);
        assert_eq!(levenshtein("", "chat"), 4);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(levenshtein("chat", "chats"), 1);
        assert_eq!(levenshtein("chat", "ch"), 2);
        assert_eq!(levenshtein("ch", "chat"), 2);
    }
}