/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part2.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 11:24:09 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::get_input_lines;

const ROCK: &str = "A";
const PAPER: &str = "B";
const SCISSORS: &str = "C";

const LOSE: &str = "X";
const DRAW: &str = "Y";
const WIN: &str = "Z";

fn resolve_round_score(result: &str) -> i32 {
    match result {
        LOSE => 0,
        DRAW => 3,
        WIN => 6,
        _ => 0,
    }
}

fn resolve_my_score(round: (&str, &str)) -> i32 {
    match round {
        (ROCK, LOSE) => 3,
        (PAPER, LOSE) => 1,
        (SCISSORS, LOSE) => 2,

        (ROCK, DRAW) => 1,
        (PAPER, DRAW) => 2,
        (SCISSORS, DRAW) => 3,

        (ROCK, WIN) => 2,
        (PAPER, WIN) => 3,
        (SCISSORS, WIN) => 1,

        _ => 0,
    }
}

pub fn call() {
    let mut total_score = 0;

    for result in get_input_lines() {
        let line: String = result.unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        let round = (tokens[0], tokens[1]);
        let (_, result) = round;

        total_score += resolve_round_score(result);
        total_score += resolve_my_score(round);
    }

    println!("Part 2: {} total score.", total_score);
}
