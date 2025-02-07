use chrono::NaiveDate;

enum M {
    Yes(u32),
    Maybe(u32),
    No,
}

fn month_in(field: &str) -> M {
    match field.parse::<u32>() {
        Ok(n) if n <= 12 => M::Maybe(n),
        _ => {
            if field.len() < 3 {
                return M::No;
            }
            match &field.to_ascii_lowercase()[..3] {
                "jan" => M::Yes(1),
                "feb" => M::Yes(2),
                "mar" => M::Yes(3),
                "apr" => M::Yes(4),
                "may" => M::Yes(5),
                "jun" => M::Yes(6),
                "jul" => M::Yes(7),
                "aug" => M::Yes(8),
                "sep" => M::Yes(9),
                "oct" => M::Yes(10),
                "nov" => M::Yes(11),
                "dec" => M::Yes(12),
                _ => M::No,
            }
        }
    }
}

fn day_in(field: &str) -> Option<u32> {
    match field.parse::<u32>() {
        Ok(n) if n <= 31 => Some(n),
        _ => None,
    }
}

fn year_in(field: &str) -> Option<i32> {
    match field.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

enum Format {
    Japanese, // year, month, day
    European, // day, month, year
    American, // month, day, year
}

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    if text.chars().all(|x| !x.is_ascii_digit()) {
        return None;
    }

    let fields: Vec<_> = text.split(['/', '-', '.', ' ']).collect();

    if fields.len() != 3 {
        return None;
    }

    let months = match (month_in(fields[0]), month_in(fields[1])) {
        (M::Maybe(_), M::Yes(month)) | (M::No, M::Yes(month) | M::Maybe(month)) => {
            vec![(month, Format::Japanese), (month, Format::European)]
        }
        (M::Yes(month), M::No | M::Maybe(_)) | (M::Maybe(month), M::No) => {
            vec![(month, Format::American)]
        }
        (M::Maybe(month_a), M::Maybe(month_je)) => vec![
            (month_je, Format::Japanese),
            (month_a, Format::European),
            (month_a, Format::American),
        ],
        (M::No, M::No) | (M::Yes(_), M::Yes(_)) => return None,
    };

    months
        .iter()
        .filter_map(|(month, typ)| match typ {
            Format::Japanese => match (year_in(fields[0]), day_in(fields[2])) {
                (None, _) => None,
                (_, None) => None,
                (Some(y), Some(d)) => Some((y, month, d)),
            },
            Format::European => match (year_in(fields[2]), day_in(fields[0])) {
                (None, _) => None,
                (_, None) => None,
                (Some(y), Some(d)) => Some((y, month, d)),
            },
            Format::American => match (year_in(fields[2]), day_in(fields[1])) {
                (None, _) => None,
                (_, None) => None,
                (Some(y), Some(d)) => Some((y, month, d)),
            },
        })
        .take(1)
        .next()
        .and_then(|(year, month, day)| NaiveDate::from_ymd_opt(year, *month, day))
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Apr.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        NaiveDate::from_ymd_opt(2010, 12, 11)
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        NaiveDate::from_ymd_opt(1999, 3, 2)
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        NaiveDate::from_ymd_opt(2021, 3, 1)
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        NaiveDate::from_ymd_opt(2021, 4, 5)
    )
}

#[test]
fn mdy_dot_2() {
    assert_eq!(
        flexible_date_parse("04.30.2021"),
        NaiveDate::from_ymd_opt(2021, 4, 30)
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
