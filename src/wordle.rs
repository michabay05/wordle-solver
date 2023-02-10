use crate::word::{Word, WordState};
use crate::WORD_LIST_FILE;

use std::io;

pub struct Wordle {
    words: Vec<Word>,
}

// NOTE: remove words like these
//              raise xxyxg
//              zante xxxxg

impl Wordle {
    pub fn new() -> Self {
        let mut instance = Self { words: Vec::new() };
        instance.init();
        instance
    }

    fn init(&mut self) {
        let words_list = Self::read_words(WORD_LIST_FILE);
        if words_list.is_err() {
            panic!("Couldn't read from {}", WORD_LIST_FILE);
        }
        let words_list = words_list.unwrap();
        for el in words_list.split("\n") {
            self.words.push(Word::new(el.trim(), "xxxxx"));
        }

        // NOTE: At the beginning every element is filtered from the possible list of words
    }

    pub fn process(&mut self, target: &Word) {
        self.filter_greens(target);
        self.filter_yellows(target);
        self.green_yellow_screeneer(target);
        self.filter_grays(target);
    }

    pub fn reset_selection(&mut self) {
        for el in &mut self.words {
            el.selected = false;
        }
    }

    pub fn test(&mut self) {
        self.print_test();
        // let word = Word::new("raise", "xxyxg");
        // self.process(&word);
    }

    pub fn print_test(&mut self) {
        let word = Word::new("raise", "xyyxg");
        self.process(&word);
        let mut count = 0;
        for el in &self.words {
            if el.selected {
                el.print();
                count += 1;
            }
        }
        println!("Count = {}", count);
    }

    fn filter_greens(&mut self, target: &Word) {
        for i in 0..5 {
            if target.state[i] != WordState::Green {
                continue;
            }
            for el in &mut self.words {
                if el.word[i] == target.word[i] {
                    el.state[i] = WordState::Green;
                    el.selected = true;
                }
            }
        }
    }

    fn filter_yellows(&mut self, target: &Word) {
        for i in 0..5 {
            if target.state[i] != WordState::Yellow {
                continue;
            }
            for el in &mut self.words {
                for j in 0..el.word.len() {
                    if !el.selected || i == j {
                        continue;
                    }
                    if el.word[j] == target.word[i] && el.state[j] != WordState::Green {
                        el.state[j] = WordState::Yellow;
                        el.selected = true;
                    }
                }
            }
        }
    }

    fn filter_grays(&mut self, target: &Word) {
        let mut count = 0;

        // Loop over every character from the target's word array
        for i in 0..5 {
            // Get current char
            let target_char = target.word[i];
            // Get the number of yellow states for the current char
            let target_yellow_count = target.count_state_for_char(target_char, WordState::Yellow);
            // Loop over every word in word list
            for el in &mut self.words {
                // Get the current element char at index (i)
                let el_char = el.word.iter().position(|el| *el == target_char);
                if el_char.is_none() {
                    continue;
                }
                let el_char = el.word[el_char.unwrap()];
                // Get the number of yellow states for the current char
                let el_yellow_count = el.count_state_for_char(el_char, WordState::Yellow);

                if el_yellow_count == target_yellow_count {
                    continue;
                }

                let mut count = 0;
                for j in 0..5 {
                    if count <= target_yellow_count && el.word[j] == target.word[i] {
                        if el.state[j] == WordState::Yellow {
                            count += 1;
                        }
                    }
                    if count > target_yellow_count && el.word[j] == target.word[i] {
                        el.state[j] = WordState::Gray;
                    }
                }
            }
        }
    }

    fn green_yellow_screeneer(&mut self, target: &Word) {
        for el in &mut self.words {
            for j in 0..el.word.len() {
                if !el.state.contains(&WordState::Yellow) {
                    el.selected = false;
                }
            }
        }
    }

    fn read_words(filepath: &str) -> Result<String, io::Error> {
        std::fs::read_to_string(filepath)
    }
}
