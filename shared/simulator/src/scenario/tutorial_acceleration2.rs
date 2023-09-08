use super::{draw_ngon, prelude::*};

pub struct TutorialAcceleration2 {
    hit_target: bool,
    target: Option<Point2<f64>>,
}

impl TutorialAcceleration2 {
    pub fn new() -> Self {
        Self {
            hit_target: false,
            target: None,
        }
    }
}

impl Scenario for TutorialAcceleration2 {
    fn name(&self) -> String {
        "tutorial_acceleration2".into()
    }

    fn human_name(&self) -> String {
        "Tutorial 3: Acceleration #2".into()
    }

    fn init(&mut self, sim: &mut Simulation, seed: u32) {
        let mut rng = new_rng(seed);
        self.target = Some(
            Rotation2::new(rng.gen_range(0.0..std::f64::consts::TAU))
                .transform_point(&point![rng.gen_range(400.0..500.0), 0.0]),
        );
        let handle = ship::create(
            sim,
            Rotation2::new(rng.gen_range(0.0..std::f64::consts::TAU))
                .transform_vector(&vector![rng.gen_range(100.0..200.0), 0.0]),
            vector![0.0, 0.0],
            0.0,
            fighter_without_missiles_or_radar(0),
        );
        sim.write_target(handle, self.target.unwrap().coords, vector![0.0, 0.0]);
    }

    fn tick(&mut self, sim: &mut Simulation) {
        if let Some(&handle) = sim.ships.iter().next() {
            let ship = sim.ship(handle);
            if (ship.position().vector - self.target.unwrap().coords).magnitude() < 50.0 {
                self.hit_target = true;
            }
        }
    }

    fn lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        let center: Point2<f64> = self.target.unwrap();
        let n = 20;
        let r = 50.0;
        let color = if self.hit_target {
            vector![0.0, 1.0, 0.0, 1.0]
        } else {
            vector![1.0, 0.0, 0.0, 1.0]
        };
        draw_ngon(&mut lines, n, center, r, color);

        lines
    }

    fn status(&self, _: &Simulation) -> Status {
        if self.hit_target {
            Status::Victory { team: 0 }
        } else {
            Status::Running
        }
    }

    fn initial_code(&self) -> Vec<Code> {
        vec![builtin("tutorial/tutorial_acceleration2_initial")]
    }

    fn solution(&self) -> Code {
        builtin("tutorial/tutorial_acceleration2_solution")
    }

    fn next_scenario(&self) -> Option<String> {
        Some("tutorial_rotation".to_string())
    }

    fn previous_names(&self) -> Vec<String> {
        vec!["tutorial03".into()]
    }
}
