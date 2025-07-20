use wordterm::play_today;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
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