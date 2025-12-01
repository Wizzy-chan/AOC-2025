fn parse_input() -> std::io::Result<Vec<i32>> {
		Ok(std::fs::read_to_string("input.txt")?
				.split('\n')
				.take_while(|x| x.len() > 0)
				.map(|x| {
						let n: i32 = x[1..].parse().unwrap();
						match x.chars().nth(0) {
								Some('R') => n,
								Some('L') => -n,
								_ => todo!(),
				}})
				.collect())
}

fn part_1(rotations: &[i32]) -> i32 {
		let mut dial = 50;
		let mut result = 0;
		for rotation in rotations {
				dial = dial + rotation;
				if dial % 100 == 0 {
						result += 1;
				}
		}
		result
}

fn part_2(rotations: &[i32]) -> i32 {
		let mut dial = 50;
		let mut result = 0;
		for rotation in rotations {
				for _ in 0..rotation.abs() {
						dial = dial + rotation.signum();
						if dial % 100 == 0 {
								result += 1;
						}
				}				
		}
		result
}

fn main() -> std::io::Result<()> {
		let nums = parse_input()?;
		println!("Part 1: {}\nPart 2: {}", part_1(&nums), part_2(&nums));
		Ok(())
}
