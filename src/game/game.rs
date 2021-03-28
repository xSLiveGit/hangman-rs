use std::{io::Read, process::Command};

pub struct HangmaneGame {
    secret_line: String,
    discovered_letters: Vec<char>,
    lives: u32,
    tested_letters: Vec<char>,
    printed_mask: String,
}


impl HangmaneGame {
    pub fn new(secret_line: String) -> HangmaneGame {
        let s_len = secret_line.len();
        HangmaneGame {
            secret_line,
            discovered_letters: Vec::new(),
            lives: 6,
            tested_letters: Vec::new(),
            printed_mask: (0..s_len).map(|_| "_").collect::<String>(),
        }
    }

    pub fn play(&mut self) {
        self.update_screen();

        while self.is_running() {
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
            let mut byte = input.bytes().nth(0).expect("no byte read") as char;
            
            if self.is_input_valid(&byte){
                byte = byte.to_ascii_lowercase();
                self.press_key(byte);                
            }
        }

        match self.lives {
            0 => {
                println!("GAME OVER");
                println!("The word was: {}", self.secret_line);
            }

            _ => println!("Congratulations"),
        }
    }

    fn is_input_valid(&mut self, c: &char) -> bool {
        (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z')
    }

    fn is_running(&self) -> bool {
        self.lives > 0 && self.printed_mask.contains('_')
    }

    fn press_key(&mut self, key: char) {
        self.on_key(key);
        self.update_screen();
    }

    fn on_key(&mut self, key: char) {
        if self.tested_letters.contains(&key) {
            return;
        }

        if self.secret_line.contains(key) {
            self.on_match_key(key)
        } else {
            self.on_wrong_key(key)
        }
    }

    fn update_screen(&self) {
        self.clear_terminal();
        println!("Guess the word");
        println!("Word is: {}", &self.printed_mask);
        let word = self.tested_letters.clone().iter().cloned().collect::<String>();
        println!("Tested letters: {}", word);

        self.print_hangman();
    }

    fn print_hangman(&self) {
        match self.lives {
            0 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         O  ");
                println!("|        /|\\ ");
                println!("|         |  ");
                println!("|        / \\ ");
            }
            1 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         O  ");
                println!("|        /|\\ ");
                println!("|         |  ");
                println!("|        /   ");
            }
            2 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         O  ");
                println!("|        /|\\ ");
                println!("|         |  ");
                println!("|            ");
            }
            3 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         O  ");
                println!("|        /|  ");
                println!("|         |  ");
                println!("|            ");
            }
            4 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         O  ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|            ");
            }
            5 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|         O  ");
                println!("|            ");
                println!("|            ");
                println!("|            ");
            },
            6 => {
                println!(" _________   ");
                println!("|         |  ");
                println!("|         |  ");
                println!("|            ");
                println!("|            ");
                println!("|            ");
                println!("|            ");
            }
            _ => panic!(),
        }
    }

    fn clear_terminal(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn on_match_key(&mut self, key: char) {
        self.printed_mask = String::from("");
        self.tested_letters.push(key);
        self.discovered_letters.push(key);

        for c in self.secret_line.chars(){
            if self.tested_letters.contains(&c){
                self.printed_mask.push(c);
            } else {
                self.printed_mask.push('_');
            }
        }
    }

    fn on_wrong_key(&mut self, key: char) {
        self.tested_letters.push(key);
        self.lives -= 1;
    }
}
