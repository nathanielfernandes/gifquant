use clap::Parser;
use gifquant::{
    encode_gif,
    gif::GlobalPalette,
    read::{read_dir, read_images},
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "gifquant")]
#[command(author = "Nathaniel F. <nathaniel.s.fernandes@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "gifquant cli", long_about = None)]
struct Args {
    #[clap(name = "images", help = "directory of images")]
    images: PathBuf,

    #[clap(
        short = 'p',
        long = "palette",
        help = ".gpl palette to use for quantization"
    )]
    palette: Option<PathBuf>,

    #[clap(
        short = 'o',
        long = "output",
        help = "output file path",
        default_value = "out.gif"
    )]
    output: PathBuf,

    #[clap(
        short = 'd',
        long = "delay",
        help = "delay between frames in milliseconds",
        default_value = "100"
    )]
    delay: u64,

    #[clap(
        short = 'l',
        long = "loop",
        help = "number of times to loop the gif",
        default_value = "0"
    )]
    loop_count: u16,

    #[clap(
        short = 'q',
        long = "speed",
        help = "quantization speed",
        default_value = "1"
    )]
    speed: i32,

    #[clap(
        short = 'c',
        long = "colors",
        help = "number of colors to quantize to",
        default_value = "256"
    )]
    colors: usize,

    #[clap(
        short = 'i',
        long = "interlaced",
        help = "whether to interlace the gif",
        default_value = "false"
    )]
    interlaced: bool,

    #[clap(
        short = 's',
        long = "show-palette",
        help = "whether to show the palette",
        default_value = "false"
    )]
    show_palette: bool,
}

fn main() {
    let start = std::time::Instant::now();
    let args = Args::parse();

    println!("Reading images...");
    let mut tick = std::time::Instant::now();
    let images = read_dir(&args.images);
    let images = read_images(&images).expect("Failed to read images");
    println!("\tRead {} images in {:?}\n", images.len(), tick.elapsed());

    if images.len() < 1 {
        panic!("Not enough images");
    }

    println!("Building palette...");
    tick = std::time::Instant::now();
    let gp = if let Some(palette) = args.palette {
        let palette = std::fs::read_to_string(palette).expect("Failed to read palette");
        let palette = gifquant::parse::parse_gpl(&palette);
        let gp: GlobalPalette = palette.into();
        gp
    } else {
        gifquant::gif::GlobalPalette::new(args.speed, args.colors, &images[0])
    };
    println!("\tBuilt palette in {:?}\n", tick.elapsed());

    // if args.show_palette {
    //     for image in images.iter_mut() {
    //         draw_palette(image, &gp);
    //     }
    // }

    tick = std::time::Instant::now();
    println!("Encoding gif...");
    let delay = std::time::Duration::from_millis(args.delay);
    let buf = match encode_gif(&images, delay, &gp) {
        Ok(buf) => buf,
        Err(e) => panic!("{}", e),
    };
    println!("\tEncoded gif in {:?}\n", tick.elapsed());

    std::fs::write(&args.output, buf).expect("Failed to write gif");
    println!("Wrote gif to {:?} in {:?}", args.output, start.elapsed());
}
