pub fn process(input: &str) -> String {
    // Sum of all the numbers
    let mut sum = 0;
    // Process each line
    for line in input.lines() {
        // Digits found in the line
        let mut digits = vec![];
        for i in 0..line.len() {
            // Go through each character in the line
            // and look for digits ar the start
            match &line[i..] {
                line if line.starts_with("one") => {
                    digits.push(1);
                }
                line if line.starts_with("two") => {
                    digits.push(2);
                }
                line if line.starts_with("three") => {
                    digits.push(3);
                }
                line if line.starts_with("four") => {
                    digits.push(4);
                }
                line if line.starts_with("five") => {
                    digits.push(5);
                }
                line if line.starts_with("six") => {
                    digits.push(6);
                }
                line if line.starts_with("seven") => {
                    digits.push(7);
                }
                line if line.starts_with("eight") => {
                    digits.push(8);
                }
                line if line.starts_with("nine") => {
                    digits.push(9);
                }
                // Add the numeric digit or continue
                line => {
                    let digit = line.chars().next().unwrap().to_digit(10);
                    match digit {
                        Some(digit) => {
                            digits.push(digit);
                        }
                        None => continue,
                    }
                }
            }
        }
        // When there is only one digit,
        // the first and last are the same
        if digits.len() > 1 {
            sum += digits[0] * 10 + digits.last().unwrap();
        } else {
            sum += digits[0] * 10 + digits[0];
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
