fn parse_input() -> Vec<Vec<bool>> {
		std::fs::read_to_string("input.txt").unwrap()
				.split('\n')
				.take_while(|row| row.len() > 0)
				.map(|row|
						 row.chars().map(|roll| match roll {
								 '@' => true,
								 _ => false,
						 })
						 .collect())
				.collect()
}

fn get_neighbours(row: usize, col: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
		let lr = if row > 0 { row - 1 } else { row };
		let ur = if row < height - 1 { row + 1 } else { row };
		let lc = if col > 0 { col - 1 } else { col };
		let uc = if col < width - 1 { col + 1 } else { col };
		(lr..=ur).map(|r| (lc..=uc).map(move |c| (r, c))).flatten().collect()
}

fn count_rolls(rolls: &[Vec<bool>]) -> usize {
		rolls.into_iter().map(|row| row.into_iter().filter(|&&roll| roll).count()).sum()
}

fn remove_rolls(rolls: &[Vec<bool>]) -> Vec<Vec<bool>> {
		let height = rolls.len();
		let width = rolls[0].len();
		(0..rolls.len()).map(|row| {
				(0..rolls[row].len()).map(|col| {
						if rolls[row][col] {
								get_neighbours(row, col, width, height)
										.into_iter()
										.filter(|&(row, col)| rolls[row][col])
										.count() > 4
						} else { false }
				}).collect()
		}).collect()
}

fn part_1(rolls: &[Vec<bool>]) -> usize {
		let after_iter = remove_rolls(rolls);
		count_rolls(rolls) - count_rolls(&after_iter)
}

fn part_2(mut rolls: Vec<Vec<bool>>) -> usize {
		let mut total_removed = 0;
		loop {
				let after_iter = remove_rolls(&rolls);
				let removed = count_rolls(&rolls) - count_rolls(&after_iter);
				total_removed += removed;
				rolls = after_iter;
				if removed == 0 { break; }
		}
		total_removed
}

fn main() {
		let rolls = parse_input();
		println!("{}", count_rolls(&rolls));
		println!("{}", part_1(&rolls));
		println!("{}", part_2(rolls));
}
