/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 22:36:53 by lpaulo-m          #+#    #+#             */
/*   Updated: 2022/12/03 23:43:37 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod part1;
mod part2;

pub const INPUT_FILE: &str = "inputs/day1";

pub fn call() {
    println!("=== Day 1 ===");
    part1::print_max_calories();
    part2::print_top_three_sum();
}
