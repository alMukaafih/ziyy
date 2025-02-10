use crate::ErrorKind;

fn char_to_digit(ch: char) -> Result<u32, ErrorKind> {
    Ok(match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => return Err(ErrorKind::InvalidNumber),
    })
}

fn parse_num(s: &str, radix: u32) -> Result<u32, ErrorKind> {
    let mut n = s.len() as u32;
    let mut num = 0;
    for ch in s.as_bytes() {
        n -= 1;
        let ch = *ch as char;

        let digit = char_to_digit(ch)?;

        assert!(digit < radix, "invalid digit");

        num += digit * radix.pow(n);
    }

    Ok(num)
}

fn parse_decimal(s: &str, radix: u32) -> Result<f64, ErrorKind> {
    let mut n = 1;
    let mut float = 0.0;
    for ch in s.as_bytes() {
        let ch = *ch as char;

        let digit = char_to_digit(ch)?;

        assert!(digit < radix, "invalid digit");

        float += f64::from(digit) / f64::from(radix).powi(n);

        n += 1;
    }

    Ok(float)
}

/// # Panics
/// Does not panic
pub fn str_to_u32(s: &str, radix: u32) -> Result<u32, ErrorKind> {
    let mut parts = s.split('.');

    let mut num = parse_num(parts.next().unwrap(), radix)?;

    if let Some(decimal) = parts.next() {
        num += parse_decimal(decimal, radix)?.round() as u32;
    }

    Ok(num)
}

#[test]
fn test_str_to_u32() {
    assert_eq!(str_to_u32("13", 10), Ok(13));
    assert_eq!(str_to_u32("33", 8), Ok(27));
    assert_eq!(str_to_u32("ff", 16), Ok(255));
    assert_eq!(str_to_u32("ff.ff", 16), Ok(256));
}
