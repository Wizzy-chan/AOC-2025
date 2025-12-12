struct Problem {
		numbers: Vec<String>,
		op: char,
}

fn parse_input() -> Vec<Problem> {
		let content = std::fs::read_to_string("input.txt").unwrap();
		let lines: Vec<_> = content.lines().collect();
		let mut problems = vec![];
		let mut last = 0;
		for i in 0..lines[0].len() {
				if lines.iter().all(|line| line.chars().nth(i) == Some(' ')) {
						let [numbers @ .., op] = &lines.iter()
								.map(|line| line[last..i].to_string())
								.collect::<Vec<_>>()[..] else { todo!() };
						problems.push(Problem{numbers: numbers.to_vec(), op: op.chars().nth(0).unwrap()});
						last = i + 1;
				}
		}
		let mut iter = lines.iter().map(|line| line[last..].to_string());
		let numbers = iter.clone().take(4).collect();
		let op = iter.nth(4).unwrap().chars().next().unwrap();
		problems.push(Problem{numbers, op});
		problems
}

fn part_1(problems: &[Problem]) -> usize {
		problems.iter().map(|problem| {
				let numbers = problem.numbers.iter().map(|n| n.trim().parse::<usize>().unwrap());
				match problem.op {
						'*' => numbers.reduce(|a, b| a*b),
						'+' => numbers.reduce(|a, b| a+b),
						_ => todo!(),
				}
		}).flatten().sum()
}

fn transpose(strings: &[String]) -> Vec<String> {
		let mut transposed = vec![];
		for i in 0..strings[0].len() {
				let mut new = String::new();
				for string in strings {
						new.push(string.chars().nth(i).unwrap());
				}
				transposed.push(new);
		}
		transposed
}

fn part_2(problems: &[Problem]) -> usize {
		problems.iter().map(|problem| {
				let transposed = transpose(&problem.numbers);
				let numbers = transposed.iter().map(|n| n.trim().parse::<usize>().unwrap());
				match problem.op {
						'*' => numbers.reduce(|a, b| a*b),
						'+' => numbers.reduce(|a, b| a+b),
						_ => todo!(),
				}
		}).flatten().sum()
}

fn main() {
		let problems = parse_input();
		println!("Part 1: {}", part_1(&problems));
		println!("Part 2: {}", part_2(&problems));
}
