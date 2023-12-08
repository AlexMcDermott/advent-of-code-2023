fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut info = line.split(": ");
            let game_id: usize = info.next().unwrap().replace("Game ", "").parse().unwrap();
            let is_valid_game: bool = info
                .next()
                .unwrap()
                .split("; ")
                .map(|reveal| {
                    reveal
                        .split(", ")
                        .map(|cube| {
                            let mut info = cube.split(" ");
                            let count: usize = info.next().unwrap().parse().unwrap();
                            let colour = info.next().unwrap();
                            match colour {
                                "red" => count <= 12,
                                "green" => count <= 13,
                                "blue" => count <= 14,
                                _ => false,
                            }
                        })
                        .all(|is_valid| is_valid)
                })
                .all(|is_valid| is_valid);

            match is_valid_game {
                true => game_id,
                false => 0,
            }
        })
        .sum();

    println!("{}", sum);
}
