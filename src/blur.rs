use fastblur::gaussian_blur;
use image::{DynamicImage, GenericImageView};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::colors::mix_colors;

pub fn blur_image(image: DynamicImage, color: [u8; 3], sigma: f32,gui_scale: f32) -> DynamicImage {
    let resolution = resolution::current_resolution().unwrap();
    let res_w = (resolution.0 as f32 * 1.0) as u32;
    let res_h = (resolution.1 as f32 * 1.0) as u32;

    let w1 = image.width();
    println!("w1: {}", w1);
    println!("res_w: {}", res_w);

    let image = image.resize_to_fill(res_w, res_h, image::imageops::FilterType::Nearest);

    let mut data: Vec<[u8; 3]> = image
        .pixels()
        .map(|p| [p.2 .0[0], p.2 .0[1], p.2 .0[2]])
        .collect();

    gaussian_blur(
        &mut data,
        image.width() as usize,
        image.height() as usize,
        sigma,
    );

    let factor = 0.9;
    data.par_iter_mut()
        .for_each(|p| *p = mix_colors(*p, color, factor));

    let blurred_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_fn(res_w, res_h, |x, y| {
            let pixel = data[(y * res_w + x) as usize];
            image::Rgb([pixel[0], pixel[1], pixel[2]])
        });

    DynamicImage::ImageRgb8(blurred_image)
}
