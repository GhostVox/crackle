use std::collections::HashMap;

/// Takes a vector of words, a hashmap of yellow positions, and a hashmap of excluded characters. It uses the hashmaps yellow positions and excluded characters to filter the words.
pub fn filter_potential_words(
    mut words: Vec<String>,
    yellow_positions: &HashMap<(char, usize), bool>,
    excluded: &HashMap<char, bool>,
    current_word: &str,
    yellow_characters: &HashMap<char, bool>,
) -> Vec<String> {
    words.retain(|word| {
        // remove the last guess from the list of potential words
        if word == current_word {
            return false;
        }

        // Check that all yellow characters are present somewhere in the word
        let word_chars: std::collections::HashSet<char> = word.chars().collect();
        let all_yellows_present = yellow_characters
            .keys()
            .all(|&yellow_char| word_chars.contains(&yellow_char));

        if !all_yellows_present {
            return false;
        }

        // Check that the word does not contain a yellow character in the wrong position and that the word does not contain a excluded character.
        word.char_indices().all(|(i, c)| {
            let excluded_yellow_position = (c, i);
            !excluded.contains_key(&c) && !yellow_positions.contains_key(&excluded_yellow_position)
        })
    });
    words
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_filtering() {
        let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let yellow_positions = HashMap::new();
        let excluded = HashMap::new();
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words.clone(),
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, words);
    }

    #[test]
    fn test_all_words_filtered() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let yellow_positions = HashMap::new();
        let mut excluded = HashMap::new();
        excluded.insert('o', true); // Both words contain 'o'
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_exclude_characters() {
        let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let yellow_positions = HashMap::new();
        let mut excluded = HashMap::new();
        excluded.insert('l', true);
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, vec!["rust".to_string()]);
    }

    #[test]
    fn test_exclude_yellow_positions() {
        let words = vec![
            "hello".to_string(),
            "helps".to_string(),
            "world".to_string(),
        ];
        let mut yellow_positions = HashMap::new();
        yellow_positions.insert(('e', 1), true); // 'e' at position 1
        let excluded = HashMap::new();
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, vec!["world".to_string()]);
    }

    #[test]
    fn test_both_exclusions() {
        let words = vec![
            "hello".to_string(),
            "helps".to_string(),
            "world".to_string(),
            "great".to_string(),
        ];
        let mut yellow_positions = HashMap::new();
        yellow_positions.insert(('e', 1), true); // 'e' at position 1
        let mut excluded = HashMap::new();
        excluded.insert('l', true);
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, vec!["great".to_string()]);
    }

    #[test]
    fn test_multiple_yellow_positions() {
        let words = vec![
            "abcde".to_string(),
            "aecdb".to_string(),
            "fghij".to_string(),
        ];
        let mut yellow_positions = HashMap::new();
        yellow_positions.insert(('a', 0), true); // 'a' at position 0
        yellow_positions.insert(('e', 4), true); // 'e' at position 4
        let excluded = HashMap::new();
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, vec!["fghij".to_string()]);
    }
    #[test]
    fn test_yellow_characters_with_position_exclusion() {
        let words = vec![
            "bread".to_string(),
            "great".to_string(),
            "heart".to_string(),
        ];
        let mut yellow_positions = HashMap::new();
        yellow_positions.insert(('e', 1), true); // 'e' not at position 1
        let excluded = HashMap::new();
        let mut yellow_characters = HashMap::new();
        yellow_characters.insert('e', true); // Must contain 'e'
        yellow_characters.insert('a', true); // Must contain 'a'

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        // "heart" has 'e' at position 1, so it's filtered out
        // "great" and "bread" both have 'e' and 'a', and 'e' is not at position 1
        assert_eq!(result, vec!["bread".to_string(), "great".to_string()]);
    }
    #[test]
    fn test_yellow_characters_requirement() {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "bread".to_string(),
            "great".to_string(),
        ];
        let yellow_positions = HashMap::new();
        let excluded = HashMap::new();
        let mut yellow_characters = HashMap::new();
        yellow_characters.insert('e', true); // Must contain 'e'
        yellow_characters.insert('a', true); // Must contain 'a'

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        // Only "bread" and "great" contain both 'e' and 'a'
        assert_eq!(result, vec!["bread".to_string(), "great".to_string()]);
    }
    #[test]
    fn test_same_character_different_positions() {
        let words = vec!["erase".to_string(), "bread".to_string()];
        let mut yellow_positions = HashMap::new();
        yellow_positions.insert(('e', 0), true); // 'e' at position 0
        let excluded = HashMap::new();
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "doger",
            &yellow_characters,
        );
        // "erase" starts with 'e' at position 0, so it gets filtered out
        // "bread" has 'e' at position 2, so it passes
        assert_eq!(result, vec!["bread".to_string()]);
    }
    #[test]
    fn test_empty_input() {
        let words = vec![];
        let yellow_positions = HashMap::new();
        let excluded = HashMap::new();
        let yellow_characters = HashMap::new();

        let result = filter_potential_words(
            words,
            &yellow_positions,
            &excluded,
            "manor",
            &yellow_characters,
        );
        assert_eq!(result, Vec::<String>::new());
    }
}
