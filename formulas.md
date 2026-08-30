# Rusty Ballistic — Physics and Geometry Formulas

This document describes the mathematical formulas currently implemented by Rusty Ballistic.

The formulas correspond to the current source code rather than to an idealized physics model.

---

# Notation

| Symbol | Meaning |
|---|---|
| `p` | position |
| `v` | linear velocity |
| `a` | linear acceleration |
| `ω` | angular velocity |
| `θ` | rotation angle |
| `m` | mass |
| `m⁻¹` | inverse mass |
| `I` | moment of inertia |
| `I⁻¹` | inverse moment of inertia |
| `dt` | fixed simulation timestep |
| `n` | collision normal |
| `t` | collision tangent |
| `r` | vector from body center to contact point |
| `J` | impulse |
| `e` | restitution coefficient |
| `μs` | static friction coefficient |
| `μd` | dynamic friction coefficient |
| `ρ` | density |
| `A` | polygon area |
| `s` | projectile size scale |
| `×` | 2D scalar cross product |
| `·` | dot product |
| `||v||` | vector magnitude |

The simulation currently uses:

```text
HZ = 144
dt = 1 / HZ
```

Therefore:

```text
dt ≈ 0.00694444 s
```

---

# 1. 2D vector cross product

File:

```text
src/geometry/vector.rs
```

For two 2D vectors:

```text
a = (ax, ay)
b = (bx, by)
```

the scalar cross product is:

```text
a × b = ax * by - ay * bx
```

It is used in:

- polygon geometry;
- moment of inertia;
- contact effective mass;
- angular impulse calculations.

---

# 2. Perpendicular vector

For:

```text
v = (x, y)
```

Rusty Ballistic uses:

```text
perpendicular(v) = (-y, x)
```

This is used to obtain:

- polygon edge normals;
- contact tangents;
- the projected-width axis for drag.

---

# 3. Polygon signed area

File:

```text
src/geometry/polygon.rs
```

For polygon vertices:

```text
p0, p1, ..., pn-1
```

with:

```text
pn = p0
```

the signed area is:

```text
A_signed =
1/2 * Σ (xi * y(i+1) - x(i+1) * yi)
```

Equivalently using the 2D cross product:

```text
A_signed =
1/2 * Σ (pi × p(i+1))
```

The absolute polygon area is:

```text
A = |A_signed|
```

If:

```text
A_signed < 0
```

the vertex order is reversed so the stored polygon uses counter-clockwise winding.

---

# 4. Polygon centroid

Before a shape is stored, its vertices are recentered around the polygon centroid.

For every polygon edge:

```text
cross_i = pi × p(i+1)
```

then:

```text
A_signed =
1/2 * Σ cross_i
```

and:

```text
C =
[Σ (pi + p(i+1)) * cross_i]
/
[6 * A_signed]
```

Each loaded vertex is then transformed to:

```text
p_local = p_original - C
```

As a consequence, the projectile position is used as its polygon center / rotation origin.

---

# 5. Local-to-world vertex transformation

File:

```text
src/geometry/bullet_shape.rs
```

Given a centered local vertex:

```text
p_local = (x, y)
```

and projectile size:

```text
s
```

first scale:

```text
p_scaled = s * p_local
```

For projectile rotation:

```text
θ
```

the 2D rotation is:

```text
x_rot = x_scaled * cos(θ) - y_scaled * sin(θ)

y_rot = x_scaled * sin(θ) + y_scaled * cos(θ)
```

Finally:

```text
p_world =
p_projectile
+
p_rotated
```

---

# 6. Scaled polygon area

File:

```text
src/geometry/mass_properties.rs
```

If the local polygon area is:

```text
A_local
```

and the polygon is uniformly scaled by:

```text
s
```

then:

```text
A_scaled =
A_local * s²
```

---

# 7. Mass

Mass is derived from density and scaled area:

```text
m =
ρ * A_scaled
```

Therefore:

```text
m =
ρ * A_local * s²
```

Mass is not directly configured in the spawn UI.

It changes automatically when either:

```text
density
```

or:

```text
size
```

changes.

---

# 8. Polygon inertia factor

File:

```text
src/geometry/moment_of_inertia.rs
```

The polygon vertices have already been centered around their centroid.

For each edge:

```text
vi → vi+1
```

define:

```text
cross_i =
vi × vi+1
```

and:

```text
q_i =
||vi||²
+
vi · vi+1
+
||vi+1||²
```

The implementation accumulates:

```text
cross_sum =
Σ cross_i
```

and:

```text
inertia_sum =
Σ cross_i * q_i
```

The shape inertia factor is:

```text
k =
| inertia_sum / (6 * cross_sum) |
```

The factor depends only on the local polygon geometry.

---

# 9. Projectile moment of inertia

The final moment of inertia is:

```text
I =
m * s² * k
```

where:

```text
m = projectile mass
s = projectile size
k = local polygon inertia factor
```

Because:

```text
m ∝ s²
```

the total moment of inertia scales approximately with:

```text
s⁴
```

for constant density and shape.

---

# 10. Wind velocity

File:

```text
src/models/physics.rs
```

The current implementation computes:

```text
v_wind =
wind_direction * wind_speed
```

The wind direction is currently used directly and is not normalized by this formula.

The initial values are:

```text
wind_direction = (0.5, 1.0)
wind_speed     = 5.5
```

---

# 11. Turbulence

File:

```text
src/models/wind.rs
```

Each physics update generates:

```text
Δtx ∈ [-TURBULENCE_DELTA_MAX, +TURBULENCE_DELTA_MAX]
Δty ∈ [-TURBULENCE_DELTA_MAX, +TURBULENCE_DELTA_MAX]
```

with:

```text
TURBULENCE_DELTA_MAX = 0.005
```

Then:

```text
tx_new = clamp(
    tx_old + Δtx,
    -TURBULENCE_MAX_X,
    +TURBULENCE_MAX_X
)

ty_new = clamp(
    ty_old + Δty,
    -TURBULENCE_MAX_Y,
    +TURBULENCE_MAX_Y
)
```

Current bounds:

```text
TURBULENCE_MAX_X = 0.5
TURBULENCE_MAX_Y = 0.5
```

This creates a bounded random walk.

---

# 12. Air-relative velocity

When wind is active:

```text
v_air =
v_projectile
-
v_wind
-
v_turbulence
```

Therefore:

```text
v_air =
v_projectile
-
(wind_direction * wind_speed)
-
v_turbulence
```

When wind is inactive:

```text
v_air =
v_projectile
```

---

# 13. Shape-dependent projected width

To estimate how much of the polygon is exposed to the air, the normalized air-relative direction is:

```text
d =
v_air / ||v_air||
```

A perpendicular projection axis is:

```text
a =
(-dy, dx)
```

Every world-space polygon vertex is projected using:

```text
projection_i =
a · vertex_i
```

Then:

```text
minimum =
min(projection_i)

maximum =
max(projection_i)
```

The projected width is:

```text
W =
maximum - minimum
```

This means drag changes with projectile orientation.

---

# 14. Shape drag factor

The implementation currently uses:

```text
reference_width = 1
```

Therefore:

```text
shape_drag_factor =
projected_width / reference_width
```

which currently simplifies to:

```text
shape_drag_factor =
projected_width
```

---

# 15. Drag force

The current implementation uses a linear drag model:

```text
F_drag =
-k_air
*
shape_drag_factor
*
v_air
```

where:

```text
k_air = AIR_RESISTANCE
```

Current value:

```text
AIR_RESISTANCE = 0.001
```

This is not the classical quadratic aerodynamic formula:

```text
1/2 ρ C_d A v²
```

Rusty Ballistic currently uses a simpler linear velocity-dependent model.

---

# 16. Drag acceleration

Using:

```text
F = m * a
```

the drag acceleration is:

```text
a_drag =
F_drag / m
```

Therefore:

```text
a_drag =
[
-k_air
*
shape_drag_factor
*
v_air
]
/
m
```

---

# 17. Gravity

Gravity acceleration is:

```text
a_gravity =
(0, -g)
```

with:

```text
g = 9.81
```

The total linear acceleration is:

```text
a =
a_gravity
+
a_drag
```

---

# 18. Linear integration

The current implementation first updates velocity:

```text
v_new =
v_old
+
a * dt
```

and then updates position using the newly computed velocity:

```text
p_new =
p_old
+
v_new * dt
```

This corresponds to semi-implicit / symplectic Euler integration for the linear state.

---

# 19. Angular damping

File:

```text
src/models/physics.rs
```

Angular velocity uses exponential damping.

The damping factor is:

```text
D =
exp(
    -angular_damping * dt
)
```

Then:

```text
ω_new =
ω_old * D
```

Current default:

```text
angular_damping = 0.2 s⁻¹
```

If:

```text
|ω_new| < 0.001
```

then:

```text
ω_new = 0
```

Rotation is integrated as:

```text
θ_new =
θ_old
+
ω_new * dt
```

---

# 20. AABB computation

For all world-space polygon vertices:

```text
pi = (xi, yi)
```

the AABB is:

```text
min_x = min(xi)
max_x = max(xi)

min_y = min(yi)
max_y = max(yi)
```

Therefore:

```text
AABB.min =
(min_x, min_y)

AABB.max =
(max_x, max_y)
```

A projectile is considered outside the world if its AABB is outside the configured world bounds.

---

# 21. Spatial grid dimensions

File:

```text
src/collision/broad_phase.rs
```

Current cell size:

```text
CELL_SIZE = 100
```

For world width:

```text
W_world
```

and height:

```text
H_world
```

the grid dimensions are:

```text
grid_width =
ceil(W_world / CELL_SIZE)

grid_height =
ceil(H_world / CELL_SIZE)
```

with a minimum dimension of one cell.

---

# 22. AABB-to-grid conversion

The world is centered around `(0, 0)`.

For:

```text
half_width = W_world / 2
half_height = H_world / 2
```

an AABB coordinate is converted to a cell coordinate using:

```text
cell_x =
floor(
    (world_x + half_width)
    / CELL_SIZE
)

cell_y =
floor(
    (world_y + half_height)
    / CELL_SIZE
)
```

The result is clamped to valid grid indices.

Every cell covered by the AABB receives the projectile index.

---

# 23. Polygon projection for SAT

For a normalized SAT axis:

```text
n
```

and polygon vertex:

```text
pi
```

the scalar projection is:

```text
si =
n · pi
```

The interval is:

```text
[min(si), max(si)]
```

---

# 24. SAT interval penetration

Given intervals:

```text
[min1, max1]
[min2, max2]
```

the implementation computes directional penetrations:

```text
p12 =
max1 - min2

p21 =
max2 - min1
```

If:

```text
p12 <= 0
```

or:

```text
p21 <= 0
```

there is no collision on that axis.

Otherwise:

```text
penetration =
min(p12, p21)
```

This directional form also handles containment cases correctly.

---

# 25. SAT collision condition

For every edge normal from both polygons:

```text
project polygon A
project polygon B
```

If one separating axis exists:

```text
collision = false
```

If no separating axis exists:

```text
collision = true
```

The collision penetration depth is the smallest penetration over all tested axes:

```text
penetration_depth =
min(all axis penetrations)
```

---

# 26. Collision normal orientation

After selecting the minimum-overlap axis:

```text
n
```

the polygon centroids are computed.

Define:

```text
d =
center_2 - center_1
```

If:

```text
d · n < 0
```

the axis is reversed:

```text
n = -n
```

The resulting collision normal therefore points from polygon 1 toward polygon 2.

---

# 27. Outward edge normal

For polygon edge:

```text
e =
p_next - p_current
```

an initial perpendicular normal is:

```text
n =
normalize(
    (-ey, ex)
)
```

The edge midpoint is:

```text
m =
(p_current + p_next) / 2
```

and:

```text
d =
m - polygon_centroid
```

If:

```text
n · d < 0
```

the normal is reversed:

```text
n = -n
```

This produces an outward-facing edge normal.

---

# 28. Reference edge selection

The reference edge is the polygon edge whose outward normal is most aligned with the desired reference normal.

For every edge:

```text
score_i =
outward_normal_i · reference_normal
```

Select:

```text
reference_edge =
argmax(score_i)
```

---

# 29. Incident edge selection

The incident edge is the edge whose outward normal is most opposite to the reference normal.

For every allowed incident edge:

```text
score_i =
outward_normal_i · reference_normal
```

Select:

```text
incident_edge =
argmin(score_i)
```

For concave triangle collisions, internal Ear Clipping diagonals are excluded from incident-edge selection.

---

# 30. Segment clipping

File:

```text
src/collision/contact_manifold.rs
```

For a clipping plane:

```text
normal = n
offset = o
```

the signed distance of point `p` is:

```text
d =
p · n - o
```

A point is kept if:

```text
d <= 0
```

If the two segment endpoints lie on opposite sides:

```text
d0 * d1 < 0
```

the intersection interpolation factor is:

```text
α =
d0 / (d0 - d1)
```

and the intersection point is:

```text
p_intersection =
p0
+
α * (p1 - p0)
```

The incident edge is clipped against both side planes of the reference edge.

---

# 31. Contact position adjustment

For a clipped contact point:

```text
p
```

the separation against the reference plane is:

```text
separation =
reference_normal
·
(p - reference_start)
```

A contact is accepted when:

```text
separation <= EPSILON
```

The current implementation shifts the contact halfway between the overlapping surfaces:

```text
contact =
p
-
reference_normal
*
separation
*
0.5
```

---

# 32. Concave collision triangulation

Concave polygons are triangulated using Ear Clipping.

For vertices:

```text
previous
current
next
```

a candidate vertex is convex when:

```text
(current - previous)
×
(next - current)
>
0
```

A triangle is an ear if:

```text
the vertex is convex
```

and:

```text
no other remaining polygon vertex lies inside the candidate triangle
```

The ear is removed and the process repeats until one triangle remains.

---

# 33. Point-in-triangle test

For triangle:

```text
A, B, C
```

and point:

```text
P
```

the implementation computes:

```text
s1 =
(B - A) × (P - A)

s2 =
(C - B) × (P - B)

s3 =
(A - C) × (P - C)
```

The point is considered inside when:

```text
s1 >= -EPSILON
s2 >= -EPSILON
s3 >= -EPSILON
```

for the normalized counter-clockwise polygon winding.

---

# 34. Concave triangle boundary filtering

Each Ear Clipping triangle has three edge flags.

For original polygon indices:

```text
i
j
```

the edge is considered part of the original polygon boundary when the vertices are adjacent cyclically.

Conceptually:

```text
j = i + 1
```

or:

```text
i = j + 1
```

with wraparound.

Internal triangulation diagonals are marked:

```text
boundary = false
```

During triangle SAT:

- all triangle edges still participate in separation testing;
- only boundary edges can become the reference collision feature;
- only boundary edges can become the incident collision feature.

---

# 35. Concave manifold normal merge condition

File:

```text
src/collision/narrow_phase.rs
```

Two candidate manifolds must first have sufficiently aligned normals.

The alignment is:

```text
alignment =
n_group · n_candidate
```

They pass the normal test when:

```text
alignment >= 0.999
```

---

# 36. Concave manifold contact merge condition

After normal alignment, at least one pair of contacts must satisfy:

```text
distance(contact_a, contact_b)
<=
0.01
```

The implementation performs the equivalent squared-distance comparison.

---

# 37. Duplicate contact removal

Contacts are considered duplicates when:

```text
distance(contact_a, contact_b)
<=
0.001
```

Again, the implementation uses squared distance.

---

# 38. Contact reduction

If a merged contact group contains more than two contact points, define the tangent:

```text
t =
perpendicular(n)
```

Each contact is projected onto the tangent:

```text
projection_i =
contact_i · t
```

The solver keeps:

```text
contact with minimum projection
contact with maximum projection
```

This reduces each merged contact region to at most two contacts.

---

# 39. Merged penetration depth

When compatible manifolds are merged:

```text
penetration_group =
max(
    penetration_group,
    penetration_candidate
)
```

The deepest penetration is retained.

---

# 40. Contact lever arms

For contact point:

```text
c
```

and body positions:

```text
p1
p2
```

the contact lever arms are:

```text
r1 =
c - p1

r2 =
c - p2
```

---

# 41. Angular contact velocity

For angular velocity:

```text
ω
```

and contact radius:

```text
r = (rx, ry)
```

the 2D equivalent of:

```text
ω × r
```

is:

```text
(-ω * ry, ω * rx)
```

Therefore the contact-point velocity is:

```text
v_contact =
v_linear
+
ω × r
```

---

# 42. Relative contact velocity

For the two bodies:

```text
v_contact_1
v_contact_2
```

the relative contact velocity is:

```text
v_relative =
v_contact_2
-
v_contact_1
```

---

# 43. Normal contact velocity

For collision normal:

```text
n
```

the normal relative velocity is:

```text
v_n =
v_relative · n
```

---

# 44. Tangent

The contact tangent is:

```text
t =
(-ny, nx)
```

The tangential relative velocity is:

```text
v_t =
v_relative · t
```

---

# 45. Effective mass

For a constraint axis:

```text
a
```

which may be either:

```text
normal
```

or:

```text
tangent
```

define:

```text
r1a =
r1 × a

r2a =
r2 × a
```

The impulse denominator is:

```text
K =
m1⁻¹
+
m2⁻¹
+
(r1 × a)² * I1⁻¹
+
(r2 × a)² * I2⁻¹
```

The effective mass is:

```text
M_eff =
1 / K
```

If:

```text
|K| <= EPSILON
```

the implementation returns:

```text
M_eff = 0
```

---

# 46. Restitution coefficient combination

The combined coefficient is:

```text
e =
min(e1, e2)
```

---

# 47. Restitution velocity threshold

Restitution is only generated if:

```text
v_n
<
-RESTITUTION_VELOCITY_THRESHOLD
```

Current threshold:

```text
1.0
```

Then:

```text
v_restitution =
-e * v_n
```

Otherwise:

```text
v_restitution = 0
```

---

# 48. Sequential normal impulse

For the current normal velocity:

```text
v_n
```

and precomputed restitution target:

```text
v_restitution
```

the desired incremental impulse is:

```text
ΔJ_n =
(
    v_restitution
    -
    v_n
)
*
M_normal
```

The solver uses accumulated impulses.

If:

```text
J_n_old
```

is the previous accumulated normal impulse:

```text
J_n_new =
max(
    J_n_old + ΔJ_n,
    0
)
```

Only the difference is applied:

```text
J_n_applied =
J_n_new
-
J_n_old
```

The vector impulse is:

```text
J =
n * J_n_applied
```

---

# 49. Applying the normal impulse to linear velocity

For body 1:

```text
v1_new =
v1
-
J * m1⁻¹
```

For body 2:

```text
v2_new =
v2
+
J * m2⁻¹
```

---

# 50. Applying the normal impulse to angular velocity

For body 1:

```text
ω1_new =
ω1
-
(r1 × J)
*
I1⁻¹
```

For body 2:

```text
ω2_new =
ω2
+
(r2 × J)
*
I2⁻¹
```

---

# 51. Friction coefficient combination

Static friction:

```text
μs =
sqrt(
    μs1 * μs2
)
```

Dynamic friction:

```text
μd =
sqrt(
    μd1 * μd2
)
```

---

# 52. Unconstrained tangent impulse

The raw tangent impulse change is:

```text
ΔJ_t =
-v_t
*
M_tangent
```

The candidate accumulated impulse is:

```text
J_t_candidate =
J_t_old
+
ΔJ_t
```

---

# 53. Static friction limit

The maximum accumulated static friction impulse is:

```text
J_t_static_max =
μs * J_n
```

If:

```text
|J_t_candidate|
<=
J_t_static_max
```

then:

```text
J_t_new =
J_t_candidate
```

---

# 54. Dynamic friction limit

If the static limit is exceeded:

```text
J_t_dynamic_max =
μd * J_n
```

Then:

```text
J_t_new =
clamp(
    J_t_candidate,
    -J_t_dynamic_max,
    +J_t_dynamic_max
)
```

The applied tangent impulse is:

```text
J_t_applied =
J_t_new - J_t_old
```

and:

```text
J_friction =
t * J_t_applied
```

It is applied to both linear and angular velocities using the same impulse equations as the normal impulse.

---

# 55. Sequential solver iterations

For every collision pair:

```text
build contact constraints
```

then repeat:

```text
normal constraint
friction constraint
```

for every contact.

Current iteration count:

```text
SOLVER_ITERATIONS = 8
```

Conceptually:

```text
for iteration in 0..8:
    for contact:
        solve normal
        solve friction
```

Accumulated impulses are preserved across these iterations.

They are currently not preserved across simulation frames.

---

# 56. Direct penetration correction

After velocity constraints are solved, each contact manifold receives positional correction.

Current constants:

```text
PENETRATION_SLOP = 0.01

CORRECTION_PERCENTAGE = 0.8
```

Let:

```text
d =
penetration_depth
```

The corrected penetration error is:

```text
error =
max(
    d - PENETRATION_SLOP,
    0
)
```

Let:

```text
S =
m1⁻¹ + m2⁻¹
```

Then:

```text
correction_magnitude =
(error / S)
*
CORRECTION_PERCENTAGE
```

The correction vector is:

```text
C =
n * correction_magnitude
```

Positions are changed according to inverse mass:

```text
p1_new =
p1
-
C * m1⁻¹
```

```text
p2_new =
p2
+
C * m2⁻¹
```

This is direct positional correction.

The current solver does not use Baumgarte velocity bias.

---

# 57. Launcher drag velocity

File:

```text
src/systems/bullet_launcher.rs
```

The drag vector is:

```text
d =
drag_end
-
drag_start
```

The launcher resource computes:

```text
v_drag =
d * velocity_scale
```

Current:

```text
velocity_scale = 2
```

The result is clamped to:

```text
MAX_BULLET_VELOCITY = 1000
```

The input system launches in the opposite direction:

```text
v_launch =
-v_drag
```

Therefore the interaction behaves like a slingshot.

---

# 58. Random projectile velocity

For each component:

```text
rx, ry ∈ [0, 1)
```

the random velocity is:

```text
vx =
rx * 2 * Vmax - Vmax

vy =
ry * 2 * Vmax - Vmax
```

Current:

```text
Vmax = 1000
```

Therefore:

```text
vx ∈ [-1000, 1000)

vy ∈ [-1000, 1000)
```

---

# 59. Defense detection distance

File:

```text
src/defense/threat_detection.rs
```

Let:

```text
p_b =
bullet position

p_d =
defense position
```

The relative position is:

```text
r =
p_b - p_d
```

The distance to the defense is:

```text
distance =
||r||
```

A bullet is considered for threat prediction only when:

```text
distance
<=
detection_radius
```

Current detection radius:

```text
1000
```

---

# 60. Threat approach test

The current implementation checks:

```text
r · v < EPSILON
```

where:

```text
r =
bullet_position - defense_position
```

and:

```text
v =
bullet velocity
```

For a projectile moving toward the defense, `r` and `v` generally point in opposing directions, resulting in a negative dot product.

---

# 61. Protected-area trajectory equation

Threat prediction currently assumes constant velocity:

```text
p(t) =
r + v * t
```

The protected area is a circle centered on the defense with radius:

```text
R
```

The trajectory reaches the protected boundary when:

```text
||r + v*t||² =
R²
```

Expanding:

```text
(v · v)t²
+
2(r · v)t
+
(r · r - R²)
=
0
```

Therefore:

```text
a =
v · v

b =
2(r · v)

c =
r · r - R²
```

---

# 62. Threat discriminant

The quadratic discriminant is:

```text
D =
b² - 4ac
```

If:

```text
D < 0
```

the current straight-line trajectory does not intersect the protected circle.

The bullet is therefore not classified as a threat.

---

# 63. Estimated time to protected area

If:

```text
D >= 0
```

the two roots are:

```text
t1 =
(-b - sqrt(D))
/
(2a)
```

```text
t2 =
(-b + sqrt(D))
/
(2a)
```

The implementation selects the first non-negative root.

If the projectile is already inside the protected radius:

```text
time_to_protected_area = 0
```

Threats are sorted in ascending order by:

```text
time_to_protected_area
```

---

# 64. Threat prediction limitation

The previous equations assume:

```text
v(t) = constant
```

Therefore threat prediction currently ignores future:

```text
gravity
drag
wind
turbulence
collisions
```

This is only the prediction model.

The actual projectile remains simulated by the full physics model.

---

# 65. Interceptor initial velocity

File:

```text
src/systems/defense.rs
```

When an interceptor is launched:

```text
d =
target_position
-
defense_position
```

The initial interceptor velocity is:

```text
v_interceptor =
normalize(d)
*
interceptor_speed
```

Current speed:

```text
1400
```

---

# 66. Defense launch conditions

An interceptor may be launched when:

```text
defense enabled
```

and:

```text
cooldown_remaining <= EPSILON
```

and:

```text
active_interceptor_count
<
maximum_active_interceptors
```

Current values:

```text
launch cooldown             = 0.25 s
maximum active interceptors = 20
```

An already engaged target is skipped.

---

# 67. Defense cooldown

Every fixed update:

```text
cooldown_new =
max(
    cooldown_old - dt,
    0
)
```

After a launch:

```text
cooldown =
launch_cooldown
```

---

# 68. Interceptor desired direction

For interceptor position:

```text
p_i
```

and current target position:

```text
p_t
```

the desired direction is:

```text
d_desired =
normalize(
    p_t - p_i
)
```

The current guidance therefore uses pure pursuit rather than a predicted lead point.

---

# 69. Interceptor current direction

Given current interceptor velocity:

```text
v_i
```

if:

```text
||v_i||² > EPSILON
```

then:

```text
d_current =
normalize(v_i)
```

Otherwise:

```text
d_current =
d_desired
```

---

# 70. Signed angular error

The scalar 2D cross term is:

```text
cross =
d_current × d_desired
```

and:

```text
dot =
d_current · d_desired
```

The signed angular error is:

```text
θ_error =
atan2(
    cross,
    dot
)
```

This gives the shortest signed angular difference between the two directions.

---

# 71. Maximum interceptor turn angle

The interceptor has a maximum angular turn rate:

```text
ω_max
```

Current value:

```text
ω_max = 3 rad/s
```

For one physics timestep:

```text
θ_max =
ω_max * dt
```

At:

```text
144 Hz
```

this is approximately:

```text
θ_max
≈
3 / 144
≈
0.020833 rad
≈
1.194°
```

per fixed update.

---

# 72. Clamped interceptor turn

The actual turn performed is:

```text
θ_turn =
clamp(
    θ_error,
    -θ_max,
    +θ_max
)
```

Therefore the interceptor can no longer instantaneously rotate to face its target.

---

# 73. Interceptor 2D direction rotation

For current direction:

```text
d_current =
(x, y)
```

and turn angle:

```text
θ
```

the new direction is:

```text
x' =
x*cos(θ)
-
y*sin(θ)
```

```text
y' =
x*sin(θ)
+
y*cos(θ)
```

Therefore:

```text
d_new =
normalize(
    (x', y')
)
```

The new commanded interceptor velocity is:

```text
v_interceptor =
d_new
*
interceptor_speed
```

The projectile rotation is updated to:

```text
rotation =
atan2(
    velocity_y,
    velocity_x
)
```

and interceptor angular velocity is reset to:

```text
0
```

---

# 74. Approximate interceptor turning radius

For a body traveling at approximately constant speed:

```text
v
```

with maximum turn rate:

```text
ω
```

an approximate kinematic turning radius is:

```text
R_turn ≈ v / ω
```

Using the current configuration:

```text
v = 1400
ω = 3
```

gives:

```text
R_turn
≈
466.67 world units
```

This is an interpretation of the configured guidance limits rather than an explicit calculation performed by the code.

---

# 75. Interception condition

After physics integration, the distance between an interceptor and its assigned target is:

```text
d =
||p_interceptor - p_target||
```

The interception succeeds if:

```text
d
<=
DEFENSE_INTERCEPTION_RADIUS
```

Current radius:

```text
40
```

Both projectiles are then marked dead and removed.

---

# 76. Interceptor collision behavior

The regular collision solver currently skips a pair if either projectile is:

```text
INTERCEPTOR
```

Therefore interceptor interactions are currently governed by:

```text
distance-based interception
```

rather than:

```text
SAT + contact manifold + sequential impulses
```

---

# 77. Current simulation constants

```text
GRAVITY                           = 9.81
AIR_RESISTANCE                    = 0.001

WORLD_SIZE                        = (5000, 5000)

HZ                                = 144
DELTA_TIME                        = 1 / 144

BULLET_COUNT                      = 10
TRAIL_MAX_POINTS                  = 300

TURBULENCE_MAX_X                  = 0.5
TURBULENCE_MAX_Y                  = 0.5
TURBULENCE_DELTA_MAX              = 0.005

MAX_BULLET_VELOCITY               = 1000

EPSILON                           = 1e-6

ANGULAR_DAMPING                   = 0.2
ANGULAR_VELOCITY_STOP_THRESHOLD   = 0.001

RESTITUTION_VELOCITY_THRESHOLD    = 1.0

SOLVER_ITERATIONS                 = 8

DEFENSE_PROTECTION_RADIUS         = 500
DEFENSE_DETECTION_RADIUS          = 1000

DEFENSE_INTERCEPTOR_SPEED         = 1400
DEFENSE_LAUNCH_COOLDOWN           = 0.25
DEFENSE_MAXIMUM_ACTIVE_INTERCEPTORS = 20
DEFENSE_INTERCEPTOR_MAX_TURN_RATE = 3

DEFENSE_INTERCEPTOR_SHAPE         = "357_magnum"
DEFENSE_INTERCEPTOR_SIZE          = 15
DEFENSE_INTERCEPTOR_DENSITY       = 1

DEFENSE_INTERCEPTOR_RESTITUTION       = 0
DEFENSE_INTERCEPTOR_STATIC_FRICTION   = 0
DEFENSE_INTERCEPTOR_DYNAMIC_FRICTION  = 0

DEFENSE_INTERCEPTION_RADIUS       = 40
```

---

# 78. Current solver constants

The following constants are currently local to the collision modules rather than `config.rs`.

Broad-phase grid:

```text
GRID_CELL_SIZE = 100
```

Concave manifold merging:

```text
NORMAL_MERGE_DOT_THRESHOLD = 0.999

CONTACT_MERGE_DISTANCE = 0.01

CONTACT_DUPLICATE_EPSILON = 0.001
```

Direct positional correction:

```text
PENETRATION_SLOP = 0.01

CORRECTION_PERCENTAGE = 0.8
```

---

# 79. Current model limitations

The formulas above describe the implementation as it exists now.

Important simplifications include:

```text
linear drag rather than quadratic aerodynamic drag
```

```text
discrete collision detection
```

```text
no continuous collision detection / time of impact solver
```

```text
no warm starting across physics frames
```

```text
direct penetration projection after velocity solving
```

```text
straight-line threat prediction
```

```text
pure-pursuit interceptor guidance
```

```text
distance-based interceptor detonation
```

```text
interceptors excluded from normal rigid-body collisions
```

These are intentional current implementation choices and potential areas for future development.