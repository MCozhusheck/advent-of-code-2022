#[derive(Debug, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    enemy_play: Play,
    my_play: Play,
}

impl TryFrom<&str> for Round {
    type Error = &'static String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let enemy_play = match value.chars().nth(0) {
            Some('A') => Play::Rock,
            Some('B') => Play::Paper,
            Some('C') => Play::Scissors,
            _ => panic!(),
        };

        let my_play = match value.chars().nth(2) {
            Some('X') => Play::Rock,
            Some('Y') => Play::Paper,
            Some('Z') => Play::Scissors,
            _ => panic!(),
        };

        Ok(Round {
            enemy_play,
            my_play,
        })
    }
}

impl Round {
    const WON_MATCH_POINTS: u32 = 6;
    const DRAW_MATCH_POINTS: u32 = 3;
    const LOST_MATCH_POINTS: u32 = 0;

    pub fn match_outcome(&self) -> u32 {
        if (self.enemy_play == Play::Rock && self.my_play == Play::Paper)
            || (self.enemy_play == Play::Paper && self.my_play == Play::Scissors)
            || (self.enemy_play == Play::Scissors && self.my_play == Play::Rock)
        {
            Round::WON_MATCH_POINTS
        } else if self.enemy_play == self.my_play {
            Round::DRAW_MATCH_POINTS
        } else {
            Round::LOST_MATCH_POINTS
        }
    }

    pub fn bonus_points(&self) -> u32 {
        match self.my_play {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    pub fn total_score(&self) -> u32 {
        self.match_outcome() + self.bonus_points()
    }
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

struct Round2 {
    enemy_play: Play,
    expected_outcome: Outcome,
}

impl TryFrom<&str> for Round2 {
    type Error = &'static String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let enemy_play = match value.chars().nth(0) {
            Some('A') => Play::Rock,
            Some('B') => Play::Paper,
            Some('C') => Play::Scissors,
            _ => panic!(),
        };

        let expected_outcome = match value.chars().nth(2) {
            Some('X') => Outcome::Lose,
            Some('Y') => Outcome::Draw,
            Some('Z') => Outcome::Win,
            _ => panic!(),
        };

        Ok(Round2 {
            enemy_play,
            expected_outcome,
        })
    }
}

impl Round2 {
    const WON_MATCH_POINTS: u32 = 6;
    const DRAW_MATCH_POINTS: u32 = 3;
    const LOST_MATCH_POINTS: u32 = 0;

    pub fn decide_my_play(&self) -> Play {
        if (self.enemy_play == Play::Rock && self.expected_outcome == Outcome::Lose)
            || (self.enemy_play == Play::Paper && self.expected_outcome == Outcome::Win)
            || (self.enemy_play == Play::Scissors && self.expected_outcome == Outcome::Draw)
        {
            Play::Scissors
        } else if (self.enemy_play == Play::Paper && self.expected_outcome == Outcome::Lose)
            || (self.enemy_play == Play::Scissors && self.expected_outcome == Outcome::Win)
            || (self.enemy_play == Play::Rock && self.expected_outcome == Outcome::Draw)
        {
            Play::Rock
        } else {
            Play::Paper
        }
    }

    pub fn match_outcome(&self) -> u32 {
        if (self.enemy_play == Play::Rock && self.decide_my_play() == Play::Paper)
            || (self.enemy_play == Play::Paper && self.decide_my_play() == Play::Scissors)
            || (self.enemy_play == Play::Scissors && self.decide_my_play() == Play::Rock)
        {
            Round::WON_MATCH_POINTS
        } else if self.enemy_play == self.decide_my_play() {
            Round::DRAW_MATCH_POINTS
        } else {
            Round::LOST_MATCH_POINTS
        }
    }

    pub fn bonus_points(&self) -> u32 {
        match &self.decide_my_play() {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    pub fn total_score(&self) -> u32 {
        self.match_outcome() + self.bonus_points()
    }
}

pub fn part1(input: String) -> u32 {
    input.split("\n").fold(0, |mut acc, round| {
        if let Ok(result) = Round::try_from(round) {
            acc += result.total_score();
        }
        acc
    })
}

pub fn part2(input: String) -> u32 {
    input.split("\n").fold(0, |mut acc, round| {
        if let Ok(result) = Round2::try_from(round) {
            acc += result.total_score();
        }
        acc
    })
}
