extern crate image;
use std::path::Path;

pub fn run(){
    use image::{RgbImage, Rgb};

let mut img = RgbImage::new(30, 30);
for x in 0..img.dimensions().0{
    for y in 0..img.dimensions().1{
        img.put_pixel(x, y, Rgb([15,15,1]));
        img.put_pixel(29-x, y, Rgb([100,255,0]));
    }
}

// for x in 15..=17 {
//     for y in 8..24 {
//         img.put_pixel(x, y, Rgb([255, 0, 0]));
//         img.put_pixel(y, x, Rgb([255, 0, 0]));
//     }
// }
img.save("test.png");
}
