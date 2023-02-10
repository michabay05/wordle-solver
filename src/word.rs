use colored::Colorize;

pub struct Word {
    pub word: [char; 5],
    pub state: [WordState; 5],
    pub selected: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WordState {
    Gray,
    Yellow,
    Green,
}

impl Word {
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
        for i in 0..state_str.len() {
            match state_str.chars().nth(i).unwrap() {
                'x' => instance.state[i] = WordState::Gray,
                'y' => instance.state[i] = WordState::Yellow,
                'g' => instance.state[i] = WordState::Green,
                _ => {}
            }
        }
    }

    pub fn count_state_for_char(&self, ltr: char, state: WordState) -> i8 {
        let char_index = self.word.iter().position(|el| *el == ltr);
        // if let None = char_index {
        if char_index.is_none() {
            eprintln!("Couldn't find specified character!");
            assert!(false);
        }
        let mut count = 0;
        for i in 0..5 {
            if self.word[i] == ltr && self.state[i] == state {
                count += 1;
            }
        }
        count
    }

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
