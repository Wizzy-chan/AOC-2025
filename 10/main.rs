use std::cmp::min;

struct Machine {
		target: String,
		buttons: Vec<Vec<usize>>,
}

fn parse_input() -> Vec<Machine> {
		std::fs::read_to_string("input.txt").unwrap()
				.lines()
				.map(|line| {
						let [mut light_pattern, button_tuples @ .., _] =
								&line.split(' ').collect::<Vec<_>>()[..] else { todo!() };
						light_pattern = light_pattern.strip_prefix('[').unwrap();
						light_pattern = light_pattern.strip_suffix(']').unwrap();
						let target = light_pattern.to_string();
						let buttons = button_tuples.iter().map(|&(mut tuple)| {
								tuple = tuple.strip_prefix('(').unwrap();
								tuple = tuple.strip_suffix(')').unwrap();
								tuple.split(',').map(|x| x.parse::<usize>().unwrap()).collect()
						}).collect();
						Machine{target, buttons}
				})
				.collect()
}

fn solve_machine_1(partial: &mut Vec<usize>, machine: &Machine) -> Option<usize> {
		if partial.len() == machine.buttons.len() {
				let mut joltages: Vec<_> = (0..machine.target.len()).map(|_| 0).collect();
				for i in 0..partial.len() {
						for &joltage in &machine.buttons[i] {
								joltages[joltage] += partial[i]
						}
				}
				if (0..machine.target.len()).all(|i| {
						match machine.target.chars().nth(i) {
								Some('#') => joltages[i] % 2 == 1,
								Some('.') => joltages[i] % 2 == 0,
								_ => todo!()
						}
				}) {
						return Some(partial.iter().sum())
				} else {
						return None
				}
		}
		let mut sols = vec![];
		for i in 0..=1 {
				partial.push(i);
				if let Some(cost) = solve_machine_1(partial, machine) {
						sols.push(cost);
				}
				partial.pop();
		}
		sols.iter().reduce(min).copied()
}

fn part_1(machine: &Machine) -> usize {
		let mut presses = vec![];
		return solve_machine_1(&mut presses, machine).unwrap()
}

fn solve(machines: &[Machine], func: fn(&Machine) -> usize) -> usize {
		machines.iter().map(func).sum()
}

fn main() {
		let machines = parse_input();
  	println!("Part 1: {}", solve(&machines, part_1));
}
