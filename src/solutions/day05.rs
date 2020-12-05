use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part_1(input: &[u8]) -> u16 {
    parse_input(input).max().unwrap()
}

fn parse_input(input: &[u8]) -> impl Iterator<Item = u16> + '_ {
    input.split(|&c| c == b'\n').map(id)
}

fn id(seat: &[u8]) -> u16 {
    let mut id = 0u16;
    for letter in seat.iter().take(7) {
        id <<= 1;
        id += match letter {
            b'F' => 0,
            b'B' => 1,
            _ => unreachable!(),
        };
    }
    for letter in seat.iter().skip(7).take(3) {
        id <<= 1;
        id += match letter {
            b'L' => 0,
            b'R' => 1,
            _ => unreachable!(),
        };
    }
    id
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u8]) -> u16 {
    let mut iter = parse_input(input);

    let first = iter.next().unwrap();
    let mut min = first;
    let mut max = first;
    let mut sum = first as u32;

    for id in iter {
        min = min.min(id);
        max = max.max(id);
        sum += id as u32;
    }

    let min = min as u32;
    let max = max as u32;

    let target = max * (max + 1) / 2 - (min - 1) * min / 2;
    (target - sum) as u16
}
