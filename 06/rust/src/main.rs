fn main() {
    let input = include_str!("../../input.txt");

    let mut state = [0; 9];
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().expect("invalid input"))
        .for_each(|i| {
            state[i] += 1;
        });

    for day in 1..=265 {
        state.rotate_left(1);
        state[6] += state[8];
        if day == 80 || day == 256 {
            println!("day {}: {}", day, state.iter().sum::<u64>());
        }
    }
}
