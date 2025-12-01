use core::panic;

fn main() {
    let input = include_str!("./input1.txt");
    let test = include_str!("./test.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> u32 {
    let mut pos: u32 = 50;
    let mut counter = 0;

    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let steps: u32 = line[1..].trim().parse().unwrap();

        let zeros = match dir {
            'R' => zeros_right(pos, steps),
            'L' => zeros_left(pos, steps),
            _ => panic!("Invalid first character"),
        };
        counter += zeros;

        let step_mod = steps % 100;
        // println!("Line: {}, pos = {}", line, pos);
        pos = match dir {
            'R' => (pos + step_mod) % 100,
            'L' => (pos + 100 - step_mod) % 100,
            _ => unreachable!(),
        };
    }

    // println!("Final position = {}", pos);
    // println!("Total zeros: {}", counter);
    return counter;
}

fn zeros_right(pos: u32, steps: u32) -> u32 {
    let laps = steps / 100;
    let rem = steps % 100;
    laps + ((pos + rem) / 100)
}

fn zeros_left(pos: u32, steps: u32) -> u32 {
    let laps = steps / 100;
    let rem = steps % 100;

    let extra = if pos > 0 && rem >= pos { 1 } else { 0 };
    laps + extra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = include_str!("./test.txt");
        let result = part2(test);
        assert_eq!(result, 6);
    }
}

