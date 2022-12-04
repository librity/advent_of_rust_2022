/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 22:36:53 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/04 19:06:01 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::utils;

mod part1;
mod part2;

const INPUT_FILE: &str = "inputs/dayx";

pub fn get_input_lines() -> utils::FileLines {
    utils::get_input_lines(INPUT_FILE)
}

pub fn call() {
    println!("=== Day x ===");
    part1::call();
    part2::call();
}
