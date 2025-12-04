fn main() {
    let input = include_str!("./input.txt");
    let test = include_str!("./test.txt");

    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            let mut best = 0;

            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let val = digits[i] * 10 + digits[j];
                    best = best.max(val);
                }
            }
            best
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = include_str!("./test.txt");
        let result = part1(test);
        assert_eq!(result, 357);
    }
}
