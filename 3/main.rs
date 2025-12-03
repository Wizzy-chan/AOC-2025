fn parse_input() -> Vec<Vec<u64>>{
		std::fs::read_to_string("input.txt").unwrap()
				.split('\n')
				.take_while(|x| x.len() > 0)
				.map(|x| x.chars()
						 .map(|c| c.to_digit(10).unwrap().into())
						 .collect())
				.collect()				
}

fn make_number(n: &[u64]) -> u64 {
		let mut result = 0;
		for d in n {
				result = result * 10 + d;
		}
		result
}

fn part_1(data: &[Vec<u64>]) -> u64 {
		data.into_iter().map(|row| {
				let (d1, d2) = row.into_iter().fold((0, 0), |(d1, d2), &digit| {
						if d2 > d1 { (d2, digit) }
						else if digit > d2 { (d1, digit) }
						else { (d1, d2) }
				});
				make_number(&[d1, d2])})
				.sum()
}

fn rotate<const N: usize>(array: [u64; N], index: usize, el: u64) -> [u64; N] {
		let mut result: [u64; N] = [0; N];
		for i in 0..N-1 {
				if i < index {
						result[i] = array[i]
				} else {
						result[i] = array[i+1]
				}
		}
		result[N-1] = el;
		result
}

fn solve<const N: usize>(data: &[Vec<u64>]) -> u64 {
		data.into_iter().map(|row| {
				let n: [u64; N] = row.into_iter().fold([0; N], |mut n, &digit| {
						for i in 0..N-1 {
								if n[i] < n[i+1] { return rotate(n, i, digit) }
						}
						if n[N-1] < digit { n[N-1] = digit }
						n
				});
				make_number(&n)})
				.sum()
}

fn main() {
		let lines = parse_input();
		println!("Part 1: {}", part_1(&lines));
		println!("Part 1 (with part 2 function): {}", solve::<2>(&lines));
		println!("Part 2: {}", solve::<12>(&lines));
}
