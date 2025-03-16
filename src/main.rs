use fastblur::gaussian_blur;
use image::{imageops::fast_blur, GenericImageView};
use palette::IntoColor;

fn brighten_color(color: [u8; 3], factor: f32) -> [u8; 3] {
    let brighten = |c: u8| ((c as f32) * (1.0 - factor) + 255.0 * factor) as u8;
    [brighten(color[0]), brighten(color[1]), brighten(color[2])]
}

fn darken_color(color: [u8; 3], factor: f32) -> [u8; 3] {
    let darken = |c: u8| ((c as f32) * (1.0 - factor) + 0.0 * factor) as u8;
    [darken(color[0]), darken(color[1]), darken(color[2])]
}

fn main() {
    let wallpaper = wallpaper::get().unwrap();
    let image = image::open(wallpaper).unwrap();
    let image = image.resize_exact(1920, 1080, image::imageops::FilterType::Nearest);
    image.save("original_wallpaper.png").unwrap();
    // let image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image.to_rgb8();
    let mut data: Vec<[u8; 3]> = image
        .pixels()
        .map(|p| [p.2 .0[0], p.2 .0[1], p.2 .0[2]])
        .collect();
    gaussian_blur(&mut data, image.width() as usize, image.height() as usize, 100.0);
    
    // 让图像变白
    let factor = 0.7; // 取值范围 0.0 (无变化) - 1.0 (完全白)
    data.iter_mut()
        .for_each(|p| *p = darken_color(*p, factor));

    let blurred_image = image::ImageBuffer::from_fn(1920, 1080, |x, y| {
        let pixel = data[y as usize * 1920 + x as usize];
        image::Rgb([pixel[0], pixel[1], pixel[2]])
    });
    blurred_image.save("blurred_wallpaper.png").unwrap();
}
