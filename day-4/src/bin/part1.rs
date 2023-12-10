fn main() {
    let sum = include_str!("input.txt")
        .lines()
        .map(|line| {
            let parts = line.split(": ");
            let numbers = parts.last().unwrap();
            let mut details = numbers.split(" | ");
            let winning = details.next().unwrap().split(" ").collect::<Vec<_>>();
            let yours = details.next().unwrap().split(" ");
            let matches = yours
                .filter(|num| winning.contains(num) && *num != "")
                .count();
            match matches {
                0 => 0,
                _ => usize::pow(2, matches as u32 - 1),
            }
        })
        .sum::<usize>();

    println!("{}", sum);
}
