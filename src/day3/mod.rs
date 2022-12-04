fn get_common_char(input: (&str, &str)) -> Option<char> {
    let (left, right) = input;
    left.chars().fold(None, |mut acc, left| {
        let common_char = right.chars().fold(None, |mut acc, right| {
            if left == right {
                acc = Some(left)
            }
            acc
        });
        if let Some(character) = common_char {
            acc = Some(character)
        }
        acc
    })
}

pub fn part1(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |mut acc, rucksack| {
            let common_char = get_common_char(rucksack);
            let priority = match common_char {
                Some(priority) => match priority.is_ascii_lowercase() {
                    true => u32::from(priority) - 96,
                    false => u32::from(priority) - 38,
                },
                None => panic!(),
            };
            acc += priority;
            acc
        })
}
