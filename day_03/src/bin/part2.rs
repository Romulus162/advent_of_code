fn main() {
    let input = include_str!("./input.txt");
    let test = include_str!("./test.txt");

    println!("{}", part2(input));
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .filter_map(
                    |c| c.to_digit(10)
                        .map(|d| d as u64)
                ).collect();

            let total = 12;
            let length_of_digs = digits.len();
            let mut result = Vec::new();
            let mut start = 0;

            for remaining in (0..total).rev() {
                let end = length_of_digs - remaining;

                let mut best_digit = 0;
                let mut best_pos = start;

                for i in start..end {
                    if digits[i] > best_digit {
                        best_digit = digits[i];
                        best_pos = i;
                    }
                }

                result.push(best_digit);
                start = best_pos + 1;
            }
            result.iter().fold(0u64, |acc, d| acc * 10 + d)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = include_str!("./test.txt");
        let result = part2(test);
        assert_eq!(result, 3121910778619);
    }
}
