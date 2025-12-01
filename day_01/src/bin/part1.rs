fn main() {
    let input = include_str!("./input1.txt");
    let test = include_str!("./test.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut pos: u32 = 50;
    let mut counter = 0;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let num: u32 = line[1..].trim().parse().unwrap();

        let step = num % 100;

        pos = match dir {
            'L' => (pos + 100 - step) % 100,
            'R' => (pos + step) % 100,
            _ => panic!("Invalid first Character"),
        };

        // println!("Line: {}, pos = {}", line, pos);
        if pos == 0 {
            counter += 1;
        }
    }

    // println!("Final position = {}", pos);
    // println!("Total zeros: {}", counter);
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = include_str!("./test.txt");
        let result = part1(test);
        assert_eq!(result, 3);
    }
}
