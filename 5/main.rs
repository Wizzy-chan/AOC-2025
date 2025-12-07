#[derive(Clone, Copy)]
struct Range {
		start: usize,
		end: usize,
}

fn parse_input() -> (Vec<Range>, Vec<usize>) {
		let content = std::fs::read_to_string("input.txt").unwrap();
		let mut input = content.split("\n\n");
		let ranges = input.next().expect("Set of ranges expected")
				.split('\n')
				.take_while(|range| range.len() > 0)
				.map(|range| {
						 let mut split = range.split('-');
						 let start = split.next().unwrap().parse().unwrap();
						 let end = split.next().unwrap().parse().unwrap();
						Range{start, end}
				})
				.collect();
		let ids = input.next().expect("Set of ids expected")
				.split('\n')
				.take_while(|id| id.len() > 0)
				.map(|id| id.parse().unwrap())
				.collect();
		(ranges, ids)
}

fn part_1(ranges: &[Range], ids: &[usize]) -> usize {
		ids.into_iter()
				.filter(|&&id|
								ranges.into_iter()
								.any(|range| id >= range.start && id <= range.end))
				.count()
}

fn add_range(ranges: &mut Vec<Range>, new_range: Range) {
		for i in 0..ranges.len() {
				let range = ranges[i];
				if new_range.start <= range.end && range.start <= new_range.end {
						ranges[i] = ranges[ranges.len()-1];
						_ = ranges.pop();
						let start = if range.start < new_range.start { range.start } else { new_range.start };
						let end = if range.end > new_range.end { range.end } else { new_range.end };
						add_range(ranges, Range{start, end});
						return;
				}
		}
		ranges.push(new_range);
}

fn part_2(ranges: &[Range]) -> usize {
		let mut merged_ranges = vec![];
		for &range in ranges {
				add_range(&mut merged_ranges, range);
		}
		merged_ranges.into_iter().map(|range| range.end - range.start + 1).sum()
}

fn main() {
		let (ranges, ids) = parse_input();
		println!("{}", part_1(&ranges, &ids));
		println!("{}", part_2(&ranges));
}
