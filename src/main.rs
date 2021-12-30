use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::fs;

fn contains_any_characters(word: &str, characters: Vec<char>) -> bool {
    for character in characters {
        if word
            .to_lowercase()
            .contains(&character.to_lowercase().to_string())
        {
            return true;
        }
    }
    false
}

fn binary_search(word: &str, words: &[String]) -> bool {
    if words.len() <= 20 {
        for word_ in words {
            if word_ == word {
                return true;
            }
        }

        return false;
    }

    let centre_index = (words.len() - 1) / 2;

    if word == words[centre_index] {
        return true;
    }

    match word.cmp(&words[centre_index]) {
        Ordering::Greater => binary_search(word, &words[centre_index..]),
        Ordering::Less => binary_search(word, &words[..centre_index]),
        _ => panic!(),
    }
}

fn all_lengths(anagram: &str, k: &usize) -> Vec<Vec<char>> {
    if *k == 1 {
        return anagram.chars().permutations(*k).unique().collect_vec();
    }

    let mut result: Vec<Vec<char>> = Vec::new();
    result.append(&mut anagram.chars().permutations(*k).unique().collect_vec());
    result.append(&mut all_lengths(anagram, &(k - 1)));

    result
}

fn main() {
    let anagram = "tests";

    println!("Calculating letter permutations...");

    let letters: Vec<Vec<char>> = all_lengths(anagram, &anagram.len());

    println!("{} letter permutations", letters.len());
    println!("Reading dictionary...");

    let words: Vec<String> = fs::read_to_string("words.txt")
        .expect("Couldn't open words.txt. Does it exist?")
        .split('\n')
        .map(String::from)
        .collect();

    println!("{} words in dictionary", words.len());
    println!("Mapping letter permutations to a dictionary...");

    let mut solved: Vec<String> = Vec::new();
    for perm in letters {
        let result = perm.into_iter().collect::<String>();

        if contains_any_characters(&result, vec!['a', 'e', 'i', 'o', 'y'])
            && !solved.iter().any(|x| x == &result)
            && binary_search(&result, &words)
        {
            solved.push(result);
        }
    }

    solved.sort_by_key(|a| Reverse(a.len()));

    for solved_word in solved {
        println!("{}", solved_word);
    }
}
