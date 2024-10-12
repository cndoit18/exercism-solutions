use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort();
    possible_anagrams.iter().filter(|possible_anagram| {
        let mut lowercase = possible_anagram.to_lowercase().chars().collect::<Vec<char>>();
        if String::from_iter(&lowercase) ==  word.to_lowercase(){
            return false;
        }
        lowercase.sort();
        sorted_word == lowercase
    }).copied().collect::<HashSet<&'a str>>()
}
