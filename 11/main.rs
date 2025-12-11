use std::collections::HashMap;

type DeviceMap = HashMap<String, Vec<String>>;

fn parse_input() -> DeviceMap {
		std::fs::read_to_string("input.txt").unwrap()
				.lines()
				.filter_map(|line| {
						let [name, outputs] = &line.split(": ").collect::<Vec<_>>()[..] else { return None };
						let outputs = outputs.split(" ").map(|output| output.to_string()).collect();
						Some((name.to_string(), outputs))
				})
				.collect() 
}

fn part_1(devices: &DeviceMap, device: &str) -> usize {
		if device == "out" { 1 }
		else {
				let Some(outputs) = &devices.get(device) else { panic!("No entry for device {}", device) };
				outputs.iter().map(|output| part_1(devices, output)).sum()
		}
}

fn part_2<'a>(devices: &'a DeviceMap, device: &'a str, visited_fft: bool, visited_dac: bool, memo: &mut HashMap<(&'a str, bool, bool), usize>) -> usize {
		if device == "out" {
				if visited_dac && visited_fft { 1 }
				else { 0 }
		}
		else {
				let visited_fft = if device == "fft" { true } else { visited_fft };
				let visited_dac = if device == "dac" { true } else { visited_dac };
				if let Some(&routes) = memo.get(&(device, visited_fft, visited_dac)) { return routes }
				let Some(outputs) = &devices.get(device) else { panic!("No entry for device {}", device) };
				let routes = outputs.iter().map(|output| part_2(devices, output, visited_fft, visited_dac, memo)).sum();
				memo.insert((device, visited_fft, visited_dac), routes);
				routes
		}
}

fn main() {
		let devices = parse_input();
		println!("Part 1: {}", part_1(&devices, "you"));
		println!("Part 2: {}", part_2(&devices, "svr", false, false, &mut HashMap::new()));
}
