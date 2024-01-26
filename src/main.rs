use std::f32::consts::PI;
use std::io;
use slint::Model;
use slint::VecModel;
use std::rc::Rc;
use rand::Rng;
use slint::{Timer, TimerMode};
use std::time::Duration;

struct Particle {
    x: f32,
    y: f32,
    vel: f32,
    bearing: f32,
    mass: f32,
}

impl Particle {
    
    fn initialise(x: f32, y: f32, mass: f32) -> Particle {
        Particle {
            x,
            y,
            vel :0f32,
            bearing: 0f32,
            mass
        }
    }
    
    fn set_vel(&mut self, vel: f32, bearing: f32) {
        if (bearing >= 0f32) && (bearing <= 360f32) {
            self.vel = vel;
            self.bearing = (bearing * PI) / 180f32;
        } 
        else {
            println!("Invalid Bearing!");
        }
    }
    
    fn move_particles(&mut self, delta_time: f32) {
        let x_vel = self.bearing.sin() * self.vel;
        let y_vel = self.bearing.cos() * self.vel;
        
        self.x += x_vel * delta_time;
        self.y += y_vel * delta_time;
        
        println!("x = {}\ny = {}", self.x, self.y);
        
        
        
    }
}

fn main() {
    let mut particles_sim: Vec<Particle> = Vec::new();
    
    let mut rng = rand::thread_rng();
    for _i in 0..10 {
        particles_sim.push(Particle::initialise(rng.gen_range(0.0..1000.0) ,rng.gen_range(0.0..1000.0), 10f32));
    }
    
    for mut particle in particles_sim {
        particle.set_vel(rng.gen_range(150.0..400.0), rng.gen_range(0..360) as f32)
    }
    
    let main_window = MainWindow::new().unwrap();
    
    let vis_particles: Vec<Particles> = main_window.get_particles().iter().collect();
    let particles_model = Rc::new(VecModel::from(vis_particles));
    main_window.set_particles(particles_model.clone().into());
    
    for (i, vp) in vis_particles.iter().enumerate() {
        vp.x = particles_sim[i].x;
        particles_model.set_row_data(i, vp.clone())
    }
    
    let timer = Timer::default();
    let fps = 100f32;
    let delta_time = 1f32 / fps;
    let weak = main_window.as_weak();
    timer.start(TimerMode::Repeated, Duration::from_millis((delta_time * 100.0) as u64),  move || {
        let window = weak.unwrap();
        let  sims = particles_sim.iter();

        let particle_sim: &Particle = sims.next().unwrap();
        
        particle_sim.move_particles(delta_time);
        
        if (particle_sim.x >= 1000.0) | (particle_sim.x <= 0.0) {
            particle_sim.bearing = (2.0 * PI) - particle_sim.bearing;
            println!("{}", particle_sim.bearing)
        }
        if (particle_sim.y >= 1000.0) | (particle_sim.y <= 0.0) {
            if particle_sim.bearing <= 180.0 {
                particle_sim.bearing = PI - particle_sim.bearing;
            } else {
                particle_sim.bearing = (2.0 * PI) - particle_sim.bearing;
            }
        }
        
        
        /*
        _ = io::stdin().read_line(&mut result);
        if result.trim() == "x" {
            done = true;
        }
        */
    });
    
    main_window.run().unwrap();
}

slint::slint! {
    
    component Vis_Particle inherits Rectangle {
        
        in-out property <length> x_pos;
        in-out property <length> y_pos;
        in property <length> parent_height;
        
        width: 20px;
        height: 20px;
        x: (x-pos - (self.width / 2));
        y: parent-height - (y_pos - (self.height / 2));
        background: black;
    }
    
    struct Particles {
        x: length,
        y: length,
    }
    
    export component MainWindow inherits Window {
        height: 1000px;
        width: 1000px;
        background: white;
        
        in-out property <[Particles]> particles;
        
        for i in 10: particles := Vis_Particle {
            x_pos: 0;
            y_pos: 0;
            parent_height: parent.height;
        }
        
    }
}