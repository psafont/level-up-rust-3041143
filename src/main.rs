mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut last_n = 1;
        let mut chars = text.chars();
        let mut last_maybe = chars.next();
        let mut encoded = String::with_capacity(text.len());

        while let Some(last) = last_maybe {
            let next = chars.next();

            if next == Some(last) && last_n != 9 {
                last_n += 1;
            } else {
                let chunk = format!("{}{}", last_n, last);
                encoded.push_str(&chunk);
                last_n = 1;
            }
            last_maybe = next;
        }
        encoded
    }

    pub fn decode(text: &str) -> String {
        let mut decoded = String::with_capacity(text.len());
        let mut chars = text.chars();

        while let (Some(count), Some(char)) = (chars.next(), chars.next()) {
            let count = count.to_digit(10).unwrap();
            for _ in 0..count {
                decoded.push(char);
            }
        }
        decoded
    }
}

fn main() {
    //
}

#[test]
fn test_empty() {
    use run_length_encoding::*;

    assert_eq!(encode(""), "");
}

#[test]
fn test_abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn test_round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn test_long() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}

#[test]
fn test_longnumbers() {
    use run_length_encoding::*;

    let input = "1223334444 55555666666777777788888888 9999999990000000000";
    assert_eq!(encode(input), "112233441 556677881 999010");
}
