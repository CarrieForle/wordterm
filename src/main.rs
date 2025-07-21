use wordterm::{play_today, LetterKind};
use std::error::Error;
use anstream::println;
use anstyle::{Reset, AnsiColor};

fn main() -> Result<(), Box<dyn Error>> {
    println!(
r#"
                        _ _                      
                       | | |                     
 __      _____  _ __ __| | |_ ___ _ __ _ __ ___  
 \ \ /\ / / _ \| '__/ _` | __/ _ \ '__| '_ ` _ \ 
  \ V  V / (_) | | | (_| | ||  __/ |  | | | | | |
   \_/\_/ \___/|_|  \__,_|\__\___|_|  |_| |_| |_|
=================================================
{blue}Wordle starts now! You have 6 tries to get the 
right five-letter word.
For each tries the letters are colored based on 
their occurance as a hint:
- {green}Green{blue} means this letter is correct.
- {yellow}Yellow{blue} means this letter is part of the word 
  but in the wrong position.
- {white}White{blue} means the letter is not part of the word.
{reset}"#, 
blue = AnsiColor::BrightCyan.on_default(),
green = LetterKind::Exact.style(), 
yellow = LetterKind::WrongPos.style(), 
white = LetterKind::None.style(),
reset = Reset, );

    let (tries, max_tries, wordle) = play_today()?;

    match tries {
        1 => {
            println!("Correct. You did it first try. GG.");
        }
        t if t == max_tries => {
            println!("You're out of tries. The answer is {}.", wordle.answer());
        }
        t => {
            println!("Correct. You did it in {t} tries.");
        }
    }

    Ok(())
}