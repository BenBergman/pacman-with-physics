extern crate native;
extern crate "nalgebra" as na;
extern crate ncollide;
extern crate "nphysics2df32" as nphysics;
extern crate nphysics_testbed2d;

use na::{Vec2, Translation};
use ncollide::geom::{Ball, Plane};
use ncollide::math::{Vect};
use nphysics::world::World;
use nphysics::object::RigidBody;
use nphysics_testbed2d::Testbed;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn make_wall(plane_vector: Vect, translation_vector: Vect) -> RigidBody {
    let mut rb = RigidBody::new_static(Plane::new(plane_vector), 0.3, 0.6);
    rb.append_translation(&translation_vector);
    return rb
}

fn main() {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vec2::new(0.0f32, 0.0));

    world.add_body(make_wall(Vec2::new(-1.0f32, 0.0), Vec2::new(30.0, 0.0)));
    world.add_body(make_wall(Vec2::new(1.0f32, 0.0), Vec2::new(-30.0, 0.0)));
    world.add_body(make_wall(Vec2::new(0.0f32, -1.0), Vec2::new(0.0, 30.0)));
    world.add_body(make_wall(Vec2::new(0.0f32, 1.0), Vec2::new(0.0, -30.0)));

    /*
     * Create the balls
     */
    let num     = (400.0f32.sqrt()) as uint;
    let rad     = 0.5;
    let shift   = 5.0 * rad;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift * (num as f32) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift - centery;

            let mut rb3 = RigidBody::new_dynamic(Ball::new(rad), 1.0f32, 0.3, 0.6);

            rb3.append_translation(&Vec2::new(x, y));

            world.add_body(rb3);
        }
    }

    /*
     * Run the simulation.
     */
    let mut testbed = Testbed::new(world);

    testbed.run();
}
