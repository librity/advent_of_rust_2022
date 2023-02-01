/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   part1.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lpaulo-m <lpaulo-m@student.42sp.org.br>    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/12/03 20:56:59 by lpaulo-m          #+#    #+#             */
/*   Updated: 2023/02/01 09:52:11 by lpaulo-m         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::get_input_lines;

//             [L] [M]         [M]    
//         [D] [R] [Z]         [C] [L]
//         [C] [S] [T] [G]     [V] [M]
// [R]     [L] [Q] [B] [B]     [D] [F]
// [H] [B] [G] [D] [Q] [Z]     [T] [J]
// [M] [J] [H] [M] [P] [S] [V] [L] [N]
// [P] [C] [N] [T] [S] [F] [R] [G] [Q]
// [Z] [P] [S] [F] [F] [T] [N] [P] [W]
//  1   2   3   4   5   6   7   8   9 

fn handle_move(line: String, stacks: &mut Vec<Vec<String>>) {
    println!("move");
    
    // let mut words = line.split_whitespace();
    // let crate_name = words.nth(1).unwrap().to_string();
    // let mut stack_index = words.nth(5).unwrap().parse::<usize>().unwrap();
    // stack_index -= 1;

    // for stack in stacks.iter_mut() {
    //     if let Some(index) = stack.iter().position(|x| *x == crate_name) {
    //         stack.remove(index);
    //         break;
    //     }
    // }

    // stacks[stack_index].push(crate_name);
}

fn handle_stacks(line: String, stacks: &mut Vec<Vec<String>>) {
    println!("stacks");

    // let mut words = line.split_whitespace();
    // let mut stack_index = words.nth(1).unwrap().parse::<usize>().unwrap();
    // stack_index -= 1;
    // let mut stack_count = words.nth(3).unwrap().parse::<usize>().unwrap();
    // stack_count -= 1;
    
    // let mut stack = stacks[stack_index].clone();
    // stacks[stack_index].clear();
    
    // for i in 0..stack_count {
    //     stacks[stack_index + i].append(&mut stack);
    //     stack = stacks[stack_index + i + 1].clone();
    //     stacks[stack_index + i + 1].clear();
    // }
}

pub fn call() {
    let mut top_crates = "ABC";
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let stack_count = 9;
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for result in get_input_lines() {
        let line = result.unwrap();
        
        if (line.contains("move")){
            
            handle_move(line, &mut stacks);
        } else {
            handle_stacks(line, &mut stacks);
        }
    }

    for stack in stacks.iter() {
        stack.iter().for_each(|x| print!("{} ", x));
        println!();
    }

    println!("Part 1: the top crates are {}.", top_crates);
}
