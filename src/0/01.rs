static INPUT: &str = include_str!("../../inputs/0/01.in");

fn main() {
    part_a();
}

fn part_a() {
    let mut sum: u64 = 0;

    for line in INPUT.lines() {
        let mut digits = (None, None);

        for c in line.chars() {
            match get_digit(c) {
                Some(x) => {
                    digits.0 = Some(x);
                    break;
                }
                None => {}
            }
        }

        for c in line.chars().rev() {
            match get_digit(c) {
                Some(x) => {
                    digits.1 = Some(x);
                    break;
                }
                None => {}
            }
        }

        // println!("{:?}", digits);

        if digits.0.is_some() && digits.1.is_some() {
            sum += digits.1.unwrap() * 10 + digits.0.unwrap();
        }
    }

    println!("Output: {}", sum);
}

fn get_digit(c: char) -> Option<u64> {
    if matches!(c, '0'..='9') {
        u64::from_str_radix(&c.to_string(), 10).ok()
    } else {
        None
    }
}
