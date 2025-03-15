use image::imageops::fast_blur;
use image::{DynamicImage, GenericImageView, RgbImage};
use palette::cast::{from_component_slice, into_component_slice};
use palette::{FromColor, IntoColor, Lab, Srgb};
use kmeans_colors::{get_kmeans, Calculate, Kmeans, MapColor, Sort};
use noise::{NoiseFn, Perlin, Seedable, utils::*};

fn main() {
    // 1. 读取图片并提取主色
    let img = image::open("input_image.png").expect("Failed to open image");
    let img = img.to_rgb8();
    let img_vec = img.as_raw();

    // 转换到 Lab 颜色空间进行 k-means 聚类
    let lab: Vec<Lab> = from_component_slice::<Srgb<u8>>(img_vec)
        .iter()
        .map(|x| x.into_format().into_color())
        .collect();

    // k-means 参数
    let k = 1;
    let max_iter = 20;
    let converge = 1.0;
    let runs = 3;
    let seed = 42;

    let mut result = Kmeans::new();
    for i in 0..runs {
        let run_result = get_kmeans(k, max_iter, converge, false, &lab, seed + i as u64);
        if run_result.score < result.score {
            result = run_result;
        }
    }

    // 获取主色
    let dominant_color = result.centroids[0];
    let dominant_rgb = Srgb::from_color(dominant_color).into_format::<u8>();

    // 2. 生成柏林噪声图
    let perlin = Perlin::new(seed as u32);// 固定种子保证可重复性
    let (width, height) = img.dimensions();
    let mut noise_map = vec![0.0; (width * height) as usize];

    // 生成噪声数据
    let frequency = 0.06; // 控制噪声纹理密度
    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * frequency;
            let ny = y as f64 * frequency;
            let noise_value = perlin.get([nx, ny, 0.0]);
            noise_map[(y * width + x) as usize] = (noise_value + 1.0) / 2.0; // 归一化到 [0,1]
        }
    }

    // 3. 生成双色噪点图
    let mut output_img = RgbImage::new(width, height);
    let threshold = 0.9; // 控制颜色分布比例

    for y in 0..height {
        for x in 0..width {
            let noise_val = noise_map[(y * width + x) as usize];
            let color = if noise_val > threshold {
                dominant_rgb
            } else {
                Srgb::new(255, 255, 255) // 白色
            };
            output_img.put_pixel(x, y, image::Rgb([color.red, color.green, color.blue]));
        }
    }

    // 4. 添加模糊效果
    let blurred_img = fast_blur(&output_img, 30.0);

    // 保存结果
    blurred_img.save("output_perlin.png").expect("Failed to save image");
}