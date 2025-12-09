use std::cmp::{min, max};

struct Tile {
		x: usize,
		y: usize,
}

enum Edge {
		Horizontal{y: usize, x1: usize, x2: usize}, // x1 < x2
		Vertical{x: usize, y1: usize, y2: usize}, // y1 < y2
}
use Edge::{Horizontal, Vertical};


fn edges(tiles: &[Tile]) -> Vec<Edge> {
		(0..tiles.len()).map(|i| {
				let tile1 = &tiles[i];
				let tile2 = &tiles[(i+1)%tiles.len()];
				if tile1.x == tile2.x {
						Vertical{x: tile1.x, y1: min(tile1.y, tile2.y), y2: max(tile1.y, tile2.y)}
				} else {
						Horizontal{y: tile1.y, x1: min(tile1.x, tile2.x), x2: max(tile1.x, tile2.x)}
				}
		}).collect()
}

fn edge_in(edge: &Edge, tile1: &Tile, tile2: &Tile) -> bool {
		let left = min(tile1.x, tile2.x);
		let right = max(tile1.x, tile2.x);
		let bottom = min(tile1.y, tile2.y);
		let top = max(tile1.y, tile2.y);
		match edge {
				&Horizontal{y, x1, x2} => {
						if y <= bottom || y >= top { false }
						else { !(x2 <= left || x1 >= right) }
				}
				&Vertical{x, y1, y2} => {
						if x <= left || x >= right { false }
						else { !(y2 <= bottom || y1 >= top) }
				}
		}
}

fn parse_input() -> Vec<Tile> {
		std::fs::read_to_string("input.txt").unwrap()
				.lines()
				.filter(|line| line.len() > 0)
				.map(|line| if let [xx, yy] = line.split(',').collect::<Vec<_>>()[..] {
						let x = xx.parse().unwrap();
						let y = yy.parse().unwrap();
						Tile{x, y}
				} else { todo!() })
				.collect()
}

fn part_1(tiles: &[Tile]) -> usize {
		(0..tiles.len()).map(|i| {
				(i+1..tiles.len()).map(move |j| {
						let tile1 = &tiles[i];
						let tile2 = &tiles[j];
						let width = if tile1.x > tile2.x { tile1.x - tile2.x + 1 } else { tile2.x - tile1.x + 1 };
						let height = if tile1.y > tile2.y { tile1.y - tile2.y + 1 } else { tile2.y - tile1.y + 1 };
						width * height
				})})
				.flatten()
				.reduce(|biggest, area| if area > biggest { area } else { biggest })
				.unwrap()
}

// If the shape is concave this may find a large rectangle outside the shape.
// The AOC input is concave but the concave area is small enough that it doesn't matter.
// We also assume the shape given by the tiles does not intersect itself
fn part_2(tiles: &[Tile]) -> usize {
		let edges = edges(tiles);
		let edges_ref = &edges;
		(0..tiles.len()).map(|i| {
				(i+1..tiles.len()).map(move |j| {
						let tile1 = &tiles[i];
						let tile2 = &tiles[j];
						if edges_ref.into_iter().any(|edge| edge_in(edge, tile1, tile2)) { None }
						else {
								let width = if tile1.x > tile2.x { tile1.x - tile2.x + 1 } else { tile2.x - tile1.x + 1 };
								let height = if tile1.y > tile2.y { tile1.y - tile2.y + 1 } else { tile2.y - tile1.y + 1 };
								Some(width * height)
						}
				})})
				.flatten()
				.flatten()
				.reduce(|biggest, area| if area > biggest { area } else { biggest })
				.unwrap()
}

fn main() {
		let tiles = parse_input();
		println!("{}", part_1(&tiles));
		println!("{}", part_2(&tiles));
}
