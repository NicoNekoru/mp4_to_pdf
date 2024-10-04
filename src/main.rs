use std::env::{self, current_dir};
use std::fs::{DirEntry, File};

use image::{self, GenericImageView};

use std::fs::{ReadDir, read_dir, create_dir_all};
use std::process::Command;

fn main() {
	let mp4_filename =
		env::args()
			.nth(1)
			.expect("No video file provided");
	println!("Converting file `{}` to pdf", &mp4_filename);
	
	let pages_per_second: u32 =
		env::args()
			.nth(2)
			.unwrap_or("0".to_string())
			.parse()
			.unwrap_or(0);

	let mut frame_arg = "".to_string();

	if pages_per_second != 0 {
		frame_arg = format!("-r {}/1", pages_per_second);
	}

	println!("Openning directory");
	create_dir_all("./mp4_to_pdf").expect("Could not create dir");

	println!("Running ffmpeg");
	let _output = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", &format!("ffmpeg -i {0} {1} ./mp4_to_pdf/{0}%03d.bmp -y", &mp4_filename, &frame_arg)])
			.output()
			.expect("Ffmpeg failed to execute")
	} else {
		Command::new("sh")
			.arg("-c")
			.arg(&format!("ffmpeg -i {0} {1} ./mp4_to_pdf/{0}03d.bmp -y", &mp4_filename, &frame_arg))
			.output()
			.expect("Ffmpeg failed to execute")
	};

	println!("Parsing ffmpeg output");
	let dir: ReadDir = read_dir("./mp4_to_pdf").expect("Could not read dir");
	let mut frames: Vec<DirEntry> = 
		dir
			.into_iter()
			.map(|f| f.expect("Error reading frame dump"))
			// .filter(|f| 
			// 	f	.file_name()
			// 		.into_string()
			// 		.unwrap()
			// 		.contains(&mp4_filename))
			.collect();

	frames.sort_by_key(|f| f.file_name());

	println!("Reading frames");
	println!("{:?}", &frames);
	println!("{:?}", &current_dir());
	println!("{:?}", read_dir("./").unwrap());
    let paths = std::fs::read_dir("./mp4_to_pdf").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().file_name().into_string().unwrap())
    }

	let first_frame_bmp = 
		image::ImageReader::open(frames[0].path())
			.expect("Cannot read frame dump")
			.decode()
			.expect("Could not decode frame dump");

	// 1 inch = 25.4 mm
	// printer at 300 DPI | 1 px = 1 dot
	let dimensions = first_frame_bmp.dimensions();
	let height = dimensions.0 as f32 / 300.0 * 25.4;
	let width = dimensions.1 as f32 / 300.0 * 25.4;

	let mut first_frame_file = File::open(frames[0].path()).expect("Could not read frame dump");
	let first_frame_bmp = printpdf::image_crate::codecs::bmp::BmpDecoder::new(&mut first_frame_file).expect("Could not load bmp");
	let bmp_embed = printpdf::Image::try_from(first_frame_bmp).expect("Could not read bmp");

	let (doc, page1, layer1) = 
		printpdf::PdfDocument::new(
			&mp4_filename,
			printpdf::Mm(height), 
			printpdf::Mm(width), 
			"Page 1" );

	bmp_embed.add_to_layer(
		doc.get_page(page1).get_layer(layer1), 
		printpdf::ImageTransform::default() );

	println!("Adding frames to pdf");	
	for i in 1..frames.len() {
		let (curr_page, curr_layer) = doc.add_page(printpdf::Mm(height), printpdf::Mm(width), format!("Page {}", i + 1));

		let mut curr_frame_file = File::open(frames[i].path()).expect("Could not read frame dump");
		let curr_frame_bmp = printpdf::image_crate::codecs::bmp::BmpDecoder::new(&mut curr_frame_file).expect("Could not load bmp");
		let bmp_embed = printpdf::Image::try_from(curr_frame_bmp).expect("Could not read bmp");

		bmp_embed.add_to_layer(
			doc.get_page(curr_page).get_layer(curr_layer), 
			printpdf::ImageTransform::default() );
	}

	doc.save(&mut std::io::BufWriter::new(
		File::create(
			format!("{}.pdf", &mp4_filename)
		).expect("Could not create pdf")
	)).expect("Could not write to pdf");
}
