/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part2.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 20:12:33 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::get_input_lines;

fn find_common_type(group: &(&String, &String, &String)) -> char {
    let (first, second, third) = group;

    for c in first.chars() {
        if second.contains(c) && third.contains(c) {
            return c;
        }
    }

    return ' ';
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
    let mut line_count = 1;
    let mut total_priorities = 0;
    let mut rucksacks: Vec<String> = Vec::new();

    for result in get_input_lines() {
        let rucksack = result.unwrap();

        rucksacks.push(rucksack);
        if line_count % 3 == 0 {
            let ref group = (&rucksacks[0], &rucksacks[1], &rucksacks[2]);
            let common_type = find_common_type(group);
            total_priorities += resolve_priority(common_type);
            rucksacks.clear();
        }

        line_count += 1;
    }

    println!("Part 1: {} priorities.", total_priorities);
}
