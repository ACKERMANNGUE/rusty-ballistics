use std::collections::HashSet;

use crate::{
    geometry::{aabb::AABB, bullet_shape::get_bullet_world_aabb},
    models::bullet::Bullet,
    resources::shape_library::ShapeLibrary,
};

const GRID_CELL_SIZE: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
struct CellRange {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

pub fn build_candidate_pairs(
    bullets: &[Bullet],
    world_size: (f32, f32),
    shape_library: &ShapeLibrary,
) -> Vec<(usize, usize)> {
    let spatial_grid = build_spatial_grid(bullets, world_size, shape_library);

    let mut pairs = HashSet::new();

    for cell in &spatial_grid {
        for first_index in 0..cell.len() {
            for second_index in first_index + 1..cell.len() {
                let bullet_a = cell[first_index];
                let bullet_b = cell[second_index];

                let pair = if bullet_a < bullet_b {
                    (bullet_a, bullet_b)
                } else {
                    (bullet_b, bullet_a)
                };

                pairs.insert(pair);
            }
        }
    }

    let mut pairs: Vec<(usize, usize)> = pairs.into_iter().collect();
    pairs.sort_unstable();

    pairs
}

fn build_spatial_grid(
    bullets: &[Bullet],
    world_size: (f32, f32),
    shape_library: &ShapeLibrary,
) -> Vec<Vec<usize>> {
    let (grid_width, grid_height) = compute_grid_size(world_size);

    let mut grid = vec![Vec::new(); grid_width * grid_height];

    for (bullet_index, bullet) in bullets.iter().enumerate() {
        let Some(aabb) = get_bullet_world_aabb(bullet, shape_library) else {
            continue;
        };

        let cell_range = compute_aabb_cell_range(&aabb, world_size, grid_width, grid_height);

        for y in cell_range.min_y..=cell_range.max_y {
            for x in cell_range.min_x..=cell_range.max_x {
                let cell_index = y * grid_width + x;
                grid[cell_index].push(bullet_index);
            }
        }
    }

    grid
}

fn compute_grid_size(world_size: (f32, f32)) -> (usize, usize) {
    let grid_width = (world_size.0 / GRID_CELL_SIZE).ceil().max(1.0) as usize;
    let grid_height = (world_size.1 / GRID_CELL_SIZE).ceil().max(1.0) as usize;
    (grid_width, grid_height)
}

fn compute_aabb_cell_range(
    aabb: &AABB,
    world_size: (f32, f32),
    grid_width: usize,
    grid_height: usize,
) -> CellRange {
    let half_width = world_size.0 * 0.5;

    let half_height = world_size.1 * 0.5;

    let min = aabb.get_min();
    let max = aabb.get_max();

    let min_x = ((min.x + half_width) / GRID_CELL_SIZE).floor() as isize;
    let max_x = ((max.x + half_width) / GRID_CELL_SIZE).floor() as isize;
    let min_y = ((min.y + half_height) / GRID_CELL_SIZE).floor() as isize;
    let max_y = ((max.y + half_height) / GRID_CELL_SIZE).floor() as isize;

    CellRange {
        min_x: min_x.clamp(0, (grid_width as isize) - 1) as usize,
        max_x: max_x.clamp(0, (grid_width as isize) - 1) as usize,
        min_y: min_y.clamp(0, (grid_height as isize) - 1) as usize,
        max_y: max_y.clamp(0, (grid_height as isize) - 1) as usize,
    }
}
