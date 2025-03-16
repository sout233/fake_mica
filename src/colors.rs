use kmeans_colors::{get_kmeans, Sort};

struct MicaColors {
    primary: (u8, u8, u8),
    secondary: (u8, u8, u8),
}

impl MicaColors {
    pub fn new(primary: (u8, u8, u8), secondary: (u8, u8, u8)) -> Self {
        Self { primary, secondary }
    }
}


// fn extract_mica_colors(img_path: &str) -> anyhow::Result<MicaColors> {
//     let img = image::open(img_path)?.into_rgb8();
//     let pixels: Vec<[f32; 3]> = img.pixels()
//         .map(|p| [p.0[0] as f32, p.0[1] as f32, p.0[2] as f32])
//         .collect();

//     let kmeans = get_kmeans(1, 3, 0.5, false, &pixels, 42);
//     let sorted = kmeans.sorted(Sort::Weight);

//     Ok(MicaColors {
//         primary: sorted.centroids[0].map(|v| v as u8),
//         secondary: sorted.centroids[1].map(|v| v as u8),
//     })
// }

pub fn brighten_color(color: [u8; 3], factor: f32) -> [u8; 3] {
    let brighten = |c: u8| ((c as f32) * (1.0 - factor) + 255.0 * factor) as u8;
    [brighten(color[0]), brighten(color[1]), brighten(color[2])]
}

pub fn darken_color(color: [u8; 3], factor: f32) -> [u8; 3] {
    let darken = |c: u8| ((c as f32) * (1.0 - factor) + 0.0 * factor) as u8;
    [darken(color[0]), darken(color[1]), darken(color[2])]
}

pub fn mix_colors(color1: [u8; 3], color2: [u8; 3], factor: f32) -> [u8; 3] {
    let overlay = |base: u8, blend: u8| {
        let base_f = base as f32 / 255.0;
        let blend_f = blend as f32 / 255.0;
        let result = if base_f < 0.5 {
            2.0 * base_f * blend_f
        } else {
            1.0 - 2.0 * (1.0 - base_f) * (1.0 - blend_f)
        };
        (result * 255.0) as u8
    };

    let normal_blend = |base: u8, blend: u8| {
        ((base as f32) * 0.3 + (blend as f32) * 0.7) as u8 // 70% 透明度的叠加
    };

    let overlay_result = [
        overlay(color1[0], color2[0]),
        overlay(color1[1], color2[1]),
        overlay(color1[2], color2[2]),
    ];

    [
        normal_blend(overlay_result[0], color2[0]),
        normal_blend(overlay_result[1], color2[1]),
        normal_blend(overlay_result[2], color2[2]),
    ]
}
