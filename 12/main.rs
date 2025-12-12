fn main() {
		let present_sizes: Vec<usize> = std::fs::read_to_string("input.txt").unwrap()
				.split("\n\n")
				.map(|chunk| chunk.lines().map(|line| line.chars().filter(|&c| c == '#').count()).sum())
				.collect();
		let regions = std::fs::read_to_string("input.txt").unwrap()
				.lines()
				.filter_map(|line| {
						let [dim, presents] = &line.split(": ").collect::<Vec<_>>()[..] else { return None };
						let [x, y] = &dim.split("x").collect::<Vec<_>>()[..] else { return None };
						let x = x.parse().ok()?;
						let y = y.parse().ok()?;
						let presents = presents.split(" ").map(|present| present.parse().unwrap()).collect::<Vec<_>>();
						return Some((x, y, presents))
				}).filter(|&(x, y, ref presents): &(usize, usize, Vec<usize>)| {
						let region_size = x*y;
						let presents_size = presents.iter().enumerate().map(|(i, presents)| present_sizes[i] * presents).sum();
						region_size >= presents_size })
				.count();
		println!("Part 1: {}", regions)
}
