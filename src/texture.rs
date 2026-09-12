use crate::color::Color;

pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Texture {
    pub fn load(path: &str) -> Texture {
        let image = image::open(path)
            .unwrap_or_else(|e| panic!("no se pudo abrir la textura {}: {}", path, e))
            .to_rgb8();

        let width = image.width() as usize;
        let height = image.height() as usize;

        let pixels = image
            .pixels()
            .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32)
            .collect();

        Texture {
            width,
            height,
            pixels,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        let x = ((u * self.width as f32) as usize).min(self.width - 1);
        let y = ((v * self.height as f32) as usize).min(self.height - 1);

        Color::from_hex(self.pixels[y * self.width + x])
    }
}
