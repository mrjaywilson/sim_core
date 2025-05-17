use std::sync::Mutex;
use once_cell::sync::Lazy;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum InputDirection {
    None = 0,
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

struct Tick {
    input: InputDirection,
    position: Vec2,
}

struct Simulation {
    state: Vec2,
    ticks: Vec<Tick>,
}

static SIM: Lazy<Mutex<Simulation>> = Lazy::new(|| {
   Mutex::new(Simulation {
       state: Vec2 { x: 0.0, y: 0.0 },
       ticks: Vec::new(),
   }) 
});

#[no_mangle]
pub extern "C" fn reset_simulation() {
    let mut sim = SIM.lock().unwrap();
    sim.state = Vec2 { x: 0.0, y: 0.0 };
    sim.ticks.clear();
}

#[no_mangle]
pub extern "C" fn advance_tick(direction: InputDirection) {
    let delta = match direction {
        InputDirection::Up => Vec2 { x: 0.0, y: 1.0 },
        InputDirection::Down => Vec2 { x: 0.0, y: -1.0 },
        InputDirection::Left => Vec2 { x: -1.0, y: 0.0 },
        InputDirection::Right => Vec2 { x: 1.0, y: 0.0 },
        InputDirection::None => Vec2 { x: 0.0, y: 0.0 },
    };

    let mut sim = SIM.lock().unwrap();
    sim.state.x += delta.x;
    sim.state.y += delta.y;

    let current_position = sim.state;

    sim.ticks.push(Tick {
        input: direction,
        position: current_position,
    })
}

#[no_mangle]
pub extern "C" fn get_position() -> Vec2 {
    let sim = SIM.lock().unwrap();
    sim.state
}

#[no_mangle]
pub extern "C" fn get_tick_count() -> usize {
    let sim = SIM.lock().unwrap();
    sim.ticks.len()
}

#[no_mangle]
pub extern "C" fn get_position_at_tick(index: usize) -> Vec2 {
    let sim = SIM.lock().unwrap();

    if index < sim.ticks.len() {
        sim.ticks[index].position
    } else {
        sim.state
    }
}