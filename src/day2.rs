#[derive(Debug, PartialEq)]
enum Roshambo {
    Rock,
    Paper,
    Scissors,
}

type Strat = (Roshambo, Roshambo);

fn parse_strat(s: &str) -> Strat {
    let spl = s.split_once(" ").unwrap();
    let them = match spl.0 {
        "A" => Roshambo::Rock,
        "B" => Roshambo::Paper,
        "C" => Roshambo::Scissors,
        _ => panic!(),
    };
    let us = match spl.1 {
        "X" => Roshambo::Rock,
        "Y" => Roshambo::Paper,
        "Z" => Roshambo::Scissors,
        _ => panic!(),
    };
    (them, us)
}

fn calc_choice_score((_, us): &Strat) -> i32 {
    match us {
        Roshambo::Rock => 1,
        Roshambo::Paper => 2,
        Roshambo::Scissors => 3,
    }
}

fn calc_winner_score(strat: &Strat) -> i32 {
    match strat {
        (Roshambo::Rock, Roshambo::Paper)
        | (Roshambo::Scissors, Roshambo::Rock)
        | (Roshambo::Paper, Roshambo::Scissors) => 6,
        (x, y) if x == y => 3,
        _ => 0,
    }
}

fn part1(input: &str) -> i32 {
    let strats: Vec<_> = input.lines().map(|l| parse_strat(l)).collect();
    let mut score = 0;
    for strat in strats {
        score += calc_choice_score(&strat);
        score += calc_winner_score(&strat);
    }
    score
}

fn part2(input: &str) -> i32 {
    unimplemented!();
}

fn main() {
    let input = include_str!("../input/day2.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day2.txt");

    #[test]
    fn parse_test() {
        assert_eq!(parse_strat("B X"), (Roshambo::Paper, Roshambo::Rock));
    }

    #[test]
    fn choice_score_test(){
        assert_eq!(calc_choice_score(&(Roshambo::Rock, Roshambo::Paper)), 2);

    }

    #[test]
    fn winner_score_test_win() {
        assert_eq!(calc_winner_score(&(Roshambo::Paper, Roshambo::Scissors)), 6);
    }

    #[test]
    fn winner_score_lose() {
        assert_eq!(calc_winner_score(&(Roshambo::Rock, Roshambo::Scissors)), 0);
    }

    #[test]
    fn winner_score_draw() {
        assert_eq!(calc_winner_score(&(Roshambo::Rock, Roshambo::Rock)), 3);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 15);
    }
}
