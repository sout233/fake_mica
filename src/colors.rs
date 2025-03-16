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
    let mix = |c1: u8, c2: u8| ((c1 as f32) * (1.0 - factor) + (c2 as f32) * factor) as u8;
    [mix(color1[0], color2[0]), mix(color1[1], color2[1]), mix(color1[2], color2[2])]
}