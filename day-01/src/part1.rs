pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            // Look for the first digit,
            // from left to right
            let first = iter
                .find(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .expect("No digits found, probably invalid input");
            // Look for the second digit,
            // from right to left
            let second = iter
                .rfind(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                // When there is only one digit,
                // the first and last are the same
                .unwrap_or(first);

            10 * first + second
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet";
        assert_eq!("142", process(input));
    }
}
