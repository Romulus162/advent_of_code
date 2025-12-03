fn main() {
    let input = include_str!("./input.txt");
    let test = include_str!("./test.txt");
    println!("{}", part1(input));
    //println!("{}", part1(test));
}

fn part1(input: &str) -> u64 {
    let mut total_sum: u64 = 0;

    for range_input in input.split(',') {
        let trimmed = range_input.trim();
        if trimmed.is_empty() {
            // println!("{trimmed}");
            continue;
        }

        let (start_str, end_str) = trimmed.split_once('-').expect("Range needs a '-'");

        let start: u64 = start_str.parse().expect("Invalid start number");
        let end: u64 = end_str.parse().expect("Invalid start number");

        for id in start..=end {
            let id_to_str = id.to_string();
            let half = id_to_str.len() / 2;
            let (first, second) = id_to_str.split_at(half);
            if first == second {
                total_sum += id;
            }
        }
    }
    return total_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = include_str!("./test.txt");
        let result = part1(test);
        assert_eq!(result, 1227775554);
    }
}
