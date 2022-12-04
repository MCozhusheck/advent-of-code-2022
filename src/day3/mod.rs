fn get_common_char(input: (&str, &str)) -> Vec<char> {
    let (left, right) = input;
    left.chars().fold(vec![], |mut acc, left| {
        let mut common_chars = right.chars().fold(vec![], |mut acc, right| {
            if left == right && !acc.contains(&left) {
                acc.push(left)
            }
            acc
        });
        if !common_chars.is_empty() && !acc.iter().any(|c| common_chars.contains(c)) {
            acc.append(&mut common_chars);
        }
        acc
    })
}

pub fn part1(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |mut acc, rucksack| {
            let common_chars = get_common_char(rucksack);
            let priority = match common_chars.last() {
                Some(priority) => match priority.is_ascii_lowercase() {
                    true => u32::from(*priority) - 96,
                    false => u32::from(*priority) - 38,
                },
                None => panic!(),
            };
            acc += priority;
            acc
        })
}

pub fn part2(input: String) -> u32 {
    input
        .split("\n")
        .array_chunks()
        .fold(0, |mut acc, rucksacks: [&str; 3]| {
            let badge_char = get_common_char((
                &String::from_iter(get_common_char((rucksacks[0], rucksacks[1]))),
                rucksacks[2],
            ));
            let priority = match badge_char.first() {
                Some(priority) => match priority.is_ascii_lowercase() {
                    true => u32::from(*priority) - 96,
                    false => u32::from(*priority) - 38,
                },
                None => panic!(),
            };
            acc += priority;
            acc
        })
}
