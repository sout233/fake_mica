use fastblur::gaussian_blur;
use image::{DynamicImage, GenericImageView};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::colors::mix_colors;

pub fn blur_image(image: DynamicImage, color: [u8; 3], sigma: f32) -> DynamicImage {
    let resolution = resolution::current_resolution().unwrap();
    let res_w = resolution.0 as u32;
    let res_h = resolution.1 as u32;

    let image = image.resize_exact(res_w, res_h, image::imageops::FilterType::Nearest);

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

    // 让图像变白
    let factor = 0.7; // 取值范围 0.0 (无变化) - 1.0 (完全白)
    data.par_iter_mut()
        .for_each(|p| *p = mix_colors(*p, color, factor));

    let blurred_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_fn(res_w, res_h, |x, y| {
            let pixel = data[(y * res_w + x) as usize];
            image::Rgb([pixel[0], pixel[1], pixel[2]])
        });

    DynamicImage::ImageRgb8(blurred_image)
}
