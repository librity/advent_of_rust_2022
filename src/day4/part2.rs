/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part2.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 21:06:22 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::utils::str_to_int;

use super::get_input_lines;

fn ranges_overlap(left: (i32, i32), right: (i32, i32)) -> bool {
    let (left_start, left_end) = left;
    let (right_start, right_end) = right;

    if left_start <= right_end && left_end >= right_start {
        return true;
    }
    return false;
}

pub fn call() {
    let mut total_overlaps = 0;

    for result in get_input_lines() {
        let line = result.unwrap();

        let tokens: Vec<&str> = line.split(&[',', '-']).collect();
        let left = (str_to_int(tokens[0]), str_to_int(tokens[1]));
        let right = (str_to_int(tokens[2]), str_to_int(tokens[3]));

        if ranges_overlap(left, right) {
            total_overlaps += 1;
        }
    }

    println!("Part 1: {} total overlaps.", total_overlaps);
}
