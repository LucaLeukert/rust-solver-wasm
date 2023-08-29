use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io;
use std::path::Path;

use wasm_bindgen::prelude::*;

fn calculate_similarity(correct_letters: &[Letter], misplaced_letters: &[Letter], candidate: &str) -> usize {
    let mut similarity_score = 0;

    for letter in correct_letters {
        if let Some(pos) = candidate.chars().position(|c| c == letter.letter) {
            similarity_score += (letter.position as isize - pos as isize).abs() as usize;
        }
    }

    for letter in misplaced_letters {
        if let Some(pos) = candidate.chars().position(|c| c == letter.letter) {
            similarity_score += (letter.position as isize - pos as isize).abs() as usize;
        }
    }

    similarity_score
}

fn find_best_match<'a>(correct_letters: &[Letter], misplaced_letters: &[Letter], candidates: &'a [String]) -> Option<&'a String> {
    let mut best_similarity_score = usize::MAX;
    let mut best_match: Option<&String> = None;

    for candidate in candidates {
        let similarity_score = calculate_similarity(correct_letters, misplaced_letters, candidate);
        if similarity_score < best_similarity_score {
            best_similarity_score = similarity_score;
            best_match = Some(candidate);
        }
    }

    best_match
}

fn remove_escape_sequences(input: &str) -> String {
    input
        .replace("\n", "")
        .replace("\t", "")
        .replace("\r", "")
        .replace("\\", "")
}

struct Letter {
    letter: char,
    position: usize,
}

fn solve(words: &mut Vec<String>, bad_chars: String, good_chars: String, misplaced_chars: String) -> String {
    println!("Please enter all the characters that are not included in the answer. (Format: b,w)");
    let bad_chars: Vec<Letter> = remove_escape_sequences(&bad_chars).to_lowercase()
        .split(',')
        .enumerate()
        .map(|(i, c)| Letter { letter: c.chars().nth(0).unwrap(), position: i })
        .collect();

    println!("Please enter all the characters that are in their correct place. (Format: H__L_)");
    let good_chars: Vec<Letter> = remove_escape_sequences(&good_chars).to_lowercase()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != '_')
        .map(|(i, c)| Letter { letter: c, position: i })
        .collect();

    println!("Please enter all the characters that are in the word, but not in their correct place. (Format: _0__A)");
    let misplaced_chars: Vec<Letter> = remove_escape_sequences(&misplaced_chars).to_lowercase()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != '_')
        .map(|(i, c)| Letter { letter: c, position: i })
        .collect();

    let mut indices_to_remove: Vec<usize> = Vec::new();

    'outer: for i in 0..words.len() {
        let word = words[i].clone().to_lowercase();
        for bad_char in &bad_chars {
            let mut word_clone = word.clone();
            for good_char in &good_chars {
                if word_clone.len() > good_char.position {
                    /*println!("removing {}", good_char.position);*/
                    word_clone.remove(good_char.position);
                }
            }

            if word.contains(bad_char.letter) {
                indices_to_remove.push(i);
                continue 'outer;
            }
        }

        for good_char in &good_chars {
            if let Some(char_at_position) = word.chars().nth(good_char.position) {
                if char_at_position != good_char.letter {
                    indices_to_remove.push(i);
                    continue 'outer;
                }
            } else {
                println!("out of bounds");
                indices_to_remove.push(i);
                continue 'outer;
            }
        }

        for misplaced_char in &misplaced_chars {
            if !word.contains(misplaced_char.letter) {
                indices_to_remove.push(i);
                continue 'outer;
            }

            if word.chars().nth(misplaced_char.position).unwrap_or('\0') == misplaced_char.letter {
                indices_to_remove.push(i);
                continue 'outer;
            }
        }
    }

    for &index in indices_to_remove.iter().rev() {
        words.remove(index);
    }

    println!("{}", words.len());

    let best_match = find_best_match(&good_chars, &misplaced_chars, &words).unwrap_or(&String::from("No match found!")).to_string();
    best_match.to_string()
}

#[wasm_bindgen]
pub fn solve_wordle(words: &str, good_chars: &str, bad_chars: &str, misplaced_chars: &str) -> String {
    let mut words = words.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

    format!("{} New-Words-List: {}", solve(&mut words, bad_chars.to_string(), good_chars.to_string(), misplaced_chars.to_string()), words.join("\n")).to_string()
}