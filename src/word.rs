use colored::Colorize;

pub struct Word {
    // Stores the five letters of a given word in array format
    pub word: [char; 5],
    // Stores the states of each letter in a 5-length array with matching indices
    pub state: [WordState; 5],
    // Stores whether this instance is selected by the 'Wordle' instance
    pub selected: bool,
}

// Enumeration of all the possible states of letter
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WordState {
    Gray,
    Yellow,
    Green,
}

impl Word {
    // Constructor
    pub fn new(new_word: &str, new_state: &str) -> Self {
        let mut instance = Self {
            word: ['0'; 5],
            state: [WordState::Gray; 5],
            selected: false,
        };

        // Initialize word string to char array
        for i in 0..5 {
            instance.word[i] = new_word.chars().nth(i).unwrap();
        }

        // Interpret state from string to WordState array
        Self::interpret_state(&mut instance, new_state);
        instance
    }

    pub fn interpret_state(instance: &mut Self, state_str: &str) {
        // Loop over every letter
        for i in 0..state_str.len() {
            // Match letter to 'WordState' enum
            match state_str.chars().nth(i).unwrap() {
                'x' => instance.state[i] = WordState::Gray,
                'y' => instance.state[i] = WordState::Yellow,
                'g' => instance.state[i] = WordState::Green,
                _ => {}
            }
        }
    }

    pub fn count_state_for_char(&self, ltr: char, state: WordState) -> i8 {
        // Find the given character in the word array
        let char_index = self.word.iter().position(|el| *el == ltr);
        // If it doesn't exists, return 0
        if char_index.is_none() {
            return 0;
        }

        // Stores the total number of matches found
        let mut count = 0;
        // Loop over every letter and check for a match
        for i in 0..5 {
            if self.word[i] == ltr && self.state[i] == state {
                count += 1;
            }
        }
        count
    }

    // Util method that color prints the word using the word's state
    pub fn print(&self) {
        for i in 0..5 {
            let curr_char = self.word[i];
            match self.state[i] {
                WordState::Gray => print!("{}", curr_char.to_string()),
                WordState::Yellow => print!("{}", curr_char.to_string().color("yellow").bold()),
                WordState::Green => print!("{}", curr_char.to_string().color("green").bold()),
            }
        }
        println!();
    }
}
