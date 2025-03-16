use fastblur::gaussian_blur;
use image::{DynamicImage, GenericImageView};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::colors::mix_colors;

pub fn blur_image(image: DynamicImage, color: [u8; 3], sigma: f32) -> DynamicImage {
    let image = image.resize_exact(1920, 1080, image::imageops::FilterType::Nearest);

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
        image::ImageBuffer::from_fn(1920, 1080, |x, y| {
            let pixel = data[y as usize * 1920 + x as usize];
            image::Rgb([pixel[0], pixel[1], pixel[2]])
        });

    DynamicImage::ImageRgb8(blurred_image)
}
