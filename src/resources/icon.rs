use egui::{ColorImage, Context, ImageData, TextureId, TextureOptions};
use tiny_skia::Pixmap;
use usvg::FitTo;

fn render_svg(svg: &[u8], [width, height]: [usize; 2]) -> anyhow::Result<Pixmap> {
    // let opts = usvg::Options {
    //     image_rendering: usvg::ImageRendering::OptimizeSpeed,
    //     ..Default::default()
    // };
    let opts = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg, &opts)?;

    let pixmap_size = usvg::Size::new(width as f64, height as f64)
        .unwrap()
        .to_screen_size();

    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(
        &tree,
        FitTo::Size(width as u32, height as u32),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .ok_or(anyhow::anyhow!("Failed to render svg"))?;

    Ok(pixmap)
}

pub struct Icon {
    pub texture: TextureId,
    pub width: usize,
    pub height: usize,
}

impl Icon {
    pub fn from_svg(bytes: &[u8], size: [usize; 2], alloc: &Context) -> anyhow::Result<Self> {
        // let svg_size = size.map(|x| x * 32);
        let pixmap = render_svg(bytes, size)?;
        let image = ColorImage::from_rgba_unmultiplied(size, pixmap.data());
        let id = alloc.tex_manager().write().alloc(
            "Test name".to_string(),
            ImageData::Color(image),
            TextureOptions::default(),
        );

        Ok(Self {
            width: size[0],
            height: size[1],
            texture: id,
        })
    }

    pub fn size_f32(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }

    pub fn id(&self) -> TextureId {
        self.texture
    }
}
