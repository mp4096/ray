use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

fn write_ppm(width: usize, height: usize, pixels: &[Pixel]) -> std::io::Result<()> {
    let file = File::create("out.ppm")?;
    let mut buf_writer = BufWriter::new(file);
    let header = format!("P6 {} {} 255 ", width, height);
    buf_writer.write_all(header.as_bytes())?;
    for p in pixels {
        buf_writer.write_all(&[p.red, p.green, p.blue])?;
    }
    Ok(())
}

fn main() {
    let width = 3000;
    let height = 3000;
    println!("Writing a {}x{} image", width, height);
    let mut vec: Vec<Pixel> = Vec::with_capacity(width * height);
    for j in (0..width).rev() {
        for i in 0..height {
            let r = (i as f64) / ((width - 1) as f64);
            let g = (j as f64) / ((height - 1) as f64);
            let b = 0.25;

            let px = Pixel {
                red: (255.999 * r) as u8,
                green: (255.999 * g) as u8,
                blue: (255.999 * b) as u8,
            };

            vec.push(px);
        }
    }
    match write_ppm(width, height, &vec) {
        Ok(_) => println!("Ok!"),
        Err(_) => println!("nok..."),
    }
}
