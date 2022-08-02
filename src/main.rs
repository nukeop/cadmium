use cadmium::{init::read_cl_args, palette::Palette};
use image::{GenericImageView, Pixel};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn process_image(input_path: &str, palette: &Palette, output_path: &str) {
    let input_image = image::open(input_path).unwrap();
    let mut output_image = image::ImageBuffer::new(input_image.width(), input_image.height());

    for x in 0..input_image.width() {
        for y in 0..input_image.height() {
            let color = input_image.get_pixel(x, y);
            let closest_color = palette.get_closest_color(&color.to_rgb());
            output_image.put_pixel(x, y, closest_color);
        }
    }

    output_image.save(output_path).unwrap();
}

fn main() {
    pretty_env_logger::init();
    info!("Cadmium starting up...");

    let args = read_cl_args();
    let input = args.input.unwrap();
    let output: String = args.output.unwrap();

    let palette_file = args.palette.unwrap();
    let palette = Palette::from_file(palette_file.as_str());

    process_image(&input, &palette, &output);
}
