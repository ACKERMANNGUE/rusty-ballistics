# Formulas used in the project

This document groups the mathematical formulas used by the project by source file. The goal is to stay faithful to what is actually implemented today, while clearly marking the parts that are still simplified and should be replaced by a more realistic model later.

## src/config.rs

The constants used by the simulations are:

$$
 g = 9.81 \; \text{m/s}^2
$$

$$
\Delta t = \frac{1}{H_z}
$$

with $H_z = 144$ in the current configuration.

The world size is defined as:

$$
W = 5000, \qquad H = 5000
$$

The main numerical constants used in the code are:

$$
\text{air resistance} = 0.001,
\quad
\text{max bullet velocity} = 1000,
\quad
\text{base bullet size} = 0.5
$$

---

## src/models/bullet.rs

The bullet size is currently derived from mass by a linear scaling:

$$
 s = s_0 \cdot m
$$

where:

$$
 s_0 = \text{BASE\_BULLET\_SIZE} = 0.5
$$

The moment of inertia is approximated as a disk-like body:

$$
 I = \frac{1}{2} m s^2
$$

This is a simplified approximation, not a fully general rigid-body formula for arbitrary polygonal shapes.

> TODO: when the shape is more physically accurate, the inertia should depend on the actual polygon mass distribution and not only on the bullet size.

---

## src/models/physics.rs

### 1. Gravity

The simulation uses a constant gravity vector pointing downward in the world coordinates:

$$
\mathbf{a}_{grav} = (0, -g)
$$

with $g = 9.81$.

### 2. Euler integration

The projectile state is updated with a forward Euler integration step:

$$
\mathbf{v}_{t+\Delta t} = \mathbf{v}_t + \mathbf{a}\,\Delta t
$$

$$
\mathbf{x}_{t+\Delta t} = \mathbf{x}_t + \mathbf{v}_{t+\Delta t}\,\Delta t
$$

This is the main integration scheme used in the current implementation.

### 3. Current drag model (linear drag)

The current code computes a relative velocity and applies a drag force proportional to it:

$$
\mathbf{v}_{rel} = \mathbf{v} - \mathbf{v}_{wind} - \mathbf{v}_{turbulence}
$$

$$
\mathbf{F}_{drag} = -k \cdot \mathbf{v}_{rel}
$$

with $k = \text{air resistance}$ in the current code.

Then the drag acceleration is:

$$
\mathbf{a}_{drag} = \frac{\mathbf{F}_{drag}}{m}
$$

and the total acceleration is:

$$
\mathbf{a}_{total} = \mathbf{a}_{grav} + \mathbf{a}_{drag}
$$

The actual update used by the project is therefore:

$$
\mathbf{v}_{t+\Delta t} = \mathbf{v}_t + (\mathbf{a}_{grav} + \mathbf{a}_{drag})\,\Delta t
$$

$$
\mathbf{x}_{t+\Delta t} = \mathbf{x}_t + \mathbf{v}_{t+\Delta t}\,\Delta t
$$

This is the current implementation and is intentionally simplified.

> TODO: Replace the linear drag law with the more realistic quadratic drag law.

### 4. TODO: quadratic drag law (more realistic)

The next physically realistic version should use:

$$
\mathbf{F}_{drag} = -\frac{1}{2}\rho C_d A \|\mathbf{v}_{rel}\|\,\mathbf{v}_{rel}
$$

or, in a simplified constant-coefficient form:

$$
\mathbf{F}_{drag} = -k_q \|\mathbf{v}_{rel}\|\,\mathbf{v}_{rel}
$$

with the corresponding acceleration:

$$
\mathbf{a}_{drag} = \frac{\mathbf{F}_{drag}}{m}
$$

This is the version that should eventually replace the current linear model.

### 5. Shape-dependent projected width used for drag scaling

The code computes the projected width of the bullet polygon along a direction perpendicular to the relative motion.

Let $\mathbf{u}$ be the normalized relative velocity direction:

$$
\mathbf{u} = \frac{\mathbf{v}_{rel}}{\|\mathbf{v}_{rel}\|}
$$

The perpendicular axis is:

$$
\mathbf{n} = (-u_y, u_x)
$$

Then the polygon projection range is:

$$
\text{proj}(P, \mathbf{n}) = \left[\min_{p \in P}(\mathbf{n}\cdot p),\; \max_{p \in P}(\mathbf{n}\cdot p)\right]
$$

and the effective projected width is:

$$
 w_{proj} = \max_{p \in P}(\mathbf{n}\cdot p) - \min_{p \in P}(\mathbf{n}\cdot p)
$$

The drag factor is then approximated as:

$$
\alpha = \frac{w_{proj}}{w_{ref}}
$$

where $w_{ref} = 1.0$ in the implementation.

Then the code effectively uses:

$$
\mathbf{F}_{drag} = -k \cdot \alpha \cdot \mathbf{v}_{rel}
$$

This is a simplified shape-aware aerodynamic approximation.

### 6. Rotation update

The bullet keeps a rotation angle and angular velocity:

$$
\theta_{t+\Delta t} = \theta_t + \omega\,\Delta t
$$

This comes directly from:

$$
\theta = \theta + \omega \cdot \Delta t
$$

in the code.

### 7. Collision detection and spatial grid

The world is partitioned into cells of size $c = 100$:

$$
 x_{cell} = \left\lfloor \frac{x + W/2}{c} \right\rfloor
$$

$$
 y_{cell} = \left\lfloor \frac{y + H/2}{c} \right\rfloor
$$

and the grid size is:

$$
N_x = \left\lceil \frac{W}{c} \right\rceil,
\qquad
N_y = \left\lceil \frac{H}{c} \right\rceil
$$

This is used to reduce the number of bullet pairs checked.

### 8. Out-of-bounds test

A bullet is considered outside the world if:

$$
 x - r < -\frac{W}{2}
\quad \text{or} \quad
 x + r > \frac{W}{2}
$$

$$
 y - r < -\frac{H}{2}
\quad \text{or} \quad
 y + r > \frac{H}{2}
$$

where $r$ is the bullet radius (derived from size).

### 9. Collision response

The code computes the contact point velocity using linear and angular motion:

$$
\mathbf{v}_{contact} = \mathbf{v} + (\omega \times \mathbf{r})
$$

with $\mathbf{r}$ the lever arm between the center of mass and the contact point.

The relative velocity at the contact is:

$$
\mathbf{v}_{rel} = \mathbf{v}_{contact,2} - \mathbf{v}_{contact,1}
$$

and the normal-direction component is:

$$
 v_n = \mathbf{v}_{rel} \cdot \mathbf{n}
$$

The impulse denominator is:

$$
 D = \frac{1}{m_1} + \frac{1}{m_2} + (\mathbf{r}_1 \times \mathbf{n})^2 I_1^{-1} + (\mathbf{r}_2 \times \mathbf{n})^2 I_2^{-1}
$$

The normal impulse magnitude is:

$$
 J_n = -\frac{(1 + e)\,v_n}{D}
$$

where $e$ is the restitution coefficient.

The impulse vector is:

$$
\mathbf{J}_n = J_n\,\mathbf{n}
$$

and the updated velocities become:

$$
\mathbf{v}_1' = \mathbf{v}_1 - \frac{\mathbf{J}_n}{m_1}
$$

$$
\mathbf{v}_2' = \mathbf{v}_2 + \frac{\mathbf{J}_n}{m_2}
$$

The angular effects are also updated with:

$$
\omega_1' = \omega_1 - \frac{\mathbf{r}_1 \times \mathbf{J}_n}{I_1}
$$

$$
\omega_2' = \omega_2 + \frac{\mathbf{r}_2 \times \mathbf{J}_n}{I_2}
$$

### 10. Friction approximation used in the code

After the normal impulse, the code computes the tangential relative velocity and applies a simplified Coulomb friction approximation:

$$
\mathbf{v}_{tangent} = \mathbf{v}_{rel} - (\mathbf{v}_{rel}\cdot \mathbf{n})\mathbf{n}
$$

Then, with tangent direction $\mathbf{t}$:

$$
\mathbf{t} = \frac{\mathbf{v}_{tangent}}{\|\mathbf{v}_{tangent}\|}
$$

the friction impulse magnitude is approximated by:

$$
 J_t \approx -\frac{(\mathbf{v}_{rel}^{after}\cdot \mathbf{t})}{D_t}
$$

and clamped with a Coulomb-like condition:

$$
\|\mathbf{J}_t\| \le \mu_s \|\mathbf{J}_n\| \quad \text{or} \quad \|\mathbf{J}_t\| \le \mu_d \|\mathbf{J}_n\|
$$

depending on whether the contact is considered static or dynamic.

This is a simplified friction model, but it is the one currently implemented.

### 11. Penetration correction

The code resolves overlapping bodies by moving them apart along the collision normal:

$$
\delta = \text{penetration depth}
$$

$$
\text{correction} = \frac{(\delta - s)_{+}}{\frac{1}{m_1} + \frac{1}{m_2}} \cdot c \cdot \mathbf{n}
$$

with:

$$
 s = 0.01,
\qquad
 c = 0.8
$$

The final update is:

$$
\mathbf{x}_1' = \mathbf{x}_1 - \frac{\text{correction}}{m_1}
$$

$$
\mathbf{x}_2' = \mathbf{x}_2 + \frac{\text{correction}}{m_2}
$$

This is a simplified positional correction used to prevent sticking.

---

## src/models/wind.rs

The current wind is modeled as a direction vector and a scalar speed:

$$
\mathbf{v}_{wind} = \mathbf{d} \cdot s
$$

where:

- $\mathbf{d}$ is a wind direction vector
- $s$ is the wind speed

The project also adds a turbulence contribution:

$$
\Delta x \sim \mathcal{U}(-\Delta_{max}, \Delta_{max})
$$

$$
\Delta y \sim \mathcal{U}(-\Delta_{max}, \Delta_{max})
$$

and updates the turbulence vector as:

$$
\mathbf{v}_{turbulence}^{n+1} = \operatorname{clamp}\left(\mathbf{v}_{turbulence}^{n} + (\Delta x, \Delta y),\; [-T_x, T_x],\;[-T_y, T_y]\right)
$$

where:

$$
T_x = 0.5,
\qquad
T_y = 0.5,
\qquad
\Delta_{max} = 0.005
$$

The wind direction angle is computed as:

$$
\theta = \operatorname{atan2}(d_y, d_x)
$$

and converted to degrees with:

$$
\theta_{deg} = \theta \cdot \frac{180}{\pi}
$$

---

## src/geometry/projection.rs

The projection of a polygon onto an axis is computed from a dot product with each vertex:

$$
\pi_{\mathbf{a}}(p) = \mathbf{a}\cdot p
$$

Then:

$$
\min_{proj} = \min_{p \in P}(\mathbf{a}\cdot p)
$$

$$
\max_{proj} = \max_{p \in P}(\mathbf{a}\cdot p)
$$

The function returns:

$$
(\min_{proj}, \max_{proj})
$$

This is used for SAT-based collision testing.

---

## src/collision/separating_axis_theorem.rs

For each separating axis $\mathbf{a}$, the projected intervals are compared:

$$
[a_{min}, a_{max}] = \text{proj}(P_1, \mathbf{a})
$$

$$
[b_{min}, b_{max}] = \text{proj}(P_2, \mathbf{a})
$$

The polygons overlap along that axis if:

$$
\max(a_{min}, b_{min}) < \min(a_{max}, b_{max})
$$

The overlap magnitude is:

$$
\text{overlap} = \min(a_{max}, b_{max}) - \max(a_{min}, b_{min})
$$

If the overlap is positive for every axis, the polygons are colliding.

The best collision axis is the one with the minimum overlap:

$$
\mathbf{n} = \arg\min_{\mathbf{a}} \text{overlap}(\mathbf{a})
$$

The contact normal is then oriented using the center-to-center direction:

$$
\mathbf{d}_{A\to B} = \mathbf{c}_B - \mathbf{c}_A
$$

and the sign of the normal is corrected so that:

$$
\mathbf{d}_{A\to B} \cdot \mathbf{n} \ge 0
$$

This is the SAT-driven collision normal used by the response phase.

---

## Summary of the exact formulas currently used

The implementation is currently based on the following set of equations:

$$
\mathbf{a}_{grav} = (0, -g)
$$

$$
\mathbf{v}_{t+\Delta t} = \mathbf{v}_t + \mathbf{a}\,\Delta t
$$

$$
\mathbf{x}_{t+\Delta t} = \mathbf{x}_t + \mathbf{v}_{t+\Delta t}\,\Delta t
$$

$$
\mathbf{v}_{rel} = \mathbf{v} - \mathbf{v}_{wind} - \mathbf{v}_{turbulence}
$$

$$
\mathbf{F}_{drag} = -k\,\mathbf{v}_{rel}
$$

$$
\mathbf{a}_{drag} = \frac{\mathbf{F}_{drag}}{m}
$$

$$
\mathbf{a}_{total} = \mathbf{a}_{grav} + \mathbf{a}_{drag}
$$

$$
\mathbf{v}_{wind} = \mathbf{d}\,s
$$

$$
\theta = \operatorname{atan2}(d_y, d_x)
$$

$$
\mathbf{v}_{contact} = \mathbf{v} + (\omega \times \mathbf{r})
$$

$$
J_n = -\frac{(1+e)\,v_n}{D}
$$

$$
D = \frac{1}{m_1} + \frac{1}{m_2} + (\mathbf{r}_1 \times \mathbf{n})^2 I_1^{-1} + (\mathbf{r}_2 \times \mathbf{n})^2 I_2^{-1}
$$

$$
\text{overlap} = \min(a_{max}, b_{max}) - \max(a_{min}, b_{min})
$$

$$
 x_{cell} = \left\lfloor \frac{x + W/2}{c} \right\rfloor,
\qquad
 y_{cell} = \left\lfloor \frac{y + H/2}{c} \right\rfloor
$$

---

## Final note

This project currently uses a physically simplified model that is good for a sandbox and for experimentation, but some parts are intentionally approximate:

- linear drag instead of quadratic drag
- simplified inertia model based on size
- simplified Coulomb friction handling
- approximate collision correction and angular damping

> TODO: Replace the current simplified formulas with more realistic rigid-body and aerodynamic expressions when the simulation evolves toward a more accurate ballistic model.
