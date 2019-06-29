extern crate kiss3d;
extern crate nalgebra as na;
extern crate spherro;

#[macro_use]
extern crate itertools;

use na::{Vector3, UnitQuaternion, Translation3, Point3};
use kiss3d::window::Window;
use kiss3d::event::{WindowEvent, Key, Action};
use kiss3d::light::Light;
use spherro::{Universe};

const VIZ_SCALE: f32 = 0.001;

fn main() {
    let mut window = Window::new("spherro");
    window.set_background_color(0.85, 0.85, 0.85);

    let eye = na::Point3::new(300.0, 300.0, 800.0) * VIZ_SCALE;
    let look_at = na::Point3::new(300.0, 300.0, 0.0) * VIZ_SCALE;
    let mut first_person = kiss3d::camera::FirstPerson::new(eye, look_at);

    let mut universe = Universe::new(600, 600);
    let mut viz_objs: Vec<kiss3d::scene::SceneNode> = Vec::new();

    for i in 0..universe.get_size() {
        let r = 10.0 * VIZ_SCALE;
        let mut obj = window.add_sphere(r);
        obj.set_color(0.0, 0.0, 0.0);
        viz_objs.push(obj);
    }

    // Draw some borders
    for i in 0..4 {
        let sx = 10.0 * VIZ_SCALE;
        let sy = 600.0 * VIZ_SCALE;
        let sz = 10.0 * VIZ_SCALE;

        let mut c = window.add_cube(sx, sy, sz);
        c.set_color(1.0, 0.0, 0.0);

        let t = na::Translation3::new(0.0, 300.0*VIZ_SCALE, 0.0);
        let r = na::UnitQuaternion::from_euler_angles(
                                    0.0, 0.0, i as f32 * std::f32::consts::PI / 2.0);
        let ro = na::Translation3::new(300.0*VIZ_SCALE, 300.0*VIZ_SCALE, 0.0); // rotation origin
        let ro_inv = na::Translation3::new(-300.0*VIZ_SCALE, -300.0*VIZ_SCALE, 0.0); // rotation origin
        let T = ro* r * ro_inv * t;
        c.set_local_transformation(T);
    }

    window.set_light(Light::StickToCamera);

    let mut last_time = std::time::Instant::now();
    while !window.should_close() {
        window.render_with_camera(&mut first_person);

        for (pi, obj) in izip!(universe.get_particles(), &mut viz_objs) {
            let pos = pi.pos * VIZ_SCALE;
            obj.set_color(pi.col.x, pi.col.y, pi.col.z);
            obj.set_local_translation(na::Translation3::new(
                pos.x,
                pos.y,
                pos.z,
            ));
        }

        let curr_time = std::time::Instant::now();
        let dt_ms = (curr_time - last_time).as_millis();
        last_time = curr_time;
        let dt = (dt_ms as f32) / 1000.0;

        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::Space, Action::Press, _) => {

                },
                WindowEvent::Key(Key::R, Action::Press, _) => {
                    universe = Universe::new(600, 600);
                },
                _ => {}
            }
        }
        universe.clear_colors();
        universe.debug_update(dt);
        for _ in 0..10 {
            universe.update(0.001);
        }

        // Draw axes

        window.draw_line(
            &Point3::origin(),
            &Point3::new(1.0, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 0.0),
        );
        window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 1.0, 0.0),
            &Point3::new(0.0, 1.0, 0.0),
        );
        window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 0.0, 1.0),
            &Point3::new(0.0, 0.0, 1.0),
        );


    }
}

