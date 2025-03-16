use fake_mica::blur;

fn main() {
    let wallpaper = wallpaper::get().unwrap();
    let image = image::open(wallpaper).unwrap();
    let blurred_image = blur::blur_image(image, [255, 255, 255], 200.0);
    blurred_image
        .save("test_imgs/blurred_wallpaper.png")
        .unwrap();
}
