fn get_char(text: &str, i: usize, offset: i32) -> char {
    let diff = i as i32 + offset;
    if diff < 0 {
        return '.';
    } else {
        let char = text.chars().nth(diff as usize).unwrap_or('.');
        if char == '\n' {
            return '.';
        } else {
            return char;
        }
    }
}

fn is_neighbouring_symbol(text: &str, i: usize) -> bool {
    let width = text.lines().nth(0).unwrap().len() as i32 + 1;
    let neighbours = vec![
        get_char(text, i, -width - 1),
        get_char(text, i, -width),
        get_char(text, i, -width + 1),
        get_char(text, i, -1),
        get_char(text, i, 1),
        get_char(text, i, width - 1),
        get_char(text, i, width),
        get_char(text, i, width + 1),
    ];

    neighbours.iter().any(|c| !c.is_digit(10) && *c != '.')
}

fn main() {
    let mut sum = 0;
    let input = include_str!("input.txt");

    let mut num_start = 0;
    let mut is_symbol_adjacent = false;
    input.char_indices().for_each(|(i, c)| {
        if c.is_digit(10) {
            if !is_symbol_adjacent && is_neighbouring_symbol(input, i) {
                is_symbol_adjacent = true;
            }

            let prev_char = get_char(input, i, -1);
            if !prev_char.is_digit(10) {
                num_start = i;
            }
        } else {
            let num = *&input[num_start..i].parse::<usize>().unwrap_or(0);
            if is_symbol_adjacent && num > 0 {
                sum += num;
            }

            is_symbol_adjacent = false;
        }
    });

    println!("{}", sum);
}
