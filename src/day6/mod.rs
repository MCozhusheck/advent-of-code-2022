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
    let data = input.char_indices().collect::<Vec<(usize, char)>>();
    data.windows(4)
        .find_map(|window| {
            let string = window.iter().fold("".to_owned(), |mut acc, next| {
                acc.push(next.1);
                acc
            });
            match unique(&string) {
                None => Some(window[3].0 + 1),
                Some(_) => None,
            }
        })
        .unwrap()
}
