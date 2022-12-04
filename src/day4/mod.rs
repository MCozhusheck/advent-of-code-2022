pub fn part1(input: String) -> u32 {
    input.split("\n").fold(0, |mut acc, line| {
        let ranges: Vec<Vec<u32>> = line
            .split(",")
            .map(|range| {
                range
                    .split('-')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        if (ranges[0][0] <= ranges[1][0] && ranges[0][1] >= ranges[1][1])
            || (ranges[1][0] <= ranges[0][0] && ranges[1][1] >= ranges[0][1])
        {
            acc += 1
        }
        acc
    })
}
