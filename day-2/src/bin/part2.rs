use std::collections::HashMap;

fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut cubes_required = HashMap::new();
            line.split(": ")
                .last()
                .unwrap()
                .split("; ")
                .for_each(|turn| {
                    turn.split(", ").for_each(|cube| {
                        let mut info = cube.split(" ");
                        let count: usize = info.next().unwrap().parse().unwrap();
                        let colour = info.next().unwrap();
                        if count > *cubes_required.get(colour).unwrap_or(&0) {
                            cubes_required.insert(colour, count);
                        }
                    });
                });

            let r = cubes_required.get("red").unwrap_or(&0);
            let g = cubes_required.get("green").unwrap_or(&0);
            let b = cubes_required.get("blue").unwrap_or(&0);
            let power = r * g * b;
            return power;
        })
        .sum();

    println!("{}", sum);
}
