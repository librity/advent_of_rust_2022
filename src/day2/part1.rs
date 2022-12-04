/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part1.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 11:03:32 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::get_input_lines;

fn resolve_my_score(my_play: &str) -> i32 {
    match my_play {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn resolve_round_score(game: (&str, &str)) -> i32 {
    match game {
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("C", "Y") => 0,

        ("A", "X") => 3,
        ("B", "Y") => 3,
        ("C", "Z") => 3,

        ("A", "Y") => 6,
        ("B", "Z") => 6,
        ("C", "X") => 6,

        _ => 0,
    }
}

pub fn call() {
    let mut total_score = 0;

    for result in get_input_lines() {
        let line: String = result.unwrap();
        let tokens: Vec<&str> = line.split(" ").collect();
        let round = (tokens[0], tokens[1]);
        let (_, me) = round;

        total_score += resolve_my_score(me);
        total_score += resolve_round_score(round);
    }

    println!("Part 1: {} total score.", total_score);
}
