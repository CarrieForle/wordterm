use wordterm::{play, TWordle, NytWordle, LetterKind};
use std::error::Error;
use anstream::println;
use anstyle::{Reset, AnsiColor};

fn main() -> Result<(), Box<dyn Error>> {
    let wordle = NytWordle::today()?;

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
{white}- {exact}Green{blue} means this letter is correct.
{white}- {wrongpos}Yellow{blue} means this letter is part of the word 
  but in the wrong position.
{white}- {none}Gray{blue} means the letter is not part of the word.

{reset}You are playing on {year}-{month:02}-{day:02}
"#, 
blue = AnsiColor::BrightCyan.on_default(),
white = AnsiColor::BrightWhite.on_default(),
exact = LetterKind::Exact.style(), 
wrongpos = LetterKind::WrongPos.style(), 
none = LetterKind::None.style(),
year = wordle.date().year(),
month = wordle.date().month() as u8,
day = wordle.date().day(),
reset = Reset,
);
    
    let (tries, max_tries) = play(&wordle)?;

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