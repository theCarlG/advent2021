use advent2021::common;

const DAY: &str = "day06";

fn simulate(input: &[u128], days: i32) -> u128 {
    let mut data = [0_u128; 9];
    for i in input {
        data[*i as usize] += 1;
    }

    for _ in 1..=days {
        let new = data[0];
        for i in 0..=7 {
            data[i] = data[i + 1];
        }
        data[6] += new;
        data[8] = new;
    }

    data.iter().sum()
}

fn main() {
    common::time_func(|| {
        let input: Vec<u128> = common::read_input::<String>(DAY, false)
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        println!("Part 1: {}", simulate(&input, 80));
        println!("Part 2: {}", simulate(&input, 256));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<u128> = common::read_input::<String>(DAY, true)
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        assert_eq!(simulate(&input, 80), 5934);
    }

    #[test]
    fn test_part2() {
        let input: Vec<u128> = common::read_input::<String>(DAY, true)
            .first()
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u128>().unwrap())
            .collect();

        assert_eq!(simulate(&input, 256), 26984457539);
    }
}
