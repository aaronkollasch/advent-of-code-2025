const NUM_COLUMNS: usize = 141;

pub fn get_result(input: &[u8]) -> usize {
    let mut lines = input
        .split(|&b| b == b'\n')
        .filter(|&l| !l.is_empty())
        .step_by(2);
    let first_line = lines.next().unwrap();
    let num_columns = first_line.len();
    let first_pos = num_columns / 2;
    #[cfg(debug_assertions)]
    {
        assert_eq!(
            first_pos,
            first_line.iter().position(|&b| b == b'S').unwrap()
        );
        println!("start pos: {}", first_pos);
    }
    let mut state = [0usize; NUM_COLUMNS];
    let mut new_state = state.clone();
    state[first_pos] = 1;
    for (i, line) in lines.enumerate() {
        #[cfg(debug_assertions)]
        println!(
            "{}",
            state
                .iter()
                .map(|&c| match c {
                    _ if c > 1000000000000 => '█',
                    _ if c > 10000000000 => '▉',
                    _ if c > 1000000000 => '▊',
                    _ if c > 10000000 => '▋',
                    _ if c > 100000 => '▌',
                    _ if c > 10000 => '┃',
                    _ if c > 100 => '┇',
                    _ if c > 10 => '│',
                    _ if c > 0 => '┆',
                    _ => ' ',
                })
                .collect::<String>()
        );
        // #[cfg(debug_assertions)]
        // println!(
        //     "{}",
        //     str::from_utf8(line)
        //         .unwrap()
        //         .replace(".", " ")
        //         .replace("^", "⏶")
        // );
        (first_pos - i..=first_pos + i).for_each(|pos| {
            if state[pos] == 0 {
                return;
            }
            match line[pos] {
                b'^' => {
                    new_state[pos - 1] += state[pos];
                    new_state[pos + 1] += state[pos];
                }
                _ => {
                    new_state[pos] += state[pos];
                }
            };
            state[pos] = 0;
        });
        (state, new_state) = (new_state, state);
    }
    #[cfg(debug_assertions)]
    println!(
        "{}",
        state
            .iter()
            .map(|&c| match c {
                _ if c > 1000000000000 => '█',
                _ if c > 10000000000 => '▉',
                _ if c > 1000000000 => '▊',
                _ if c > 10000000 => '▋',
                _ if c > 100000 => '▌',
                _ if c > 10000 => '┃',
                _ if c > 100 => '┇',
                _ if c > 10 => '│',
                _ if c > 0 => '┆',
                _ => ' ',
            })
            .collect::<String>()
    );
    state.into_iter().sum()
}

pub fn main() {
    print!("{} ", get_result(include_bytes!("../../inputs/day07.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_example_result() {
        let result = get_result(include_bytes!("../../inputs/day07.example.txt"));
        assert_eq!(result, 40);
    }

    #[test]
    fn correct_result() {
        let result = get_result(include_bytes!("../../inputs/day07.txt"));
        assert_eq!(result, 43560947406326);
    }
}
