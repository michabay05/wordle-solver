use crate::{read_words, WORD_LIST_FILE};

const WORD_LIST_LEN: usize = 12972;

#[derive(Copy, Clone)]
struct FreqChar {
    letter: char,
    green_freq: [u32; 5],
    yellow_freq: u32,
}

impl FreqChar {
    fn new(letter: char) -> Self {
        Self {
            letter,
            green_freq: [0; 5],
            yellow_freq: 0
        }
    }

    fn print(&self) {
        print!("      {}   | ", self.letter);
        for el in &self.green_freq {
            print!(" {:4}  |", el);
        }
        println!("   {:5}   |   {:5}", self.yellow_freq, self.green_sum());
    }


    fn green_sum(&self) -> u32 {
        self.green_freq.iter().sum()
    }
}

pub struct WordFreq {
    words: Vec<String>,
    state: [FreqChar; 26],
    freq: [f32; WORD_LIST_LEN],
}

impl WordFreq {
    pub fn new() -> Self {
        let mut instance = Self {
            words: Vec::new(),
            state: [FreqChar::new('0'); 26],
            freq: [0.0; WORD_LIST_LEN],
        };
        instance.init_word_list();
        instance.init_chars();
        instance
    }

    fn init_chars(&mut self) {
        for i in 0..26 {
            self.state[i].letter = ('a' as u8 + i as u8) as char;
        }
    }

    fn init_word_list(&mut self) {
        let word_list = read_words(WORD_LIST_FILE);
        if word_list.is_err() {
            panic!("Couldn't read from specified file");
        }
        let word_list = word_list.unwrap();
        for word in word_list.split("\n") {
            self.words.push(word.trim().to_string());
        }
    }

    pub fn run(&mut self) {
        self.count_greens();
        self.count_yellows();
        self.print_states();
    }

    pub fn count_greens(&mut self) {
        for i in 0..26 {
            for el in self.words.iter() {
                for c_ind in 0..5 {
                    if el.chars().nth(c_ind).unwrap() == self.state[i].letter {
                        self.state[i].green_freq[c_ind] += 1;
                    }
                }
            }
        }
    }

    pub fn count_yellows(&mut self) {
        for i in 0..26 {
            for el in self.words.iter() {
                if el.contains(self.state[i].letter) {
                    self.state[i].yellow_freq += 1;
                }
            }
        }
    }

    fn count_char(&self, ltr: char, ind: usize) -> u8 {
        let word = &self.words[ind];
        let mut count = 0;
        for i in 0..5 {
            let curr_char = word.chars().nth(i).unwrap();
            if curr_char == ltr {
                count += 1;
            }
        }
        count
    }

    pub fn print_states(&self) {
        println!();
        println!("  Letter  |                  Green                 |   Yellow  |    Total");
        println!("===========================================================================");
        for el in &self.state {
            el.print();
        }
        println!();
    }
}
