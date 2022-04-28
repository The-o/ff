mod variations;

use rand::random;
use variations::*;

fn main() {
    const N:u32 = 10;
    let mut hist = [[0u32; 256]; 256];
    let mut max = 0;
    let (a,b,c,d,e,f): (f32, f32,f32,f32,f32,f32) = random();
    let (x, y): (f32, f32) = random();
    let mut p = (x-0.5, y-0.5);

    let f = |x:f32, y:f32| -> (f32, f32) {
        let (x,y) = (a * x + b * y + c, d * x + e * y + f);
        let (x0, y0) = v0(x, y);
        let (x1, y1) = v1(x, y);
        let (x2, y2) = v2(x, y);
        let (x3, y3) = v3(x, y);
        let (x4, y4) = v4(x, y);
        let (x5, y5) = v5(x, y);
        let (x6, y6) = v6(x, y);
        let (x7, y7) = v7(x, y);
        let (x8, y8) = v8(x, y);
        let (x9, y9) = v9(x, y);
        let (x10, y10) = v10(x, y);
        let (x11, y11) = v11(x, y);
        (
            // 0.1 * x0 +
            // 0.1 * x1 +
            0.3 * x2 +
            0.3 * x3 +
            0.3 * x4
            // 0.1 * x5 +
            // 0.1 * x6 +
            // 0.1 * x7 +
            // 0.1 * x8 +
            // 0.1 * x9 +
            // 0.1 * x10 +
            // 0.1 * x11
,
            // 0.1 * y0
            // 0.1 * y1 +
            0.3 * y2 +
            0.3 * y3 +
            0.3 * y4
            // 0.1 * y5 +
            // 0.1 * y6 +
            // 0.1 * y7 +
            // 0.1 * y8 +
            // 0.1 * y9 +
            // 0.1 * y10 +
            // 0.1 * y11
        )
    };
    for _ in 0..N {
        p = f(p.0, p.1);
        // println!("{:?}", p);
        let (x, y) = ((128. + 128. * p.0) as i32, (128. + 128. * p.1) as i32);
        if x >= 0 && x < 256 && y >= 0 && y < 256 {
            let (x, y) = (x as usize, y as usize);
            hist[x][y] += 1;
            if max < hist[x][y] {
                max = hist[x][y];
            }
        }
    }
println!("{}", max);
    let mut image = raster::Image::blank(256, 256);
    for x in 0..256 {
        for y in 0..256 {
            let clr = raster::Color::rgba((hist[x as usize][y as usize] as f32 / max as f32 * 256.) as u8, 0, 0, 255);
            image.set_pixel(x, y, clr).expect("Error setting color");
        }
    }
    raster::save(&image, "img.png").expect("Error saving image");
}
