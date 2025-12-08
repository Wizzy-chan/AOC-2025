use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct JunctionBox {
		x: isize,
		y: isize,
		z: isize,
}

fn parse_input() -> Vec<JunctionBox> {
		std::fs::read_to_string("input.txt").unwrap()
				.lines()
				.filter(|line| line.len() > 0)
				.map(|line| {
						let mut iter = line.split(',');
						let x = iter.next().unwrap().parse().unwrap();
						let y = iter.next().unwrap().parse().unwrap();
						let z = iter.next().unwrap().parse().unwrap();
						JunctionBox{x, y, z}
				})
				.collect()
}

fn make_sets(boxes: &[JunctionBox]) -> Vec<HashSet<&JunctionBox>> {
		let mut sets = vec![];
		for j in boxes {
				let mut set = HashSet::new();
				set.insert(j);
				sets.push(set);
		}
		sets
}

fn precompute_square_distances(boxes: &[JunctionBox]) -> Vec<(&JunctionBox, &JunctionBox, isize)> {
		let mut square_distances = vec![];
		for i in 0..boxes.len() {
				let j1 = &boxes[i];
				for j in i+1..boxes.len() {
						let j2 = &boxes[j];
						let square_dist = (j1.x - j2.x).pow(2) + (j1.y - j2.y).pow(2) + (j1.z - j2.z).pow(2);
						square_distances.push((j1, j2, square_dist));
				}
		}
		square_distances
}

fn part_1(boxes: &[JunctionBox], square_distances: &[(&JunctionBox, &JunctionBox, isize)]) -> usize {
		let mut sets = make_sets(&boxes);
		for i in 0..1000 {
				let (j1, j2, _) = square_distances[i];
				let j1i = sets.iter().position(|set| set.contains(j1)).unwrap();
				let j2i = sets.iter().position(|set| set.contains(j2)).unwrap();
				if j1i != j2i {
						sets[j1i] = sets[j1i].union(&sets[j2i]).map(|&j| j).collect();
						sets[j2i] = sets[sets.len()-1].clone();
						sets.pop();
				}
		}
		let mut lengths: Vec<_> = sets.iter().map(|set| set.len()).collect();
		lengths.sort_unstable_by(|a, b| b.cmp(a));
		lengths[0] * lengths[1] * lengths[2]
}

fn part_2(boxes: &[JunctionBox], square_distances: &[(&JunctionBox, &JunctionBox, isize)]) -> isize {
		let mut sets = make_sets(&boxes);
		let mut i = 0;
		while sets.len() > 1 {
				let (j1, j2, _) = square_distances[i];
				let j1i = sets.iter().position(|set| set.contains(j1)).unwrap();
				let j2i = sets.iter().position(|set| set.contains(j2)).unwrap();
				if j1i != j2i {
						sets[j1i] = sets[j1i].union(&sets[j2i]).map(|&j| j).collect();
						sets[j2i] = sets[sets.len()-1].clone();
						sets.pop();
				}
				i += 1;
		}
		let (j1, j2, _) = square_distances[i-1];
		j1.x * j2.x
}

fn main() {
		let boxes = parse_input();
		let mut square_distances = precompute_square_distances(&boxes);
		square_distances.sort_unstable_by(|(_, _, a), (_, _, b)| a.cmp(b));
		println!("Part 1: {}", part_1(&boxes, &square_distances));
		println!("Part 2: {}", part_2(&boxes, &square_distances));
}
