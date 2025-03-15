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


fn extract_mica_colors(img_path: &str) -> anyhow::Result<MicaColors> {
    let img = image::open(img_path)?.into_rgb8();
    let pixels: Vec<[f32; 3]> = img.pixels()
        .map(|p| [p.0[0] as f32, p.0[1] as f32, p.0[2] as f32])
        .collect();

    let kmeans = get_kmeans(1, 3, 0.5, false, &pixels, seed)
    let sorted = kmeans.sorted(Sort::Weight);

    Ok(MicaColors {
        primary: sorted.centroids[0].map(|v| v as u8),
        secondary: sorted.centroids[1].map(|v| v as u8),
    })
}