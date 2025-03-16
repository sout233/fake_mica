use std::{io::Cursor, sync::Arc};

use image::{DynamicImage, GenericImageView, ImageFormat};
use vizia::{
    prelude::*,
    vg::{self, Data, Point},
};

fn main() -> Result<(), ApplicationError> {
    let wallpaper_path = wallpaper::get().unwrap();
    let wallpaper = image::open(&wallpaper_path).unwrap();
    let blurred_wallpaper = fake_mica::blur::blur_image(wallpaper.clone(), [255, 255, 255], 200.0);
    let wallpaper = Arc::new(blurred_wallpaper); // 共享壁纸数据

    let initial_rect = (0, 0, 800, 450);
    let initial_bg = crop_wallpaper(&wallpaper, initial_rect);

    Application::new(move |cx| {
        AppData {
            bg_data: initial_bg,
            wallpaper: wallpaper.clone(),
            last_window_position: (0, 0),
        }
        .build(cx);

        // WindowWatcher::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Oiiaioiiiai")
                .width(Stretch(1.0))
                .height(Auto);
            MicaCard::new(cx, AppData::bg_data)
                .width(Stretch(1.0))
                .height(Stretch(1.0));
        });

        let timer = cx.add_timer(Duration::from_millis(100), None, |cx, _| {
            // println!("Timer ticked");
            cx.emit(AppEvent::CheckWindowPosition);
        });

        cx.start_timer(timer);
    })
    .title("Fake Mica")
    .inner_size((800, 450))
    .run()
}

struct MicaCard<L> {
    bg_data: L,
}

impl<L: Lens<Target = Vec<u8>>> MicaCard<L> {
    pub fn new(cx: &mut Context, bg_data: L) -> Handle<Self> {
        Self { bg_data }
            .build(cx, |cx| {
                Label::new(cx, "This is a custom view!").alignment(Alignment::BottomRight);
            })
            .bind(bg_data, |mut handle, _| handle.needs_redraw())
    }
}

impl<L: Lens<Target = Vec<u8>>> View for MicaCard<L> {
    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let data = self.bg_data.get(cx);
        let bounds = cx.bounds();
        let rect: vg::Rect = bounds.into();
        let mut path = vg::Path::new();
        path.add_rect(rect, None);
        let paint = vg::Paint::default();

        let data = Data::new_copy(&data);
        if let Some(image) = vg::Image::from_encoded(&data) {
            canvas.draw_image(&image, Point::new(rect.left(), rect.top()), Some(&paint));
        }
    }
}

#[derive(Lens)]
struct AppData {
    bg_data: Vec<u8>,
    wallpaper: Arc<DynamicImage>,
    last_window_position: (i32, i32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::UpdateBackground(position) => {
                let (x, y) = *position;
                let window_size = cx.window().unwrap().inner_size();
                self.bg_data = crop_wallpaper(
                    &self.wallpaper,
                    (x, y, window_size.width, window_size.height),
                );
                cx.needs_redraw();
            }
            AppEvent::CheckWindowPosition => {
                let window = cx.window();
                if window.is_some() {
                    let window = window.unwrap();
                    let position = window.outer_position().unwrap();
                    let pos: (i32, i32) = (position.x, position.y);
                    if pos != self.last_window_position {
                        println!("Window position changed: ({}, {})", pos.0, pos.1);
                        self.last_window_position = pos;
                        cx.emit(AppEvent::UpdateBackground(pos));
                    }
                }
            }
        })
    }
}

pub enum AppEvent {
    UpdateBackground((i32, i32)),
    CheckWindowPosition,
}

/// 截取壁纸的相应区域
fn crop_wallpaper(image: &DynamicImage, rect: (i32, i32, u32, u32)) -> Vec<u8> {
    println!("Cropping wallpaper: ({}, {}, {}, {})", rect.0, rect.1, rect.2, rect.3);
    let (x, y, width, height) = rect;

    let img_width = image.width();
    let img_height = image.height();

    // 确保裁剪范围不超出壁纸
    let crop_x = x.max(0) as u32;
    let crop_y = y.max(0) as u32;
    let crop_width = width.min(img_width.saturating_sub(crop_x)); // 避免 x + width > image.width()
    let crop_height = height.min(img_height.saturating_sub(crop_y)); // 避免 y + height > image.height()

    let cropped = image
        .view(crop_x, crop_y, crop_width, crop_height)
        .to_image();
    let mut buf = Vec::new();
    DynamicImage::ImageRgba8(cropped)
        .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .unwrap();

    println!("Cropped data size: {}", buf.len());
    buf
}
