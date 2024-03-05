// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let arg = args.remove(0).parse::<f32>().unwrap();
            let infile = args.remove(0);
            let outfile = args.remove(0);

            blur(arg, infile, outfile);
        }

        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let arg = args.remove(0).parse::<i32>().unwrap();
            let infile = args.remove(0);
            let outfile = args.remove(0);

            brighten(arg, infile, outfile);
        }

        "crop" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let arg = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);

            crop(arg, infile, outfile);
        }

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let arg = args.remove(0).parse::<i32>().unwrap();
            let infile = args.remove(0);
            let outfile = args.remove(0);

            rotate(arg, infile, outfile);
        }

        "invert" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);

            invert(infile);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);

            grayscale(infile, outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        "generate" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let arg = args.remove(0);
            let outfile = args.remove(0);
            generate(arg, outfile);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");

    println!("brighten argument INFILE OUTFILE");
    println!("crop argument INFILE OUTFILE");
    println!("rotate argument INFILE OUTFILE");
    println!("invert INFILE");
    println!("grayscale INFILE OUTFILE");
    println!("generate argument OUTFILE");

    std::process::exit(-1);
}

fn blur(arg: f32, infile: String, outfile: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(arg);

    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(arg: i32, infile: String, outfile: String) {
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(arg);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(arg: String, infile: String, outfile: String) {
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    let mut img = image::open(infile).expect("Failed to open INFILE.");

    let x = arg.split("x").nth(0).unwrap().parse::<u32>().unwrap();
    let y = arg.split("x").nth(1).unwrap().parse::<u32>().unwrap();
    let width = arg.split("x").nth(2).unwrap().parse::<u32>().unwrap();
    let height = arg.split("x").nth(3).unwrap().parse::<u32>().unwrap();

    let img2 = img.crop(x, y, width, height);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(arg: i32, infile: String, outfile: String) {
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = match arg {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => panic!("Invalid rotation amount"),
    };

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String) {
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    let mut img = image::open(&infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(infile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    // .grayscale() takes no arguments. It returns a new image.

    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(arg: String, outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let x = arg.split("x").nth(0).unwrap().parse::<i32>().unwrap() as u8;
    let y = arg.split("x").nth(1).unwrap().parse::<i32>().unwrap() as u8;
    let z = arg.split("x").nth(2).unwrap().parse::<i32>().unwrap() as u8;

    println!("{} {} {}", x, y, z);

    let color = image::Rgb([x, y, z]);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color;
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
