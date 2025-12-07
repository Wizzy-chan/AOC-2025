#[derive(PartialEq, Clone)]
enum Cell {
		Empty,
		Beam(usize),
		Splitter,
}

fn parse_input() -> Vec<Vec<Cell>> {
		std::fs::read_to_string("input.txt").unwrap()
				.lines()
				.filter(|line| line.len() > 0)
				.map(|line| line.chars().map(|c| match c {
						'S' => Cell::Beam(1),
						'^' => Cell::Splitter,
						_ => Cell::Empty})
						 .collect())
				.collect()
}

fn part_1(manifold: Vec<Vec<Cell>>) -> usize {
		let mut iter = manifold.into_iter();
		let first = iter.next().unwrap();
		let (_, splits) = iter.fold((first, 0), |(above, splits), row| {
				let (new_row, new_splits): (_, Vec<_>) = (0..row.len()).map(|i| {
						if i > 0 {
								if row[i-1] == Cell::Splitter && above[i-1] == Cell::Beam(1) { return (Cell::Beam(1), 0) }
						}
						if i < row.len() - 1 {
								if row[i+1] == Cell::Splitter && above[i+1] == Cell::Beam(1) { return (Cell::Beam(1), 0) }
						}
						if above[i] == Cell::Beam(1) && row[i] == Cell::Empty { (Cell::Beam(1), 0) }
						else if above[i] == Cell::Beam(1) && row[i] == Cell::Splitter { (Cell::Splitter, 1) }
						else { (Cell::Empty, 0) }
				}).unzip();
				(new_row, splits + new_splits.into_iter().sum::<usize>())
		});
		splits
}

fn part_2(manifold: Vec<Vec<Cell>>) -> usize {
		let final_row = manifold.into_iter().reduce(|above, row| {
				(0..row.len()).map(|i| {
						if row[i] == Cell::Splitter { return Cell::Splitter }
						let mut beam_strength = 0;
						if i > 0 {
								if let Cell::Beam(s) = above[i-1] {
										if row[i-1] == Cell::Splitter { beam_strength += s }
								}
						}
						if i < row.len() - 1 {
								if let Cell::Beam(s) = above[i+1] {
										if row[i+1] == Cell::Splitter { beam_strength += s }
								}
						}
						if let Cell::Beam(s) = above[i] { beam_strength += s }
						if beam_strength > 0 { Cell::Beam(beam_strength) }
						else { Cell::Empty }													 
				}).collect()
		}).unwrap();
		final_row.into_iter().fold(0, |acc, cell| {
				if let Cell::Beam(s) = cell { acc + s }
				else { acc }
		})
}

fn main() {
		let manifold = parse_input();
		println!("{}", part_1(manifold.clone()));
		println!("{}", part_2(manifold.clone()));
}
