fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

pub fn part1(input: String) -> usize {
    characters_to_proceed(input, 4)
}

pub fn part2(input: String) -> usize {
    characters_to_proceed(input, 14)
}

pub fn characters_to_proceed(input: String, message_length: usize) -> usize {
    let data = input.char_indices().collect::<Vec<(usize, char)>>();
    data.windows(message_length)
        .find_map(|window| {
            let string = window.iter().fold("".to_owned(), |mut acc, next| {
                acc.push(next.1);
                acc
            });
            match unique(&string) {
                None => Some(window[message_length - 1].0 + 1),
                Some(_) => None,
            }
        })
        .unwrap()
}
