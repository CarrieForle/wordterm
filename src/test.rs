#[cfg(test)]
mod test {
    use crate::wordle::*;

    #[test]
    fn test_correct() {
        let wordle = Wordle::new("hello").unwrap();
        let guess = wordle.guess("hello").unwrap();

        assert!(guess.is_correct());
    }

    #[test]
    fn test1() {
        let wordle = Wordle::new("fancy").unwrap();
        let guess = wordle.guess("fryer").unwrap();

        let expected = [
            LetterType::Exact,
            LetterType::None,
            LetterType::WrongPos,
            LetterType::None,
            LetterType::None,
        ];

        let res = guess.res
            .into_iter()
            .zip(expected.into_iter())
            .all(|(a, b)| a == b);

        assert!(res);
    }

    #[test]
    fn test2() {
        let wordle = Wordle::new("hello").unwrap();
        let guess = wordle.guess("local").unwrap();

        let expected = [
            LetterType::WrongPos,
            LetterType::WrongPos,
            LetterType::None,
            LetterType::None,
            LetterType::WrongPos,
        ];

        let res = guess.res
            .into_iter()
            .zip(expected.into_iter())
            .all(|(a, b)| a == b);

        assert!(res);
    }

    #[test]
    #[should_panic]
    fn test_not_ascii() {
        let _ = Wordle::new("幹").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_not_empty() {
        let _ = Wordle::new("").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_len_not_match() {
        let wordle = Wordle::new("hello").unwrap();
        let _ = wordle.guess("hell").unwrap();
    }
}