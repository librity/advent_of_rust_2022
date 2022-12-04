/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part1.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 19:07:52 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::get_input_lines;

fn split_compartments(rucksack: String) -> (String, String) {
    let middle = rucksack.len() / 2;

    (
        rucksack[..middle].to_string(),
        rucksack[middle..].to_string(),
    )
}

fn find_common_type(compartments: (String, String)) -> char {
    let (left, right) = compartments;

    for c in left.chars() {
        if right.contains(c) {
            return c;
        }
    }

    return ' ';
}

fn _debug_common_type(common_type: char) {
    println!(
        "{} {} {}",
        common_type,
        common_type as u32,
        resolve_priority(common_type)
    );
}

fn resolve_priority(common_type: char) -> u32 {
    if common_type.is_ascii_lowercase() {
        return common_type as u32 - 'a' as u32 + 1;
    }

    if common_type.is_ascii_uppercase() {
        return common_type as u32 - 'A' as u32 + 27;
    }

    return 0;
}

pub fn call() {
    let mut total_priorities = 0;

    for result in get_input_lines() {
        let rucksack = result.unwrap();

        let compartments = split_compartments(rucksack);
        let common_type = find_common_type(compartments);
        // _debug_common_type(common_type);

        total_priorities += resolve_priority(common_type);
    }

    println!("Part 1: {} priorities.", total_priorities);
}
