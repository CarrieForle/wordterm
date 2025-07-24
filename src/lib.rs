#[cfg(test)]
mod test;
mod word;

pub use wordle::*;

pub mod wordle {
    use core::error::Error;
    use anstyle::{Style, AnsiColor};
    use datetime::{
        LocalDate,
        convenience::Today,
        DatePiece,
    };
    use crate::word::WORDS;

    pub trait TWordle {
        fn answer(&self) -> &str;
        fn len(&self) -> usize;
        
        fn guess(&self, guess: &str) -> Result<WordleResult, &'static str> {
            if guess.len() != self.answer().len() {
                Err("The length is not right.")?;
            }

            if self.answer().len() == 5 {
                if WORDS.iter()
                .chain(&[self.answer()])
                .all(|w| *w != guess) {
                    Err("No such word.")?;
                }
            }

            let guess = validate(guess)?;
            let res: Vec<_> = guess
                .char_indices()
                .map(|(i, guess_ch)| {
                    let mut t = LetterKind::None;

                    for (j, ans_ch) in self.answer().char_indices() {
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
    }

    pub struct Wordle {
        answer: String,
    }

    impl Wordle {
        pub fn new(answer: &str) -> Result<Wordle, &'static str> {
            if answer.is_empty() {
                return Err("Wordle must not be empty.");
            }

            let answer = validate(answer)?;

            Ok(Wordle {
                answer,
            })
        }
    }

    pub struct NytWordle {
        wordle: Wordle,
        date: LocalDate,
    }

    impl NytWordle {
        pub fn today() -> Result<NytWordle, Box<dyn Error>> {
            use reqwest;

            const NYT_URL: &'static str = "https://www.nytimes.com/svc/wordle/v2";
            let today = LocalDate::today();
            let date = format!("{}-{:02}-{:02}", today.year(), today.month() as u8, today.day());
            let url = format!("{NYT_URL}/{date}.json");
            let mut res: serde_json::Value = reqwest::blocking::get(url)?
            .json()?;

            let answer: String = if let serde_json::Value::String(s) = res["solution"].take() {
                Ok(s)
            } else {
                Err("Failed to get solution from NYT. Please report.")
            }?;

            Ok(NytWordle {
                wordle: Wordle { answer },
                date: today,
            })
        }

        pub fn date(&self) -> LocalDate {
            return self.date
        }
    }

    impl TWordle for NytWordle {
        fn len(&self) -> usize {
            self.wordle.len()
        }

        fn answer(&self) -> &str {
            self.wordle.answer()
        }
    }

    impl TWordle for Wordle {
        fn len(&self) -> usize {
            self.answer.len()
        }

        fn answer(&self) -> &str {
            &self.answer
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
                Self::None => AnsiColor::White.on_default(),
            }
        }
    }

    /// Play wordle
    pub fn play(wordle: &impl TWordle) -> Result<(u8, u8), Box<dyn Error>> {
        use std::iter;
        use anstream::println;
        use anstyle::Reset;

        const MAX_TRIES_NUM: u8 = 6;

        println!("{}\n", iter::repeat_n('_', wordle.len()).collect::<String>());

        for i in 0..MAX_TRIES_NUM {
            println!("({}/{})", i + 1, MAX_TRIES_NUM);
            let res = guess_recursive(wordle, i == 0)?;

            let res_str: String = res.guess
                .chars()
                .zip(res.res.iter())
                .map(|(ch, kind)| {
                    format!("{}{}", kind.style(), ch)
                }).collect();

            println!("{res_str}{Reset}\n");

            if res.is_correct() {
                return Ok((i + 1, MAX_TRIES_NUM));
            }
        }

        Ok((MAX_TRIES_NUM, MAX_TRIES_NUM))
    }

    fn guess_recursive(wordle: &impl TWordle, mut hint: bool) 
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
            println!("{err} Please try again.");
            guess_recursive(wordle, false)
        })
    }

    fn validate(word: &str) -> Result<String, &'static str> {
        if !word.is_ascii() {
            Err("Invalid characters.")?;
        }

        let word = word.to_ascii_lowercase();
        let range = 'a'..='z';

        if word.chars().all(|ch| range.contains(&ch)) {
            Ok(word)
        } else {
            Err("Invalid characters.")
        }
    }
}