use bracket_noise::prelude::{FastNoise, NoiseType};
use colorgrad::Gradient;
use image::{ImageBuffer, Rgba, Pixel};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, ImageData};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;


pub fn get_canvas(
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

pub fn put_image_to_canvas(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, context: CanvasRenderingContext2d) {
    let clamped_buf: Clamped<&[u8]> = Clamped(img.as_raw());
    let image_data_temp =
        ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, img.width(), img.height())
            .unwrap();
    context.put_image_data(&image_data_temp, 0.0, 0.0).unwrap();
}

fn fill_noise(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, seed: u64) {
    let mut noise = FastNoise::seeded(seed);
    noise.set_noise_type(NoiseType::Simplex);
    noise.set_frequency(2.0);

    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let noise_value =
                ((noise.get_noise(x as f32 / width as f32, y as f32 / height as f32) + 1.0) / 2.0
                    * 255.0) as u8;
            img.put_pixel(x, y, Rgba([noise_value, noise_value, noise_value, 255]));
        }
    }
}

fn coordinates_to_index(x: u32, y: u32, width: u32) -> usize {
    (y * width + x) as usize
}

fn next_step(x: u32, y: u32, path: &Vec<u32>, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> (u32, u32) {
    let (width, height) = img.dimensions();
    let mut next_point: (u32, u32) = (0, 0);
    let mut min_color_dist = u8::MAX;
    let mut found = false;

    let mut window: u32 = 3;
    while !found {
        let half = window / 2;
        let mut candidates: Vec<(i32, i32)> = vec![];
        for offset in (-(half as i32) / 2)..(half + 1) as i32 {
            candidates.push((x as i32 + offset, (y - half) as i32));
            candidates.push((x as i32 + offset, (y + half) as i32));
            candidates.push(((x - half) as i32, y as i32 + offset));
            candidates.push(((x + half) as i32, y as i32 + offset));
        }

        for (nx, ny) in candidates {
            let n_ind = coordinates_to_index(nx as u32, ny as u32, width);
            if 0 <= nx && (nx as u32) < width && 0 <= ny && (ny as u32) < height && path[n_ind] == 0 {
                let color_dist = (img.get_pixel(x, y).channels()[0] as i16 - img.get_pixel(nx as u32, ny as u32).channels()[0] as i16).abs() as u8;
                if color_dist < min_color_dist {
                    min_color_dist = color_dist;
                    next_point = (nx as u32, ny as u32);
                    found = true;
                }
            }
        }

        window += 2;
    }

    next_point
}

fn walk(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, rng: &mut ChaCha8Rng) -> Vec<u32> {
    let (width, height) = img.dimensions();
    
    let mut path = vec![0; (width * height) as usize];
    let mut cur_x = rng.gen_range(0..width);
    let mut cur_y = rng.gen_range(0..height);
    let mut visited: u32 = 1;
    path[coordinates_to_index(cur_x, cur_y, width)] = visited;

    while visited < width * height - 1 {
        (cur_x, cur_y) = next_step(cur_x, cur_y, &path, img);
        visited += 1;
        path[coordinates_to_index(cur_x, cur_y, width)] = visited;
    }

    path
}

fn get_random_gradient(rng: &mut ChaCha8Rng) -> fn() -> Gradient {
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
    gradients[rng.gen_range(0..gradients.len())]
}

fn color_image(path: &Vec<u32>, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, rng: &mut ChaCha8Rng) {
    let (width, height) = img.dimensions();
    let gradient = get_random_gradient(rng)();
    for x in 0..width {
        for y in 0..height {
            let ind = coordinates_to_index(x, y, width);
            let relative_pos = path[ind] as f64 / (width * height) as f64;
            let color = gradient.at(relative_pos);
            img.put_pixel(x, y, Rgba(color.to_rgba8()));
        }
    }
}

pub fn create(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    fill_noise(img, seed);
    let path = walk(img, &mut rng);
    color_image(&path, img, &mut rng);
}
