fn char_to_digit(ch: char) -> i32 {
    match ch {
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
        _ => panic!("case not covered"),
    }
}

fn parse_num(s: &str, radix: i32) -> i32 {
    let mut n = s.len() as u32;
    let mut num = 0;
    for ch in s.as_bytes() {
        n -= 1;
        let ch = *ch as char;

        let digit = char_to_digit(ch);

        assert!(digit < radix, "invalid digit");

        num += digit * radix.pow(n);
    }

    num
}

fn parse_decimal(s: &str, radix: i32) -> f64 {
    let mut n = 1;
    let mut float = 0.0;
    for ch in s.as_bytes() {
        let ch = *ch as char;

        let digit = char_to_digit(ch);

        assert!(digit < radix, "invalid digit");

        float += f64::from(digit) / f64::from(radix).powi(n);

        n += 1;
    }

    float
}

/// # Panics
/// Does not panic
pub fn str_to_i32(s: &str, radix: i32) -> i32 {
    let mut parts = s.split('.');

    let mut num = parse_num(parts.next().unwrap(), radix);

    if let Some(decimal) = parts.next() {
        num += parse_decimal(decimal, radix).round() as i32;
    }

    num
}

#[test]
fn test_str_to_i32() {
    assert_eq!(str_to_i32("13", 10), 13);
    assert_eq!(str_to_i32("33", 8), 27);
    assert_eq!(str_to_i32("ff", 16), 255);
    assert_eq!(str_to_i32("ff.ff", 16), 256);
}
