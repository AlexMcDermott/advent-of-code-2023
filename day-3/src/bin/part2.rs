use std::collections::HashSet;

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

fn main() {
    let input = include_str!("input.txt");
    let width = input.lines().nth(0).unwrap().len() as i32 + 1;

    let sum = input
        .char_indices()
        .map(|(i, c)| {
            if c == '*' {
                let mut neighbours = HashSet::new();
                for j in -1..2 {
                    for k in -1..2 {
                        let offset = j * width + k;
                        let neighbour = get_char(input, i, offset);
                        if neighbour.is_digit(10) {
                            let num_start_offset = offset
                                - (0..)
                                    .take_while(|&n| get_char(input, i, offset - n).is_digit(10))
                                    .last()
                                    .unwrap_or(0);
                            let num_end_offset = offset
                                + (0..)
                                    .take_while(|&n| get_char(input, i, offset + n).is_digit(10))
                                    .last()
                                    .unwrap_or(0);

                            let num = input[(i as i32 + num_start_offset) as usize
                                ..(i as i32 + num_end_offset + 1) as usize]
                                .parse::<usize>()
                                .unwrap();
                            neighbours.insert(num);
                        }
                    }
                }

                if neighbours.len() == 2 {
                    let values = neighbours.iter().collect::<Vec<_>>();
                    return values[0] * values[1];
                }
            }

            0
        })
        .sum::<usize>();

    println!("{}", sum);
}
