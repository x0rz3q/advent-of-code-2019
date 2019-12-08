use std::iter::Iterator;

struct Image {
	layers: Vec<Vec<usize>>,
	width: usize,
	height: usize,
}

impl Image {
	fn parse(&mut self, input: Vec<usize>) {
		let loops = input.len() / (self.width * self.height);
		let mut index = 0;

		for _ in 0..loops {
			let mut layer: Vec<usize> = Vec::new();
			for _ in 0..self.height {
				for _ in 0..self.width {
					layer.push(input[index]);
					index = index + 1;
				}
			}

			self.layers.push(layer);
		}
	}

	fn decode(&mut self) -> Vec<Vec<usize>> {
		let mut result: Vec<Vec<usize>> = Vec::new();

		for _ in 0..self.height {
			let mut row: Vec<usize> = Vec::new();
			for _ in 0..self.width {
				row.push(2);
			}

			result.push(row);
		}

		let count = self.layers.len();
		for k in 0..count {
			let mut index = 0;
			for i in 0..self.height {
				for j in 0..self.width {
					if result[i][j] == 2 {
						result[i][j] = self.layers[k][index];
					}

					index += 1;
				}
			}
		}

		result
	}
}

fn main() {
	let input: Vec<usize> = include_str!("input")
		.trim()
		.chars()
		.map(|num| num.to_digit(10).unwrap() as usize)
		.collect();

	let mut image = Image {
		layers: Vec::new(),
		width: 25,
		height: 6,
	};

	image.parse(input);

	let mut index = 0;
	let mut min = usize::max_value();
	let mut mindex = 0;

	for layer in &image.layers {
		let count = layer.iter().filter(|&&i| i == 0).count();

		if count < min {
			mindex = index;
			min = count;
		}

		index += 1;
	}

	let layer = &image.layers[mindex];
	let mut silver = layer.iter().filter(|&&i| i == 1).count();
	silver *= layer.iter().filter(|&&i| i == 2).count();

	println!("Silver: {}", silver);
	println!("Gold: ");

	let result = image.decode();
	for i in 0..image.height {
		for j in 0..image.width {
			if result[i][j] == 1 {
				print!("▮",);
			} else if result[i][j] == 0 {
				print!(" ");
			}
		}

		println!("");
	}
}
