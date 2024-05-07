use std::time::Instant;

use image::{Rgba, RgbaImage};
use num_complex::{Complex64, ComplexFloat};
use rand::{thread_rng, Rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deb: Instant = Instant::now();
    let mut thread: rand::prelude::ThreadRng = thread_rng();
    let x0: f64 = -2.0;
    let x1: f64 = 2.0;
    let y0: f64 = -2.0;
    let y1: f64 = 2.0;
    let it_max: i32 = 10000;
    let mod_max: f64 = 20.0;
    let c_count: i32 = 10000000;
    let l: u32 = 1000;
    let h: u32 = 1000;
    let mut img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(l, h);
    let mut grille: Vec<Vec<f64>> = vec![vec![0.0; img.width() as usize]; img.height() as usize];
    for i in 0..c_count {
        if i % 100000 == 0 {
            println!("{i}");
        }

        let c: num_complex::Complex<f64> =
            Complex64::new(thread.gen_range(x0..=x1), thread.gen_range(y0..=y1));

        let mut path: Vec<(u32, u32)> = Vec::new();
        let mut z: num_complex::Complex<f64> = Complex64::new(0.0, 0.0);

        let mut n: i32 = 0;
        while n < it_max && z.abs() < mod_max {
            let px: u32 = ((z.re * l as f64) / (x1 - x0) as f64 + l as f64 / 2.0) as u32;
            let py: u32 = ((z.im * h as f64) / (y1 - y0) as f64 + h as f64 / 2.0) as u32;
            if px > 0 && py > 0 && px < l && py < h {
                path.push((px, py));
            }
            z = z * z + c.tan();
            let tmp_re = z.re;
            z.re = z.im;
            z.im = tmp_re;
            n += 1;
        }
        if z.abs() >= mod_max {
            path.iter().for_each(|p| {
                grille[(*p).1 as usize][(*p).0 as usize] += 1.0;
            });
        }
    }
    for it_y in 0..h {
        for it_x in 0..l {
            img.put_pixel(
                it_x,
                it_y,
                Rgba([grille[it_y as usize][it_x as usize] as u8, 0, 0, 255]),
            );
        }
    }
    img.save("image.png")?;
    println!("temps = {:?}", deb.elapsed());
    Ok(())
}
