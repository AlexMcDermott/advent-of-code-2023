fn get_first_digit(line: &str, reverse: bool) -> char {
    let mut chars: Box<dyn Iterator<Item = char>> = if reverse {
        Box::new(line.chars().rev())
    } else {
        Box::new(line.chars())
    };

    chars.find(|c| c.is_digit(10)).unwrap()
}

fn parse_digit_strings(line: &str) -> String {
    let mut processed_line = String::new();

    for (i, c) in line.char_indices() {
        if c.is_digit(10) {
            processed_line.push(c);
            continue;
        }

        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        .for_each(|(value, num)| {
            if line[i..].starts_with(num) {
                processed_line.push(char::from_digit((value + 1) as u32, 10).unwrap());
            }
        });
    }

    processed_line
}

fn main() {
    let input = include_str!("input.txt");
    let sum: usize = input
        .lines()
        .map(|line| {
            let processed_line = &parse_digit_strings(line)[..];
            format!(
                "{}{}",
                get_first_digit(processed_line, false),
                get_first_digit(processed_line, true)
            )
            .parse::<usize>()
            .unwrap()
        })
        .sum();

    println!("{}", sum);
}
