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
            acc += 1;
        }
        acc
    })
}

// 4 cases of overlapping:
// ....5678.
// .23456...
// ranges[1][1] >= ranges[0][0] && ranges[1][1] <= ranges[0][1]

// .234567..
// .....678.
// ranges[1][0] <= ranges[0][1] && ranges[1][0] >= ranges[0][0]

// And cases of containing fully another range from part 1

pub fn part2(input: String) -> u32 {
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

        if (ranges[1][1] >= ranges[0][0] && ranges[1][1] <= ranges[0][1])
            || (ranges[1][0] <= ranges[0][1] && ranges[1][0] >= ranges[0][0])
            || (ranges[0][0] <= ranges[1][0] && ranges[0][1] >= ranges[1][1])
            || (ranges[1][0] <= ranges[0][0] && ranges[1][1] >= ranges[0][1])
        {
            acc += 1;
        }
        acc
    })
}
