fn get_first_digit(line: &str, reverse: bool) -> String {
    let mut chars: Box<dyn Iterator<Item = char>> = if reverse {
        Box::new(line.chars().rev())
    } else {
        Box::new(line.chars())
    };

    chars.find(|c| c.is_digit(10)).unwrap().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let sum: u32 = input
        .lines()
        .map(|line| {
            String::from(get_first_digit(line, false) + &get_first_digit(line, true))
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("{}", sum);
}
