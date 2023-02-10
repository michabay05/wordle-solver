use crate::word::{Word, WordState};
use crate::WORD_LIST_FILE;

use std::io;

pub struct Wordle {
    // Stores the possible words from word list file
    words: Vec<Word>,
}

impl Wordle {
    // Constructor
    pub fn new() -> Self {
        let mut instance = Self { words: Vec::new() };
        instance.init();
        instance
    }

    fn init(&mut self) {
        // Read from word list file
        let words_list = Self::read_words(WORD_LIST_FILE);
        if words_list.is_err() {
            panic!("Couldn't read from {}", WORD_LIST_FILE);
        }
        let words_list = words_list.unwrap();

        // Split the word list by new line
        // Loop over every word in list
        for el in words_list.split("\n") {
            // Add word into list as 'Word' instance
            self.words.push(Word::new(el.trim(), "xxxxx"));
        }
    }

    // Processes a given input using a 4-stage filtering process
    pub fn process(&mut self, target: &Word) {
        // First, selects all words with green states from word list
        // If the input was "raise xxxyg", then "yitie" would have a state of "xxxxg"
        // so it would selected by the green filter method
        self.filter_greens(target);

        // Next, labels all green filtered word's letter as yellow as needed
        // After the second filter, "yitie" would have have an update state of "xyxyg"
        self.filter_yellows(target);

        // Then, unselects all green filtered word's without any yellow markings
        self.green_yellow_screener(target);

        // Lastly, corrects yellow label in case of a repeating yellow marking
        // Before this filter, "yitie" would have a state of "xyxyg", which is not correct
        // After this filter, "yitie" would have a state of "xyxxg", because there's only one 'i'
        // in the word "raise" with labeled yellow
        self.filter_grays(target);
    }

    // Resets the selection for all the 'Word' instance inside the word list
    pub fn reset_selection(&mut self) {
        for el in &mut self.words {
            el.selected = false;
        }
    }

    pub fn test(&mut self) {
        self.print_test();
    }

    // SIMPLE TEST METHOD
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

    // Marks and selects words from list based on given 'Word' instance
    fn filter_greens(&mut self, target: &Word) {
        // Loop over all 5 indices of the target word
        for i in 0..5 {
            // If the current letter's state isn't green, don't proceed
            if target.state[i] != WordState::Green {
                continue;
            }
            // For every letter with a green state from the target word
            // loop over the entire word list
            for el in &mut self.words {
                // If the current character of current word matches the current letter of
                // the target word, then set the element's state to green and select it
                if el.word[i] == target.word[i] {
                    el.state[i] = WordState::Green;
                    el.selected = true;
                }
            }
        }
    }

    fn filter_yellows(&mut self, target: &Word) {
        // Loop over all 5 indices of the target word
        for i in 0..5 {
            // If the current letter's state isn't yellow, don't proceed
            if target.state[i] != WordState::Yellow {
                continue;
            }
            // For every letter with a green state from the target word
            // loop over the entire word list
            for el in &mut self.words {
                // Loop over every character in the current word from word list
                for j in 0..el.word.len() {
                    // Before proceeding, make sure that the current word isn't selected
                    // and the indices 'i' and 'j' don't match
                    if !el.selected || i == j {
                        continue;
                    }
                    // If the current word's character matches the target's ith letter and the
                    // current word state isn't marked green, mark it yellow
                    if el.word[j] == target.word[i] && el.state[j] != WordState::Green {
                        el.state[j] = WordState::Yellow;
                    }
                }
            }
        }
    }

    fn filter_grays(&mut self, target: &Word) {
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

                // If the current word's yellow count for 'target_char' matches target yellow count
                // for char, skip the current word and don't proceed
                if el_yellow_count == target_yellow_count {
                    continue;
                }

                // The 'count' variable is used as a counter to check when the current word's yellow
                // count is greater than the target yellow count
                let mut count = 0;
                // Loop over every character in the current word
                for j in 0..5 {
                    // If current word's letter matches target's ith letter and current word state is yellow
                    if el.word[j] == target.word[i] && el.state[j] == WordState::Yellow {
                        // If count <= target yellow count, increment count
                        if count <= target_yellow_count {
                            count += 1;
                        }
                        // If count > target yellow count, set every other matching instance to gray
                        if count > target_yellow_count {
                            el.state[j] = WordState::Gray;
                        }
                    }
                }
            }
        }
    }

    // Removes green filtered word without any yellow markings after the yellow filter
    fn green_yellow_screener(&mut self, target: &Word) {
        // Loop over every word in word list
        for el in &mut self.words {
            // If the current word's state array doesn't contain a yellow marking unselect the current word
            if !el.state.contains(&WordState::Yellow) {
                el.selected = false;
            }
        }
    }

    // Util method that read from specified path
    fn read_words(filepath: &str) -> Result<String, io::Error> {
        std::fs::read_to_string(filepath)
    }
}
