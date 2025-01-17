use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug, PartialEq)]
enum InvalidIsbn {
    InvalidChar(usize, char),
    InvalidLength(usize),
    DoesNotCheck,
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();

        for (i, c) in s.char_indices() {
            if i == 3 || i == 5 || i == 8 || i == 15 {
                if c != '-' {
                    return Err(InvalidIsbn::InvalidChar(i, c));
                } else {
                    continue;
                }
            };
            match c.to_digit(10) {
                Some(n) => digits.push(n as u8),
                None => return Err(InvalidIsbn::InvalidChar(i, c)),
            }
        }

        if digits.len() != 13 {
            return Err(InvalidIsbn::InvalidLength(digits.len()));
        }

        let check_the_digit =
            |digits: &Vec<u8>| -> bool { calculate_check_digit(digits) == digits[12] };

        if !check_the_digit(&digits) {
            return Err(InvalidIsbn::DoesNotCheck);
        }

        Ok(Isbn {
            raw: s.to_string(),
            digits,
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits
        .iter()
        .take(12)
        .enumerate()
        .map(|(i, &n)| (if i % 2 == 0 { n } else { n * 3 }) as u32)
        .sum();
    match sum % 10 {
        0 => 0,
        n => 10 - n as u8,
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({}) is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, expected) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, expected, actual);
        assert_eq!(actual, *expected)
    }
}

#[test]
fn rust_in_action() {
    let expected = Isbn {
        raw: "978-3-16-148410-0".to_string(),
        digits: [9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0, 0].to_vec(),
    };
    let a_book = (expected.raw).parse();
    assert_eq!(a_book, Ok(expected))
}

#[test]
fn can_spot_invalid_check_digits() {
    let expected: Result<Isbn, InvalidIsbn> = Err(InvalidIsbn::DoesNotCheck);
    let not_a_book = "978-3-16-148410-1".parse();
    assert_eq!(not_a_book, expected)
}

#[test]
fn can_spot_invalid_length() {
    let cases: [(&str, Result<Isbn, InvalidIsbn>); 2] = [
        ("978-3-16-148410-10", Err(InvalidIsbn::InvalidLength(14))),
        ("978-3-16-148410-", Err(InvalidIsbn::InvalidLength(12))),
    ];
    for (case, expected) in cases.iter() {
        let not_a_book = case.parse();
        assert_eq!(not_a_book, *expected)
    }
}
