use crate::{
    collision::{
        contact_manifold::ContactManifold,
        separating_axis_theorem::{ check_polygon_manifold, check_triangles_manifold },
    },
    geometry::bullet_shape::{ get_bullet_world_shape, get_bullet_world_triangles },
    models::bullet::Bullet,
    resources::shape_library::ShapeLibrary,
};

pub fn detect_collision_manifolds(
    bullet_a: &Bullet,
    bullet_b: &Bullet,
    shape_library: &ShapeLibrary
) -> Vec<ContactManifold> {
    let (Some(shape_a), Some(shape_b)) = (
        shape_library.get(bullet_a.get_shape()),
        shape_library.get(bullet_b.get_shape()),
    ) else {
        return Vec::new();
    };

    if shape_a.is_convex() && shape_b.is_convex() {
        let (Some(polygon_a), Some(polygon_b)) = (
            get_bullet_world_shape(bullet_a, shape_library),
            get_bullet_world_shape(bullet_b, shape_library),
        ) else {
            return Vec::new();
        };

        return check_polygon_manifold(&polygon_a, &polygon_b).into_iter().collect();
    }

    let (Some(triangles_a), Some(triangles_b)) = (
        get_bullet_world_triangles(bullet_a, shape_library),
        get_bullet_world_triangles(bullet_b, shape_library),
    ) else {
        return Vec::new();
    };

    check_triangles_manifold(&triangles_a, &triangles_b).into_iter().collect()
}
