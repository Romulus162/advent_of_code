fn main() {
    let input = include_str!("./input.txt");
    let test = include_str!("./test.txt");
    println!("{}", part2(input));
    //println!("{}", part2(test));
}

fn part2(input: &str) -> u64 {
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
            let len = id_to_str.len();

            'pattern_check: for chunk_len in 1..=len / 2 {
                if len % chunk_len != 0 {
                    continue;
                }

                let chunk = &id_to_str[..chunk_len];
                let mut i = chunk_len;

                while i < len {
                    if &id_to_str[i..i + chunk_len] != chunk {
                        continue 'pattern_check;
                    }
                    i += chunk_len;
                }

                total_sum += id;
                break;
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
        let result = part2(test);
        assert_eq!(result, 4174379265);
    }
}
