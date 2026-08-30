# Rusty Ballistic

Rusty Ballistic is a 2D rigid-body and ballistic simulation written in Rust with Bevy.

The project started as a simple projectile simulation and has progressively evolved into a custom physics sandbox with:

- gravity;
- wind and turbulence;
- shape-dependent air resistance;
- arbitrary polygonal projectile shapes;
- convex and concave collision detection;
- SAT-based narrow phase;
- contact manifold generation through clipping;
- sequential impulse collision resolution;
- restitution;
- static and dynamic friction;
- angular velocity and torque;
- direct penetration correction;
- AABB spatial-grid broad phase;
- debug rendering;
- interactive projectile spawning;
- an interceptor-based defense system.

The collision detection and response systems are implemented manually rather than relying on an external physics engine.

---

# Current features

## Physics simulation

Each projectile stores:

- position;
- linear velocity;
- rotation;
- angular velocity;
- mass;
- moment of inertia;
- size;
- restitution;
- static friction;
- dynamic friction;
- shape;
- projectile kind.

The simulation currently runs at a fixed:

```text
144 Hz
```

with:

```text
dt = 1 / 144
```

The integration includes:

- gravity;
- linear shape-dependent drag;
- wind;
- turbulence;
- exponential angular damping.

Projectiles are removed once their world-space AABB leaves the simulation bounds.

---

# Projectile shapes

Projectile shapes are loaded from:

```text
src/assets/shapes/bullets.json
```

The current shape library contains 48 polygonal shapes, including both convex and concave polygons.

Examples include:

```text
square
triangle
diamond
pentagon
hexagon
octagon
star
arrow
plus
l_shape
u_shape
crescent
boomerang
heart
spaceship
9mm_fmj
45_acp
357_magnum
44_magnum
556_nato
762x39
762x51_nato
12_gauge_slug
50_bmg
20mm_autocannon
```

Shapes are defined in local coordinates as JSON arrays:

```json
{
  "square": [
    [-1.0, 1.0],
    [1.0, 1.0],
    [1.0, -1.0],
    [-1.0, -1.0]
  ]
}
```

During loading, every polygon is:

1. converted to `Vec2`;
2. recentered around its polygon centroid;
3. validated;
4. normalized to counter-clockwise winding;
5. tested for convexity;
6. triangulated using Ear Clipping;
7. analyzed for local area;
8. analyzed for its polygon inertia factor.

This means the projectile position corresponds to the polygon center of mass / rotation center.

---

# Concave polygons

Concave polygons are supported through Ear Clipping triangulation.

Each generated triangle stores:

```text
triangle vertex indices
+
boundary edge metadata
```

The boundary metadata distinguishes:

```text
real polygon boundary edges
```

from:

```text
internal triangulation diagonals
```

This is important during collision detection.

Internal triangulation edges may participate in SAT separation tests, but they are not allowed to become physical collision features such as reference or incident edges.

This prevents internal Ear Clipping diagonals from producing incorrect collision normals.

---

# World-space geometry

A projectile's local vertex is transformed into world space using:

```text
local vertex
    ↓
scale by projectile size
    ↓
rotate by projectile rotation
    ↓
translate by projectile position
```

The same transformation is used for:

- rendered meshes;
- collision polygons;
- world triangles;
- AABB computation.

---

# Mass and moment of inertia

Mass properties are derived from the selected polygon, projectile size, and density.

The UI does not directly configure mass.

Instead:

```text
shape area
+
size
+
density
↓
mass
↓
moment of inertia
```

The same mass-property implementation is shared between projectile creation and the UI.

See:

```text
src/geometry/mass_properties.rs
src/geometry/moment_of_inertia.rs
```

Detailed formulas are documented in:

```text
FORMULAS.md
```

---

# Air resistance

Air resistance depends on the projectile's orientation relative to its motion.

The system:

1. computes the velocity relative to the air;
2. determines the axis perpendicular to that velocity;
3. projects the world-space polygon onto that axis;
4. uses the projection width as a shape drag factor.

As a result, a long thin object experiences different drag depending on whether it travels:

```text
head-on
```

or:

```text
sideways
```

The current drag model is linear in air-relative velocity.

---

# Wind and turbulence

Wind contains:

```text
direction
speed
turbulence
active
```

The initial simulation uses:

```text
direction = (0.5, 1.0)
speed = 5.5
```

The wind direction vector is used directly by the current implementation.

Turbulence evolves as a bounded random walk.

Current limits:

```text
turbulence X: [-0.5, 0.5]
turbulence Y: [-0.5, 0.5]

maximum random change per physics step:
0.005
```

Wind can be enabled or disabled at runtime.

---

# Broad phase

Collision candidate generation uses a uniform spatial grid.

File:

```text
src/collision/broad_phase.rs
```

Current cell size:

```text
100 world units
```

For every projectile:

1. its world-space AABB is computed;
2. every grid cell overlapped by that AABB receives the projectile index;
3. all projectile pairs sharing a cell are generated;
4. duplicate pairs are removed with a `HashSet`;
5. the final pair list is sorted for deterministic iteration order.

This supports projectiles larger than one grid cell because projectiles are inserted into every cell covered by their AABB.

---

# Narrow phase

The narrow phase is centralized in:

```text
src/collision/narrow_phase.rs
```

It returns:

```text
Vec<ContactManifold>
```

for a pair of projectiles.

The narrow phase selects two different paths.

## Convex × convex

For two convex shapes:

```text
full polygon SAT
↓
reference face selection
↓
incident face selection
↓
clipping
↓
1–2 contact points
```

## Concave collisions

If either polygon is concave:

```text
Ear Clipping triangles
↓
triangle × triangle SAT
↓
boundary-aware contact generation
↓
raw triangle manifolds
↓
manifold merging
↓
maximum 2 contacts per contact region
```

---

# Separating Axis Theorem

SAT tests edge normals from both polygons.

For every candidate axis, both polygons are projected onto the axis.

If any projection intervals are separated:

```text
no collision
```

Otherwise, the smallest directional penetration is selected.

The final collision normal is oriented from projectile A toward projectile B.

For convex polygons, the polygon that supplied the minimum penetration axis becomes the reference polygon.

---

# Contact manifolds

A contact manifold contains:

```text
normal
penetration depth
contact points
```

The reference and incident edges are selected using outward-facing normals.

The incident segment is clipped against the side planes of the reference edge.

The final manifold normally contains:

```text
1 contact
```

for vertex/edge collisions, or:

```text
2 contacts
```

for face/face collisions.

---

# Concave manifold merging

Triangle decomposition can generate several nearby manifolds representing the same physical contact region.

Rusty Ballistic merges these manifolds when:

```text
normal alignment >= 0.999
```

and their contacts are spatially close.

Current parameters:

```text
NORMAL_MERGE_DOT_THRESHOLD = 0.999
CONTACT_MERGE_DISTANCE     = 0.01
CONTACT_DUPLICATE_EPSILON  = 0.001
```

After merging, if more than two contacts remain, only the two extreme contacts along the contact tangent are preserved.

This keeps the solver compact while preserving the width of the contact region.

---

# Sequential impulse solver

Collision response is implemented in:

```text
src/collision/solver.rs
```

The solver uses:

```text
8 sequential impulse iterations
```

For every contact constraint it solves:

1. normal impulse;
2. friction impulse.

The solver accounts for:

- inverse mass;
- inverse moment of inertia;
- contact lever arms;
- angular contact velocity;
- restitution;
- accumulated normal impulse;
- accumulated tangent impulse;
- static friction;
- dynamic friction.

---

# Restitution

The combined restitution coefficient is:

```text
min(restitution_a, restitution_b)
```

Restitution is only applied when the incoming normal velocity is faster than:

```text
1.0 world units / second
```

This prevents tiny resting-contact velocities from continuously producing small bounces.

---

# Friction

The combined static and dynamic friction coefficients use the geometric mean:

```text
sqrt(friction_a * friction_b)
```

The solver first attempts static friction.

If the required accumulated tangent impulse exceeds the static Coulomb limit, dynamic friction is used instead.

The friction impulse also affects angular velocity through the contact lever arm.

---

# Penetration correction

Velocity resolution and penetration correction are intentionally separate.

After sequential impulses, Rusty Ballistic performs direct positional correction.

Current values:

```text
penetration slop       = 0.01
correction percentage  = 0.8
```

This workflow was intentionally retained because it provides useful sandbox behavior when several objects are spawned overlapping at the same position: they are directly pushed apart rather than waiting for velocity bias to separate them over multiple frames.

Baumgarte velocity bias is currently not used.

---

# Angular motion

Contact impulses can modify angular velocity.

The velocity of a point on a rotating projectile is:

```text
linear velocity
+
angular velocity × contact radius
```

Angular velocity is damped exponentially every physics step.

Current damping:

```text
0.2 s^-1
```

Values below:

```text
0.001 rad/s
```

are snapped to zero.

The angular damping value can be modified from the UI.

---

# Defense system

Rusty Ballistic currently includes an experimental automatic defense / interceptor system.

It is a sandbox gameplay system built on top of the projectile simulation.

The defense system is positioned at:

```text
(0, 0)
```

and currently uses:

```text
protection radius             = 500
detection radius              = 1000

interceptor speed             = 1400
launch cooldown               = 0.25 s
maximum active interceptors   = 20

interception radius           = 40
maximum interceptor turn rate = 3 rad/s
```

The interceptor projectile currently uses:

```text
shape               = 357_magnum
size                = 15
density             = 1
restitution         = 0
static friction     = 0
dynamic friction    = 0
```

---

# Threat detection

Only regular projectiles are considered threats.

Interceptors are excluded.

A projectile becomes a threat only if:

1. it is alive;
2. it is a normal `BULLET`;
3. it is inside the defense detection radius;
4. it has non-zero velocity;
5. it is moving toward the defense area;
6. its predicted straight-line trajectory intersects the protected circle.

The prediction solves the intersection between:

```text
projectile trajectory
```

and:

```text
protected circle
```

The detected threats are sorted by estimated time to reach the protected area.

The most urgent unengaged threat is selected first.

Important limitation:

The current threat prediction assumes constant linear velocity and does not include:

- gravity;
- drag;
- wind;
- turbulence;
- future collisions.

The real projectile continues to use the complete physics simulation.

---

# Interceptor registry

Interceptors are tracked separately from their physical `Bullet` representation.

Each registry entry stores:

```text
interceptor bullet ID
target bullet ID
```

This keeps targeting logic outside of the core `Bullet` model.

A target already assigned to an interceptor is considered engaged and will not receive another interceptor.

Invalid registry entries are cleaned automatically when either projectile no longer exists.

---

# Interceptor guidance

Interceptors do not instantly snap toward their targets anymore.

The guidance system computes:

```text
current direction
desired direction toward target
signed angular error
maximum allowed turn this physics step
```

The angular correction is limited by:

```text
maximum turn rate * delta time
```

The resulting direction is calculated using a 2D rotation matrix.

This creates curved pursuit trajectories instead of instantaneous turns.

The current guidance is pure pursuit:

```text
interceptor → current target position
```

It does not yet calculate a lead/intercept point.

---

# Interception

An interceptor succeeds when the distance between it and its assigned target is:

```text
<= 40 world units
```

Both projectiles are then marked dead and removed.

Interceptors are currently excluded from the normal SAT / sequential impulse collision solver.

Their interaction with targets is handled exclusively by the defense interception-distance check.

---

# Projectile kinds

The current projectile kinds are:

```rust
BULLET
INTERCEPTOR
```

Regular bullets participate in normal rigid-body collision resolution.

Any collision pair containing an interceptor is currently skipped by the standard collision solver.

---

# Debug rendering

The simulation can visualize:

- world bounds;
- polygon hitboxes;
- Ear Clipping triangulation;
- contact points;
- contact normals;
- defense position;
- protection radius;
- detection radius;
- detected threats;
- predicted threat paths;
- interceptor-to-target links.

Pressing:

```text
H
```

toggles the debug visual group.

Wind visualization and projectile trails remain visible when the debug visuals are hidden.

---

# Trails

Each Bevy projectile entity stores a trail.

Current maximum:

```text
300 points
```

Trails use the projectile's color.

Interceptor trails therefore make their curved guidance paths easy to inspect.

---

# Camera

The camera supports:

- movement with WASD;
- movement with arrow keys;
- mouse-wheel zoom;
- clamping to world boundaries.

Current zoom range:

```text
0.2 .. 5.0
```

---

# UI

The simulation includes an egui control panel.

It displays:

## Simulation

- running / paused state;
- elapsed simulation time;
- FPS;
- physics update rate.

## World

- world width;
- world height;
- projectile count.

## Physics

- gravity;
- air resistance;
- angular damping;
- delta time;
- physics rate.

Angular damping can be changed interactively.

## Wind

- active state;
- speed;
- direction;
- turbulence direction.

## Bullet

A shape can be selected from the loaded shape library.

## Bullet Spawn Settings

The UI can modify:

```text
size
density
restitution
static friction
dynamic friction
```

It also displays the derived:

```text
local area
scaled area
mass
moment of inertia
```

Dynamic friction is constrained so that:

```text
dynamic friction <= static friction
```

---

# Controls

| Input | Action |
|---|---|
| Left mouse + drag | Launch a projectile |
| Right mouse click | Spawn 25 random projectiles at the cursor |
| `Space` | Pause / resume |
| `R` | Regenerate random projectiles |
| `C` | Clear projectiles |
| `Y` | Toggle wind |
| `H` | Toggle debug visuals |
| Mouse wheel | Zoom |
| `WASD` | Move camera |
| Arrow keys | Move camera |

The launcher uses a slingshot-style interaction:

```text
drag direction
↓
launch direction is reversed
```

The drag length controls launch speed.

The final launch velocity is clamped to:

```text
1000 world units / second
```

---

# Default simulation configuration

```text
World size                 5000 × 5000
Physics rate               144 Hz
Gravity                    9.81
Air resistance             0.001
Initial random bullets     10
Maximum launch velocity    1000
Trail points               300

Angular damping            0.2
Angular stop threshold     0.001

Solver iterations          8
Restitution threshold      1.0
```

Wind starts enabled with:

```text
direction = (0.5, 1.0)
speed     = 5.5
```

---

# Project architecture

```text
src/
├── assets/
│   └── shapes/
│       └── bullets.json
│
├── collision/
│   ├── broad_phase.rs
│   ├── contact_constraint.rs
│   ├── contact_manifold.rs
│   ├── narrow_phase.rs
│   ├── rigid_body_math.rs
│   ├── separating_axis_theorem.rs
│   └── solver.rs
│
├── components/
│   ├── bullet_entity.rs
│   └── bullet_trail.rs
│
├── defense/
│   ├── defense_system.rs
│   ├── interceptor.rs
│   └── threat_detection.rs
│
├── factories/
│   └── bullet_factory.rs
│
├── geometry/
│   ├── aabb.rs
│   ├── bullet_shape.rs
│   ├── mass_properties.rs
│   ├── moment_of_inertia.rs
│   ├── polygon.rs
│   ├── projection.rs
│   ├── shape.rs
│   ├── shape_triangle.rs
│   ├── vector.rs
│   └── world_triangle.rs
│
├── loaders/
│   └── shape_loader.rs
│
├── models/
│   ├── bullet.rs
│   ├── physics.rs
│   ├── wind.rs
│   └── world.rs
│
├── rendering/
│   ├── bullet_renderer.rs
│   ├── debug_renderer.rs
│   └── defense_renderer.rs
│
├── resources/
│   ├── bullet_spawn_settings.rs
│   ├── debug_visibility.rs
│   ├── selected_shape.rs
│   └── shape_library.rs
│
├── systems/
│   ├── bullet_launcher.rs
│   ├── camera_controller.rs
│   ├── defense.rs
│   ├── input.rs
│   ├── simulation.rs
│   ├── startup.rs
│   └── ui.rs
│
├── config.rs
└── main.rs
```

---

# Simulation pipeline

The main fixed-update pipeline is currently:

```text
update defense cooldown
        ↓
clean interceptor registry
        ↓
detect threats / launch interceptor
        ↓
update interceptor guidance
        ↓
update physical simulation
        ├── update turbulence
        ├── integrate projectile motion
        ├── remove out-of-bounds projectiles
        ├── spatial-grid broad phase
        ├── narrow phase
        └── sequential impulse solver
        ↓
resolve interceptor proximity interceptions
        ↓
despawn orphan Bevy entities
        ↓
record projectile trails
```

For normal projectile collisions:

```text
AABB broad phase
        ↓
candidate pair
        ↓
convex?
   ┌────┴────┐
  yes        no
   │          │
polygon SAT   triangle decomposition
   │          │
   │       boundary-aware SAT
   │          │
   └────┬─────┘
        ↓
contact manifold(s)
        ↓
contact constraints
        ↓
8 sequential impulse iterations
        ├── normal impulse
        └── friction impulse
        ↓
direct penetration correction
```

---

# Running the project

A Rust toolchain and Cargo are required.

From the project root:

```bash
cargo run
```

For an optimized build:

```bash
cargo run --release
```

Useful development commands:

```bash
cargo fmt
cargo check
cargo test
```

---

# Current limitations / future work

The current implementation is intentionally still evolving.

Major possible next steps include:

- warm starting / persistent contact caching;
- continuous collision detection to prevent tunneling;
- collision layers and masks instead of the current interceptor special case;
- cached world-space geometry per physics frame;
- more advanced position solving;
- static rigid bodies;
- sleeping bodies;
- stacks and persistent resting contacts;
- predictive interceptor lead targeting;
- ballistic threat prediction including gravity, drag, and wind;
- turn dynamics driven by acceleration / torque instead of directly setting interceptor velocity;
- interceptor lifetime / range limits;
- configurable ammunition;
- multiple defense systems;
- optimized broad-phase data structures;
- fewer temporary geometry allocations.

---

# Notes

Rusty Ballistic is primarily a learning and experimentation project.

The objective is not to reproduce a production physics engine, but to implement and understand the main systems manually:

```text
geometry
collision detection
contact generation
impulse resolution
angular dynamics
friction
spatial partitioning
ballistics
guidance
```

The implementation favors explicit algorithms and debuggability over hiding those systems behind a third-party physics library.