fn main() {
    let input = include_str!("input.txt").lines();
    let mut card_counts = vec![1; input.clone().count()];

    input.for_each(|line| {
        let mut parts = line.split(": ");
        let card_id_str = parts.next().unwrap().replace("Card", "");
        let card_id = card_id_str.trim().parse::<usize>().unwrap();
        let card_index = card_id - 1;

        let mut details = parts.next().unwrap().split(" | ");
        let winning = details.next().unwrap().split(" ").collect::<Vec<_>>();
        let yours = details.next().unwrap().split(" ");
        let matches = yours
            .filter(|num| winning.contains(num) && *num != "")
            .count();

        let won_card_base_index = card_index + 1;
        for i in won_card_base_index..won_card_base_index + matches {
            card_counts[i] += card_counts[card_index];
        }
    });

    println!("{}", card_counts.iter().sum::<usize>());
}
