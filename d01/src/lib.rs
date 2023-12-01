pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut sum_one: u64 = 0;
    let mut sum_two: u64 = 0;

    let mut left_one = None;
    let mut left_two = None;
    let mut right_one = None;
    let mut right_two = None;

    let mut chars = input.chars();

    loop {
        let s = chars.as_str();

        let mut value_one = None;
        let mut value_two = None;
        if s.starts_with('1') {
            value_one = Some(1);
        } else if s.starts_with('2') {
            value_one = Some(2);
        } else if s.starts_with('3') {
            value_one = Some(3);
        } else if s.starts_with('4') {
            value_one = Some(4);
        } else if s.starts_with('5') {
            value_one = Some(5);
        } else if s.starts_with('6') {
            value_one = Some(6);
        } else if s.starts_with('7') {
            value_one = Some(7);
        } else if s.starts_with('8') {
            value_one = Some(8);
        } else if s.starts_with('9') {
            value_one = Some(9);
        } else if s.starts_with("one") {
            value_two = Some(1);
        } else if s.starts_with("two") {
            value_two = Some(2);
        } else if s.starts_with("three") {
            value_two = Some(3);
        } else if s.starts_with("four") {
            value_two = Some(4);
        } else if s.starts_with("five") {
            value_two = Some(5);
        } else if s.starts_with("six") {
            value_two = Some(6);
        } else if s.starts_with("seven") {
            value_two = Some(7);
        } else if s.starts_with("eight") {
            value_two = Some(8);
        } else if s.starts_with("nine") {
            value_two = Some(9);
        }

        if value_two.is_none() {
            value_two = value_one;
        }

        if left_one.is_none() {
            left_one = value_one;
        }

        if left_two.is_none() {
            left_two = value_two;
        }

        if value_one.is_some() {
            right_one = value_one;
        }

        if value_two.is_some() {
            right_two = value_two;
        }

        let c = chars.next();
        if c.is_none() || c.unwrap().is_control() {
            add_line_value(&mut sum_one, left_one, right_one);
            add_line_value(&mut sum_two, left_two, right_two);
            left_one = None;
            left_two = None;
            right_one = None;
            right_two = None;
        }

        if c.is_none() {
            break;
        }
    }

    Ok(format!("01/01 = {}, 01/02 = {}\n", sum_one, sum_two))
}

fn add_line_value(s: &mut u64, left: Option<u64>, right: Option<u64>) {
    let v = line_value(left, right);
    if let Some(inner) = v {
        *s += inner;
    }
}

fn line_value(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (None, _) => None,
        (Some(m), None) => Some(10 * m + m),
        (Some(m), Some(n)) => Some(10 * m + n),
    }
}
