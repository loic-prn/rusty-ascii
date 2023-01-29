use image::GenericImageView;
use std::fs;
use clap::{Parser, ArgGroup};

const ASCII_CHAR_ARRAY: [&str; 8] = [" ", ".", ",", "-", "~", "+", "=", "@"];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
	ArgGroup::new("required")
		.required(true)
		.args(["file"])
))]
struct Args {
	file: Option<String>,
}

fn main() {
	let arg = Args::parse().file.unwrap();
	let path = arg.as_str();
	let scale = 4;
	let output = get_image(path, scale);
	fs::write(path.to_owned() + ".txt", output).expect("Couldn't write file");
}

fn get_image(path: &str, scale: u32) -> String {
	let img = image::open(path).unwrap();
	let (width, height) = img.dimensions();
	let mut final_art = String::new();

	for y in 0..height {
		for x in 0..width {
			if y % (scale * 2) == 0  && x % scale == 0 {
				let pixel = img.get_pixel(x, y);
				let mut intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
				if pixel[3] == 0 { 
					intensity = 0;
				}
				final_art += get_str_ascii(intensity);
			} 
		}
		if y % (scale * 2) == 0 {
			final_art += "\n";
		}
	}
	return final_art;
}

fn get_str_ascii(intensity: u8) -> &'static str {
	let index = intensity / 32;
	return ASCII_CHAR_ARRAY[index as usize];
}