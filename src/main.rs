use wordterm::play_today;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
r#"
                        _ _                      
                       | | |                     
 __      _____  _ __ __| | |_ ___ _ __ _ __ ___  
 \ \ /\ / / _ \| '__/ _` | __/ _ \ '__| '_ ` _ \ 
  \ V  V / (_) | | | (_| | ||  __/ |  | | | | | |
   \_/\_/ \___/|_|  \__,_|\__\___|_|  |_| |_| |_|
================================================="#);
    println!("Wordle starts now! You have 6 tries to get the right five-letter word.");

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