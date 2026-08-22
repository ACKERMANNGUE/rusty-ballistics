# Formulas used in the project

This document gathers the equations used by the physics model of the simulation, as well as the current wind model and the more realistic quadratic model that could be implemented afterward.

## 1. Gravity

Gravity is modeled as a vertical acceleration downward.

- Gravitational acceleration:
  - g = 9.81 m/s²
- Gravitational acceleration vector:
  - a_grav = (0, -g)

## 2. Time integration (Euler method)

Movement is computed using a forward Euler update.

- v_{t+1} = v_t + a * Δt
- x_{t+1} = x_t + v_{t+1} * Δt

This is the standard explicit integration scheme used for simple projectile simulation.

## 3. Linear drag model (current model)

The current simulation uses a linear drag force proportional to velocity.

- F_drag = -k * v

Where:
- k = drag coefficient
- v = projectile velocity

For a projectile in wind:
- v_rel = v - v_wind
- F_drag = -k * v_rel

Then the acceleration becomes:
- a_drag = F_drag / m
- a_total = a_grav + a_drag

And the update becomes:
- v_{t+1} = v_t + a_total * Δt
- x_{t+1} = x_t + v_{t+1} * Δt

This is a simplified linear drag model.

## 4. Wind model (current)

Wind is represented by a direction vector and a scalar speed.

- v_wind = d * s

Where:
- d = unit direction vector of the wind
- s = wind speed

The relative velocity of the projectile with respect to the air is therefore :
- v_rel = v - (d * s)

This is the model currently used in the simulation.

## 5. Quadratic drag model (future model)

A more realistic aerodynamic model uses quadratic drag :

- F_drag = -k_q * ||v_rel|| * v_rel

Where:
- v_rel = v - v_wind
- k_q = quadratic drag coefficient

In standard aerodynamics, this is often written as :
- F_drag = -1/2 * ρ * C_d * A * ||v_rel|| * v_rel

Then:
- a_drag = F_drag / m
- a_total = a_grav + a_drag

The future update would remain :
- v_{t+1} = v_t + a_total * Δt
- x_{t+1} = x_t + v_{t+1} * Δt

This is the natural next evolution if the project moves from linear drag to a more physically realistic drag law.

## 6. Projectile collision response

Collision resolution is based on an impulse along the collision normal and the relative velocity.

- direction = normalize(p2 - p1)
- relative_velocity = v1 - v2
- s = relative_velocity · direction

If s <= 0, there is no separating impulse in the collision direction.

Otherwise:
- J = 2 * s / (1/m1 + 1/m2)
- v1' = v1 - (J / m1) * direction
- v2' = v2 + (J / m2) * direction

This gives the post-collision velocities for two masses involved in a direct impact.

## 7. Spatial grid

The project divides the world into cells to accelerate collision detection.

- x_index = floor((x + world_width / 2) / cell_size)
- y_index = floor((y + world_height / 2) / cell_size)

Where:
- cell_size = 100.0

This allows each projectile to be assigned to a grid cell for quicker neighborhood checks.


## 8. Summary of the main formulas

- a_grav = (0, -g)
- v_wind = d * s
- v_rel = v - v_wind
- F_drag = -k * v_rel
- a_drag = F_drag / m
- a_total = a_grav + a_drag
- v_{t+1} = v_t + a_total * Δt
- x_{t+1} = x_t + v_{t+1} * Δt
- J = 2 * s / (1/m1 + 1/m2)
- x_index = floor((x + W/2) / cell_size)
- y_index = floor((y + H/2) / cell_size)

## 9. Conclusion

The current simulation uses a simple and explicit physical model :
- constant gravity
- linear drag
- wind as a vector velocity field
- Euler integration
- impulse-based collision response

References:
- [Gravity](https://en.wikipedia.org/wiki/Gravity)
- [Euler method](https://en.wikipedia.org/wiki/Euler_method)
- [Drag (physics)](https://en.wikipedia.org/wiki/Drag_(physics))
- [Drag equation](https://en.wikipedia.org/wiki/Drag_equation)
- [Wind](https://en.wikipedia.org/wiki/Wind)
- [Impulse (physics)](https://en.wikipedia.org/wiki/Impulse_(physics))
