use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::{HashMap, HashSet};

fn window_conf() -> Conf {
    Conf {
        window_title: "Ant Simulation".to_owned(),
        window_width: 950,
        window_height: 650,
        ..Default::default()
    }
}

struct Ant {
    velocity: Vec2,
    position: Vec2,
    target: Vec2,
    has_food: bool,
    hpher_count: f32,
    fpher_count: f32,
}

impl Ant {
    fn update(
        &mut self,
        food_pos: &mut HashSet<(u32, u32)>,
        home_pheromones: &mut HashMap<(u32, u32), f32>,
        food_pheromones: &mut HashMap<(u32, u32), f32>,
    ) {
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

        if self.has_food && self.position.distance(vec2(50., 50.)) <= 30. {
            self.has_food = false;
            self.hpher_count = 0.0;
            self.fpher_count = 0.0;
            self.velocity *= -1.;
        }
        let mut allowed_pheromones: HashMap<(u32, u32), f32> = HashMap::new();
        for x in self.position.x as i32 - 3..self.position.x as i32 + 3 {
            for y in self.position.y as i32 - 3..self.position.y as i32 + 3 {
                if x > 0 && y > 0 && (x, y) != (self.position.x as i32, self.position.y as i32) {
                    if !self.has_food && food_pos.contains(&(x as u32, y as u32)) {
                        self.target = vec2(x as f32, y as f32);
                        self.has_food = true;
                        self.velocity *= -1.;
                        food_pos.remove(&(x as u32, y as u32));
                        break;
                    }
                    if let Some(pv) = home_pheromones.get(&(x as u32, y as u32)) {
                        if self.has_food {
                            allowed_pheromones.insert((x as u32, y as u32), *pv);
                        }
                    }
                    if let Some(pv) = food_pheromones.get(&(x as u32, y as u32)) {
                        if !self.has_food {
                            allowed_pheromones.insert((x as u32, y as u32), *pv);
                        }
                    }
                }
            }
        }
        let mut attractiveness: HashMap<(u32, u32), f32> = HashMap::new();
        let mut total_sum = 0.0;
        for (allowed_location, pher_amount) in &allowed_pheromones {
            let distance = self
                .position
                .distance(vec2(allowed_location.0 as f32, allowed_location.1 as f32));
            let edge_prob = pher_amount.powf(0.5) * (1.0 / distance).powf(1.2);
            attractiveness.insert(*allowed_location, edge_prob);
            total_sum += edge_prob;
        }
        if total_sum == 0.0 {
            total_sum = std::f32::MIN_POSITIVE;
            for v in attractiveness.values_mut() {
                *v = std::f32::MIN_POSITIVE;
            }
        }
        let random_fl = gen_range(0.0, 1.0);
        let mut upto = 0.0;
        let mut found_pher = false;
        for (location, inten) in &attractiveness {
            let weight = inten / total_sum;
            if weight + upto >= random_fl {
                self.target = vec2(location.0 as f32, location.1 as f32);
                found_pher = true;
                break;
            }
            upto += weight;
        }

        if self.target.distance(self.position) <= 1. {
            if !found_pher {
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
        }
        let direc = (self.target - self.position).normalize();
        let dv = direc * 1.6;
        let accel = ((dv - self.velocity) * 1.6).clamp_length_max(1.6);
        self.velocity = (self.velocity + accel).clamp_length_max(1.6);
        self.position += self.velocity;
        if self.has_food {
            self.fpher_count += 1.0;
            let new_pher = 1000.0 / self.fpher_count;
            food_pheromones
                .entry((self.position.x as u32, self.position.y as u32))
                .and_modify(|a| *a += new_pher)
                .or_insert(new_pher);
        } else {
            self.hpher_count += 1.0;
            let new_pher = 1000.0 / self.hpher_count;
            home_pheromones
                .entry((self.position.x as u32, self.position.y as u32))
                .and_modify(|a| *a += new_pher)
                .or_insert(new_pher);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ants: Vec<Ant> = Vec::new();
    let mut food_pos: HashSet<(u32, u32)> = HashSet::new();
    let mut home_pheromones: HashMap<(u32, u32), f32> = HashMap::new();
    let mut food_pheromones: HashMap<(u32, u32), f32> = HashMap::new();

    let start_pos = vec2(50., 50.);
    for _ in 0..2 {
        let ang = gen_range(0.0, std::f32::consts::PI * 2.);
        let pos = vec2(start_pos.x + ang.cos(), start_pos.y + ang.sin());
        ants.push(Ant {
            position: start_pos,
            velocity: vec2(1., 1.),
            target: pos,
            has_food: false,
            hpher_count: 0.0,
            fpher_count: 0.0,
        });
    }
    loop {
        clear_background(WHITE);
        if is_mouse_button_down(MouseButton::Left) {
            let m_pos = mouse_position();
            food_pos.insert((m_pos.0 as u32, m_pos.1 as u32));
        }
        for ant in ants.iter_mut() {
            ant.update(&mut food_pos, &mut home_pheromones, &mut food_pheromones);
            draw_circle(ant.position.x, ant.position.y, 2., BLACK);
            if ant.has_food {
                let fp = ant.position + ant.velocity.normalize();
                draw_circle(fp.x, fp.y, 1.7, GREEN);
            }
        }
        for (hpk, hpv) in home_pheromones.iter_mut() {
            *hpv *= 0.006;
            draw_circle(hpk.0 as f32, hpk.1 as f32, 1., BLUE);
        }
        for (fpk, fpv) in food_pheromones.iter_mut() {
            *fpv *= 0.006;
            draw_circle(fpk.0 as f32, fpk.1 as f32, 1., RED);
        }

        for food in food_pos.iter() {
            draw_circle(food.0 as f32, food.1 as f32, 1.4, GREEN);
        }
        draw_circle(start_pos.x, start_pos.y, 30., GRAY);
        next_frame().await
    }
}
