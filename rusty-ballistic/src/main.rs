mod models;

use glam::Vec2;
use models::bullet::Bullet;
use models::world::World;
use models::physics::Physics;

const GRAVITY: f32 = 9.81;
const AIR_RESISTANCE: f32 = 0.1;
const WORLD_SIZE: (f32, f32) = (800.0, 600.0);
const HZ: f32 = 60.0;
const DELTA_TIME: f32 = 1.0 / HZ;

fn main() {
    let mut world = World::new(GRAVITY, WORLD_SIZE, AIR_RESISTANCE);

    let bullet = Bullet::new(
        String::from("Test bullet"),
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 50.0),
        0.009,
    );

    world.add_bullet(bullet);
    display_world_in_term(&world);

    let mut physics = Physics::new(world, DELTA_TIME);
    display_n_steps_in_term(&mut physics, 5);
}

fn display_world_in_term(world: &World) {
    world.display_in_term();
    world.display_bullets_in_term();
}

fn display_n_steps_in_term(physics: &mut models::physics::Physics, n: usize) {
    for step in 0..n {
        println!("\nStep [{}]", step + 1);
        physics.update();
        physics.get_world().display_bullets_in_term();
    }
}