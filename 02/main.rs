struct Range {
		start: i64,
		end: i64,
}

fn parse_input() -> std::io::Result<Vec<Range>> {
		Ok(std::fs::read_to_string("input.txt")?
			 .strip_suffix('\n').unwrap()
			 .split(',')
			 .map(|range| range.split('-')
						.map(|x| x.parse().unwrap())
						.collect::<Vec<_>>())
			 .map(|range| Range{start: range[0], end: range[1]})
			 .collect())
}

fn invalid_id_1(id: &i64) -> bool {
		let size = id.ilog10() + 1;
		if size % 2 != 0 { false }
		else { id % (10_i64.pow(size/2)+1) == 0 }
}

fn f(a: u32, b: u32) -> i64 {
		let mut result = 1;
		let c = b/a - 1;
		for _ in 0..c {
				result = result * 10_i64.pow(a) + 1
		}
		result
}

fn invalid_id_2(id: &i64) -> bool {
		let size = id.ilog10() + 1;
		(1..size).map(|x|
				if size % x != 0 { false }
				else { id % f(x, size) == 0 })
				.any(|b| b)
}

fn count_invalid(ranges: &[Range], invalid: fn(&i64)->bool) -> i64{
		ranges.into_iter()
				.map(|range| (range.start..=range.end).filter(invalid))
				.flatten()
				.sum()
}

fn main() -> std::io::Result<()> {
		let ranges = parse_input()?;
		println!("Part 1: {}", count_invalid(&ranges, invalid_id_1));
		println!("Part 2: {}", count_invalid(&ranges, invalid_id_2));
		Ok(())
}
