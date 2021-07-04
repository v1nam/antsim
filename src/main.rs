use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::{HashSet, HashMap};

fn window_conf() -> Conf {
    Conf {
        window_title: "Ant Simulation".to_owned(),
        window_width: 950,
        window_height: 650,
        ..Default::default()
    }
}

enum Pheromone {
    Home,
    Food,
}

struct Ant {
    velocity: Vec2,
    position: Vec2,
    target: Vec2,
    has_food: bool,
}

impl Ant {
    fn update(&mut self, food_pos: &mut HashSet<(u32, u32)>, pheromones: &mut HashMap<(u32, u32), Pheromone>) {
        let w = screen_width();
        let h = screen_height();
        if self.position.x >= w || self.position.x <= 0. {
            self.velocity.x *= -1.;
        }
        if self.position.y >= h || self.position.y <= 0. {
            self.velocity.y *= -1.;
        }
        if self.target.x <= 0. {
            self.target.x *= -1.;
        }
        if self.target.x >= w {
            self.target.x = w - (self.target.x - w);
        }
        if self.target.y <= 0. {
            self.target.y *= -1.;
        }
        if self.target.y >= h {
            self.target.y = h - (self.target.y - h);
        }
        for x in self.position.x as i32 - 3..self.position.x as i32 + 3 {
            for y in self.position.y as i32 - 3..self.position.y as i32 + 3 {
                if x > 0 && y > 0 {
                    if !self.has_food && food_pos.contains(&(x as u32, y as u32)) {
                        self.target = vec2(x as f32, y as f32);
                        self.has_food = true;
                        food_pos.remove(&(x as u32, y as u32));
                    } else if pheromones.contains() {}
                }
            }
        }
        if self.target.distance(self.position) <= 1. {
            let r = gen_range(20., 40.);
            let p = self.velocity + self.position;
            let theta = (p.y - self.position.y).atan2(p.x - self.position.x);
            let angle = gen_range(
                theta - std::f32::consts::PI / 6.,
                theta + std::f32::consts::PI / 6.,
            );
            self.target = vec2(
                self.position.x + r * angle.cos(),
                self.position.y + r * angle.sin(),
            );
        }
        let direc = (self.target - self.position).normalize();
        let dv = direc * 1.6;
        let accel = ((dv - self.velocity) * 1.6).clamp_length_max(1.6);
        self.velocity = (self.velocity + accel).clamp_length_max(1.6);
        self.position += self.velocity;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ants: Vec<Ant> = Vec::new();
    let mut food_pos: HashSet<(u32, u32)> = HashSet::new();
    let mut pheromones: HashMap<(u32, u32), Pheromone> = HashMap::new();
    let start_pos = vec2(50., 50.);
    for _ in 0..50 {
        let ang = gen_range(0.0, std::f32::consts::PI * 2.);
        let pos = vec2(start_pos.x + ang.cos(), start_pos.y + ang.sin());
        ants.push(Ant {
            position: start_pos,
            velocity: vec2(1., 1.),
            target: pos,
            has_food: false,
        });
    }
    loop {
        clear_background(WHITE);
        if is_mouse_button_down(MouseButton::Left) {
            let m_pos = mouse_position();
            food_pos.insert((m_pos.0 as u32, m_pos.1 as u32));
        }
        for ant in ants.iter_mut() {
            ant.update(&mut food_pos, &mut pheromones);
            draw_circle(ant.position.x, ant.position.y, 2., BLACK);
            if ant.has_food {
                let fp = ant.position + ant.velocity.normalize();
                draw_circle(fp.x, fp.y, 1.7, GREEN);
            }
        }
        for food in food_pos.iter() {
            draw_circle(food.0 as f32, food.1 as f32, 1.4, GREEN);
        }
        draw_circle(start_pos.x, start_pos.y, 30., GRAY);
        next_frame().await
    }
}
