extern crate native;
extern crate "nalgebra" as na;
extern crate ncollide;
extern crate "nphysics2df32" as nphysics;
extern crate nphysics_testbed2d;

use na::{Vec2, Translation};
use ncollide::shape::{Ball, Plane};
use nphysics::world::World;
use nphysics::object::RigidBody;
use nphysics_testbed2d::Testbed;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn make_wall(plane_vector: Vec2<f32>, translation_vector: Vec2<f32>) -> RigidBody {
    let mut rb = RigidBody::new_static(Plane::new(plane_vector), 0.3, 0.6);
    rb.append_translation(&translation_vector);
    return rb
}

fn main() {
    /*
     * World
     */
    let mut world = World::new();

    world.add_body(make_wall(Vec2::new(-1.0f32, 0.0), Vec2::new(30.0, 0.0)));
    world.add_body(make_wall(Vec2::new(1.0f32, 0.0), Vec2::new(-30.0, 0.0)));
    world.add_body(make_wall(Vec2::new(0.0f32, -1.0), Vec2::new(0.0, 30.0)));
    world.add_body(make_wall(Vec2::new(0.0f32, 1.0), Vec2::new(0.0, -30.0)));

    /*
     * Create the balls
     */
    let side_length_in_balls     = (400.0f32.sqrt()) as uint;
    let ball_radius     = 0.5;
    let ball_spacing   = 5.0 * ball_radius;
    let center_x = ball_spacing * (side_length_in_balls as f32) / 2.0;
    let center_y = ball_spacing * (side_length_in_balls as f32) / 2.0;

    for i in range(0u, side_length_in_balls) {
        for j in range(0u, side_length_in_balls) {
            let x = i as f32 * ball_spacing - center_x;
            let y = j as f32 * ball_spacing - center_y;

            let mut rb3 = RigidBody::new_dynamic(Ball::new(ball_radius), 1.0f32, 0.3, 0.6);

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
