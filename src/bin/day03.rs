use advent2021::common;

const I: u32 = 0;

fn part1(l: &Vec<String>) -> u32 {
    let mid: u32 = (l.len() / 2).try_into().unwrap();
    let s: usize = l[0].len() as usize;

    let gamma = l
        .into_iter()
        .fold(vec![I; s], |mut acc, val: &String| {
            for (i, c) in val.chars().enumerate() {
                if c == '1' {
                    acc[i] += 1;
                }
            }
            acc
        })
        .into_iter()
        .fold(I, |mut gamma, v| {
            if v > mid {
                gamma += 1;
            }
            gamma << 1
        })
        >> 1;

    let mask = (1 << s) - 1;

    gamma * (gamma ^ mask)
}

fn find(l: Vec<u32>, i: usize, least_common: bool) -> Vec<u32> {
    let mid: f32 = *&l.len() as f32 / 2.0;

    let ones = *&l
        .clone()
        .into_iter()
        .fold(I, |x, val: u32| x + ((val >> i) & 1)) as f32;

    let r: Vec<u32> = l
        .into_iter()
        .filter(|x: &u32| {
            if (least_common && ones < mid) || (!least_common && ones >= mid) {
                x >> i & 1 == 1
            } else {
                !(x >> i) & 1 == 1
            }
        })
        .collect();

    if r.len() > 1 {
        return find(r, i - 1, least_common);
    }

    return r;
}

fn part2(l: &Vec<String>) -> u32 {
    let list = l
        .into_iter()
        .map(|x| {
            let mut val: u32 = 0;
            for c in x.chars() {
                if c == '1' {
                    val += 1
                }
                val <<= 1;
            }
            val >>= 1;
            val
        })
        .collect::<Vec<u32>>();

    let s: usize = l[0].len() as usize - 1;
    let o2 = find(list.clone(), s, false)[0];
    let co = find(list, s, true)[0];

    return co * o2;
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>("input/day03.data");

        println!("Part01: {}", part1(&lines));
        println!("Part02: {}", part2(&lines));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>("input/day03.test");
        assert_eq!(part1(&lines), 198);
    }

    #[test]
    fn test_part2() {
        let lines = common::read_input::<String>("input/day03.test");

        assert_eq!(part2(&lines), 230);
    }
}
