pub fn part1(input: String) -> String {
    let split: Vec<_> = input.split("\n\n").collect();
    let crates = split.first().unwrap().to_owned();
    let mut crates = crates.split("\n").collect::<Vec<&str>>();
    crates.pop().unwrap();
    let instructions = split.last().unwrap().to_owned();
    let instructions = instructions.split("\n").collect::<Vec<&str>>();

    let mut init_state: Vec<String> = crates.iter().fold(vec![], |mut acc, line| {
        let mut iter = line.chars().array_chunks::<4>();
        let mut crates: Vec<char> = iter.by_ref().map(|a| a[1]).collect();

        let remainder = iter
            .into_remainder()
            .unwrap()
            .find(|c| c.is_ascii_alphanumeric())
            .unwrap_or(' ');
        crates.push(remainder);
        if acc.len() == 0 {
            acc = crates.iter().map(|x| x.to_string()).collect();
        } else {
            acc = crates
                .iter()
                .zip(acc.iter())
                .map(|(new_crate, crates)| format!("{}{}", crates, new_crate).trim().to_string())
                .collect();
        }
        acc
    });
    for insturction in instructions {
        let split = insturction.split(" ").collect::<Vec<&str>>();
        let crates_to_move = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        let from_crate = &init_state[from].clone();
        let crates = from_crate.split_at(crates_to_move);
        init_state.remove(from);
        init_state.insert(from, crates.1.to_owned());

        let to_crate = init_state[to].clone();
        init_state.remove(to);
        init_state.insert(to, format!("{}{}", crates.0, to_crate));
    }

    init_state
        .iter()
        .fold("".to_owned(), |mut acc, next| match next.chars().nth(0) {
            Some(first_crate) => format!("{}{}", acc, first_crate),
            None => acc,
        })
}
