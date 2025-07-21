pub use crate::wordle::*;

pub mod wordle {
    use core::error::Error;
    use anstyle::{Style, AnsiColor};

    pub struct Wordle {
        pub answer: String
    }

    impl Wordle {
        pub fn today() -> Result<Wordle, Box<dyn Error>> {
            use reqwest;
            use datetime::{
                LocalDate,
                convenience::Today,
                DatePiece,
            };

            const NYT_URL: &'static str = "https://www.nytimes.com/svc/wordle/v2";
            let today = LocalDate::today();
            let date = format!("{}-{:02}-{:02}", today.year(), today.month() as u8, today.day());
            let url = format!("{NYT_URL}/{date}.json");
            let res: serde_json::Value = reqwest::blocking::get(url)?
            .json()?;

            let answer = res["solution"]
                .as_str()
                .ok_or("Failed to parse NYT response")?
                .to_string();

            Ok(Wordle {
                answer,
            })
        }

        pub fn len(&self) -> usize {
            self.answer.len()
        }

        pub fn new(answer: &str) -> Result<Wordle, &'static str> {
            if answer.is_empty() {
                return Err("Wordle must not be empty");
            }

            let answer = Wordle::validate_word(answer).ok_or("Invalid characters in wordle")?;

            Ok(Wordle {
                answer,
            })
        }

        pub fn guess<'a>(&'a self, guess: &str)  
            -> Result<WordleResult, &'static str> {
            if guess.len() != self.answer.len() {
               return Err("Guess length does not match");
            }

            let guess = Wordle::validate_word(guess).ok_or("Invalid guess")?;
            let res: Vec<_> = guess
                .char_indices()
                .map(|(i, guess_ch)| {
                    let mut t = LetterKind::None;

                    for (j, ans_ch) in self.answer.char_indices() {
                        if guess_ch == ans_ch {
                            if i == j {
                                t = LetterKind::Exact;
                                break;
                            } else {
                                t = LetterKind::WrongPos;
                            }
                        }
                    };

                    t
                })
                .collect();

            Ok(WordleResult {
                guess,
                res,
            })
        }

        pub fn answer(&self) -> &str {
            &self.answer
        }
        
        fn validate_word(word: &str) -> Option<String> {
            if !word.is_ascii() {
                return None;
            }

            let word = word.to_ascii_lowercase();
            let range = 'a'..='z';

            if word.chars().all(|ch| range.contains(&ch)) {
                Some(word)
            } else {
                None
            }
        }
    }

    pub struct WordleResult {
        pub guess: String,
        pub res: Vec<LetterKind>,
    }

    impl WordleResult {
        pub fn is_correct(&self) -> bool {
            self.res.iter().all(|x| {
                LetterKind::Exact == *x
            })
        }
    }

    #[derive(PartialEq)]
    pub enum LetterKind {
        Exact,
        WrongPos,
        None,
    }

    impl LetterKind {
        pub fn style(&self) -> Style {
            match self {
                Self::Exact => AnsiColor::BrightGreen.on_default(),
                Self::WrongPos => AnsiColor::BrightYellow.on_default(),
                Self::None => AnsiColor::BrightWhite.on_default(),
            }
        }
    }

    pub fn play_today() -> Result<(u8, u8, Wordle), Box<dyn Error>> {
        use std::iter;
        use anstream::println;
        use anstyle::Reset;

        let wordle = Wordle::today()?;
        const MAX_TRIES_NUM: u8 = 6;

        println!("{}\n", iter::repeat_n('_', wordle.len()).collect::<String>());

        for i in 0..MAX_TRIES_NUM {
            println!("({}/{})", i + 1, MAX_TRIES_NUM);
            let res = guess_recursive(&wordle, i == 0)?;

            if res.is_correct() {
                return Ok((i + 1, MAX_TRIES_NUM, wordle));
            }

            let res: String = res.guess
                .chars()
                .zip(res.res.into_iter())
                .map(|(ch, kind)| {
                    format!("{}{}", kind.style(), ch)
                }).collect();

            println!("{res}{Reset}\n");
        }

        Ok((MAX_TRIES_NUM, MAX_TRIES_NUM, wordle))
    }

    fn guess_recursive<'a>(wordle: &'a Wordle, mut hint: bool) 
        -> Result<WordleResult, Box<dyn Error>> {
        use std::io::{self, Write};
        
        let mut input = String::new();
        let mut trimmed_input = "";
        
        while trimmed_input.is_empty() {
            if hint {
                print!("Start typing ");

                hint = false;
            }

            print!("> ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut input)?;
            trimmed_input = input.trim_end();
        }

        wordle.guess(trimmed_input).or_else(|err| {
            println!("{err}. Please try again.");
            guess_recursive(wordle, false)
        })
    }
}

mod test;