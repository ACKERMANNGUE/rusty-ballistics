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
    let physics = Physics::new(DELTA_TIME, AIR_RESISTANCE, GRAVITY);
    let mut world = World::new(WORLD_SIZE, physics);

    let bullet_1 = Bullet::new(
        String::from("B1"),
        Vec2::new(1.2, 2.7),
        Vec2::new(100.0, 50.0),
        0.009,
    );

    let bullet_2 = Bullet::new(
        String::from("B2"),
        Vec2::new(1.2, 0.3),
        Vec2::new(15.0, 275.0),
        0.009,
    );

    world.add_bullet(bullet_1);
    world.add_bullet(bullet_2);
    world.display_in_term();
    world.update();
}

fn display_world_in_term(world: &World) {
    world.display_in_term();
    world.display_bullets_in_term();
}