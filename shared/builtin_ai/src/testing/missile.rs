use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        if let Some(contact) = scan() {
            seek(contact.position, contact.velocity);

            let dp = contact.position - position();
            let dv = contact.velocity - velocity();
            if dp.length().min((dp + dv * TICK_LENGTH).length()) < 25.0 {
                explode();
            }

            set_radar_heading((contact.position - position()).angle());
            set_radar_width((20.0 * TAU / dp.length()).clamp(TAU / 360.0, TAU));
        } else {
            set_radar_width(TAU / 16.0);
            set_radar_heading(radar_heading() + radar_width());
        }
    }
}

pub fn seek(p: Vec2, v: Vec2) {
    const N: f64 = 4.0;
    let acc = if fuel() > 200.0 {
        max_forward_acceleration()
    } else {
        0.0
    };
    let dp = p - position();
    let dv = v - velocity();
    let closing_speed = -(dp.y * dv.y - dp.x * dv.x).abs() / dp.length();
    let los = dp.angle();
    let los_rate = (dp.y * dv.x - dp.x * dv.y) / (dp.length() * dp.length());
    let badv = -(dv - dv.dot(dp) * dp.normalize() / dp.length());
    let ax = (acc - badv.length() * 5.0).clamp(0.0, acc);
    let ay = N * closing_speed * los_rate;
    let a = vec2(ax, ay).rotate(los);
    let a = vec2(max_forward_acceleration(), 0.0).rotate(a.angle());
    accelerate(a);

    turn_to(a.angle());
}

fn turn_to(target_heading: f64) {
    let heading_error = angle_diff(heading(), target_heading);
    turn(10.0 * heading_error);
}
