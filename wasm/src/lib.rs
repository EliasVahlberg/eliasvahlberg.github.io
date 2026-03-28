use wasm_bindgen::prelude::*;
use terrain_forge::{Grid, ops};

/// Generate a map and return it as ASCII art.
/// algorithm: "bsp", "cellular", "drunkard", "maze", "rooms", "dla", "voronoi"
/// width/height: map dimensions (capped at 80x30)
/// seed: RNG seed
#[wasm_bindgen]
pub fn generate(algorithm: &str, width: u32, height: u32, seed: u32) -> String {
    let w = (width as usize).min(80);
    let h = (height as usize).min(30);
    let mut grid = Grid::new(w, h);

    if ops::generate(algorithm, &mut grid, Some(seed as u64), None).is_err() {
        return format!("error: unknown algorithm '{}'\ntry: bsp, cellular, drunkard, maze, rooms", algorithm);
    }

    let mut out = String::with_capacity(w * (h + 1));
    for y in 0..h {
        for x in 0..w {
            let is_floor = grid.get(x as i32, y as i32)
                .map(|t| t.is_floor())
                .unwrap_or(false);
            out.push(if is_floor { '.' } else { '#' });
        }
        out.push('\n');
    }
    out
}
