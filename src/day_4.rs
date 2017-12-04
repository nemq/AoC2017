use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn first_puzzle() -> String
{
    let valid_count = count_valid_passphrases("passphrases.txt", has_repeated_words);
    format!("{}",valid_count)
}

pub fn second_puzzle() -> String
{
    let valid_count = count_valid_passphrases("passphrases.txt", has_anagrams);
    format!("{}", valid_count)
}

pub fn count_valid_passphrases<F: Fn(&str) -> bool>(file: &str, invalid_phrase: F) -> usize
{
    let file = File::open(file).expect("Failed to open passphrases file");
    let reader = BufReader::new(file);
    reader.lines()
          .filter_map(|res| res.ok())
          .filter(|line| !invalid_phrase(&line))
          .count()
}

fn has_repeated_words(phrase: &str) -> bool
{
    let mut dictonary = HashSet::new();
    for word in phrase.split_whitespace()
    {
        if dictonary.contains(word)
        {
            return true
        }
        else
        {
            dictonary.insert(word);
        }
    }

    false
}

fn word_histogram(word: &str) -> HashMap<char, u32>
{
    let mut hist = HashMap::new();
    for ch in word.chars()
    {
        let count = hist.entry(ch).or_insert(0);
        *count += 1;
    }

    hist
}

fn has_anagrams(passphrase: &str) -> bool
{
    let mut hist_vec = Vec::new();
    for word in passphrase.split_whitespace().filter(|w| w.len() > 1)
    {
        let new_hist = word_histogram(word);
        for hist in hist_vec.iter()
        {
            if *hist == new_hist 
            {
                return true
            }
        }

        hist_vec.push(new_hist);
    }

    false
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn first_puzzle()
    {
        assert_eq!(false, has_repeated_words("aa bb cc dd ee"));
        assert_eq!(true, has_repeated_words("aa bb cc dd aa"));
        assert_eq!(false, has_repeated_words("aa bb cc dd aaa"));
    }

    #[test]
    fn second_puzzle()
    {
        assert_eq!(false, has_anagrams("abcde fghij"));
        assert_eq!(true, has_anagrams("abcde xyz ecdab"));
        assert_eq!(false, has_anagrams("a ab abc abd abf abj"));
        assert_eq!(false, has_anagrams("iiii oiii ooii oooi oooo"));
        assert_eq!(true, has_anagrams("oiii ioii iioi iiio"));
    }
}