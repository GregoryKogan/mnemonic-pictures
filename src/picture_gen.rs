use bracket_noise::prelude::{FastNoise, NoiseType};
use colorgrad::Gradient;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    fn new_gray(v: u8) -> Self {
        Pixel { r: v, g: v, b: v }
    }

    fn new_from_rgba_u8(channels: [u8; 4]) -> Self {
        Pixel {
            r: channels[0],
            g: channels[1],
            b: channels[2],
        }
    }

    fn brightness(&self) -> u8 {
        ((self.r as u16 + self.g as u16 + self.b as u16) / 3) as u8
    }
}

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        let pixels = vec![Pixel::new(0, 0, 0); (width * height) as usize];
        Image {
            width,
            height,
            pixels,
        }
    }

    fn inside(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    fn coordinates_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn put_pixel(&mut self, p: Pixel, x: u32, y: u32) {
        if !self.inside(x, y) {
            return;
        };
        let index = self.coordinates_to_index(x, y);
        self.pixels[index] = p;
    }

    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        if !self.inside(x, y) {
            {}
        };
        let index = self.coordinates_to_index(x, y);
        Some(self.pixels[index])
    }

    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; self.pixels.len() * 4];
        for i in 0..self.pixels.len() {
            bytes[i * 4 + 0] = self.pixels[i].r;
            bytes[i * 4 + 1] = self.pixels[i].g;
            bytes[i * 4 + 2] = self.pixels[i].b;
            bytes[i * 4 + 3] = 255;
        }
        bytes
    }
}

pub struct MnemonincGenerator {
    rng: ChaCha8Rng,
    context: CanvasRenderingContext2d,
    img: Image,
    path: Vec<u32>,
}

impl MnemonincGenerator {
    pub fn new(canvas_id: &str, seed: u64) -> Self {
        let (canvas, context) = MnemonincGenerator::get_canvas(canvas_id);
        let (width, height) = (canvas.width(), canvas.height());
        MnemonincGenerator {
            rng: ChaCha8Rng::seed_from_u64(seed + width as u64 + height as u64),
            context,
            img: Image::new(width, height),
            path: vec![0; (width * height) as usize],
        }
    }

    pub fn generate(&mut self) {
        self.noise_fill();
        self.walk();
        self.color_image();
        self.put_image();
    }

    fn noise_fill(&mut self) {
        let noise_seed = self.rng.gen::<u64>();
        let mut noise = FastNoise::seeded(noise_seed);
        noise.set_noise_type(NoiseType::Simplex);
        let freq = self.rng.gen_range(0.5..5.0);
        noise.set_frequency(freq);

        for x in 0..self.img.width {
            for y in 0..self.img.height {
                let noise_value = ((noise.get_noise(
                    x as f32 / self.img.width as f32,
                    y as f32 / self.img.height as f32,
                ) + 1.0)
                    / 2.0
                    * 255.0) as u8;
                self.img.put_pixel(Pixel::new_gray(noise_value), x, y);
            }
        }
    }

    fn walk(&mut self) {
        let mut cur_x = self.rng.gen_range(0..self.img.width);
        let mut cur_y = self.rng.gen_range(0..self.img.height);
        let mut visited: u32 = 1;
        self.path[self.img.coordinates_to_index(cur_x, cur_y)] = visited;

        let (mut nx, mut ny);
        while visited < self.img.width * self.img.height {
            (nx, ny) = self.next_step(cur_x, cur_y);
            if (nx, ny) == (cur_x, cur_y) {
                break;
            }
            (cur_x, cur_y) = (nx, ny);
            visited += 1;
            self.path[self.img.coordinates_to_index(cur_x, cur_y)] = visited;
        }
    }

    fn next_step(&self, x: u32, y: u32) -> (u32, u32) {
        let mut found = false;
        let mut window: i32 = 3;
        let x_signed = x as i32;
        let y_signed = y as i32;
        while !found {
            let half = window / 2;
            let mut candidates: Vec<(i32, i32)> = vec![];
            for i in (-half + 1)..half {
                candidates.push((x_signed + i, y_signed - half));
                candidates.push((x_signed + i, y_signed + half));
                candidates.push((x_signed - half, y_signed + i));
                candidates.push((x_signed + half, y_signed + i));
            }
            candidates.push((x_signed - half, y_signed - half));
            candidates.push((x_signed - half, y_signed + half));
            candidates.push((x_signed + half, y_signed - half));
            candidates.push((x_signed + half, y_signed + half));

            let mut min_brightness_diff = u8::MAX;
            let mut next_point = (u32::MAX, u32::MAX);
            for (nx, ny) in candidates {
                if nx >= 0
                    && ny >= 0
                    && self.img.inside(nx as u32, ny as u32)
                    && self.path[self.img.coordinates_to_index(nx as u32, ny as u32)] == 0
                {
                    found = true;
                    let brightness_diff = self.img.get_pixel(x, y).unwrap().brightness().abs_diff(
                        self.img
                            .get_pixel(nx as u32, ny as u32)
                            .unwrap()
                            .brightness(),
                    );
                    if brightness_diff < min_brightness_diff {
                        min_brightness_diff = brightness_diff;
                        next_point = (nx as u32, ny as u32);
                    }
                }
            }

            if found {
                return next_point;
            }
            window += 2;
        }

        (x, y)
    }

    fn get_random_gradient(&mut self) -> fn() -> Gradient {
        let gradients = [
            colorgrad::br_bg,
            colorgrad::pr_gn,
            colorgrad::pi_yg,
            colorgrad::pu_or,
            colorgrad::rd_bu,
            colorgrad::rd_gy,
            colorgrad::rd_yl_bu,
            colorgrad::rd_yl_gn,
            colorgrad::spectral,
            colorgrad::blues,
            colorgrad::greens,
            colorgrad::greys,
            colorgrad::oranges,
            colorgrad::purples,
            colorgrad::reds,
            colorgrad::viridis,
            colorgrad::inferno,
            colorgrad::magma,
            colorgrad::plasma,
            colorgrad::bu_gn,
            colorgrad::bu_pu,
            colorgrad::gn_bu,
            colorgrad::or_rd,
            colorgrad::pu_bu_gn,
            colorgrad::pu_bu,
            colorgrad::rd_pu,
            colorgrad::yl_gn_bu,
            colorgrad::yl_gn,
            colorgrad::yl_or_br,
            colorgrad::yl_or_rd,
        ];
        gradients[self.rng.gen_range(0..gradients.len())]
    }

    fn color_image(&mut self) {
        let gradient = self.get_random_gradient()();
        for x in 0..self.img.width {
            for y in 0..self.img.height {
                let ind = self.img.coordinates_to_index(x, y);
                let relative_pos =
                    self.path[ind] as f64 / (self.img.width * self.img.height) as f64;
                let color = gradient.at(relative_pos);
                self.img
                    .put_pixel(Pixel::new_from_rgba_u8(color.to_rgba8()), x, y);
            }
        }
    }

    fn get_canvas(
        canvas_id: &str,
    ) -> (
        web_sys::HtmlCanvasElement,
        web_sys::CanvasRenderingContext2d,
    ) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        (canvas, context)
    }

    fn put_image(&self) {
        let image_data_temp = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.img.bytes()),
            self.img.width,
            self.img.height,
        )
        .unwrap();
        self.context
            .put_image_data(&image_data_temp, 0.0, 0.0)
            .unwrap();
    }
}
