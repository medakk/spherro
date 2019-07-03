use spherro::util::{Vector3f};
use cgmath::{InnerSpace};

extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::event::{Action, WindowEvent, Key};
use kiss3d::window::Window;
use na::{Translation3, Point3};

fn main() {
    let circle_radius: f32 = 5.0;
    let rect_w = 20.0;
    let rect_h = 15.0;


    let mut window = Window::new("Kiss3d: rectangle");
    let mut cube = window.add_cube(rect_w, rect_h, 1.0);
    let mut sphere = window.add_sphere(circle_radius);

    let eye = na::Point3::new(0.0, 0.0 , 20.0);
    let look_at = na::Point3::new(0.0, 0.0 ,0.0);
    let mut first_person = kiss3d::camera::FirstPerson::new(eye, look_at);

    cube.set_color(0.0, 0.0, 1.0);
    sphere.set_color(1.0, 1.0, 1.0);

    window.set_light(Light::StickToCamera);

    const AMOUNT: f32 = 2.0;
    let mut x = 0.0;
    let mut y = 0.0;
    while !window.should_close() {
        window.render_with_camera(&mut first_person);

        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::W, Action::Press, _) => {
                    y += AMOUNT;
                },
                WindowEvent::Key(Key::S, Action::Press, _) => {
                    y -= AMOUNT;
                },
                WindowEvent::Key(Key::A, Action::Press, _) => {
                    x -= AMOUNT;
                },
                WindowEvent::Key(Key::D, Action::Press, _) => {
                    x += AMOUNT;
                },
                _ => {},
            }
        }

        cube.set_local_translation(Translation3::new(x, y, 0.0));

        if circle_rect_collide(
            (Vector3f::new(0.0, 0.0, 0.0), circle_radius),
            &(Vector3f::new(x-rect_w/2.0, y-rect_h/2.0, 0.0), Vector3f::new(x+rect_w/2.0, y+rect_h/2.0, 0.0)),
        ) {
            sphere.set_color(1.0, 0.0, 0.0);
        } else {
            sphere.set_color(1.0, 1.0, 1.0);
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

fn circle_line_collision(circle: &(Vector3f, f32), line: &(Vector3f, Vector3f)) -> bool {
    let (p, r) = circle;

    if (line.0 - circle.0).magnitude2() < r.powi(2) {
        return true;
    }
    if (line.1 - circle.0).magnitude2() < r.powi(2) {
        return true;
    }

    let line_vec = (line.1 - line.0).normalize();
    let lambda = line_vec.dot(p - line.0);

    let poi = lambda * line_vec + line.0;
    let t = if line.1.x - line.0.x > 1e-6 {
        (poi.x - line.0.x) / (line.1.x - line.0.x)
    } else {
        (poi.y - line.0.y) / (line.1.y - line.0.y)
    };

    if t > 0.0 && t < 1.0 {
        let intersect_length = (poi - *p).magnitude();
        intersect_length < *r
    } else {
        false
    }
}

fn circle_rect_collide(circle: (Vector3f, f32), rect: &(Vector3f, Vector3f)) -> bool {
    let circle_in_rect = circle.0.x > rect.0.x &&
                         circle.0.x < rect.1.x &&
                         circle.0.y > rect.0.y &&
                         circle.0.y < rect.1.y;
    if circle_in_rect {
        return true;
    }

    let l1 = (
        rect.0,
        Vector3f::new(rect.1.x, rect.0.y, 0.0),
    );
    let l2 = (
        Vector3f::new(rect.1.x, rect.0.y, 0.0),
        rect.1,
    );
    let l3 = (
        Vector3f::new(rect.0.x, rect.1.y, 0.0),
        rect.1,
    );
    let l4 = (
        Vector3f::new(rect.0.x, rect.1.y, 0.0),
        rect.0,
    );

    circle_line_collision(&circle, &l1) ||
    circle_line_collision(&circle, &l2) ||
    circle_line_collision(&circle, &l3) ||
    circle_line_collision(&circle, &l4)
}