use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

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

#[derive(Copy, Clone)]
struct TickInput {
    tick: usize,
    entity_id: u32,
    direction: InputDirection,
}

struct TickState {
    tick: usize,
    positions: HashMap<u32, Vec2>,
}

struct Simulation {
    current_tick: usize,
    positions: HashMap<u32, Vec2>,
    history: Vec<TickState>,
    inputs: Vec<TickInput>,
}

static SIM: Lazy<Mutex<Simulation>> = Lazy::new(|| {
   Mutex::new(Simulation {
       current_tick: 0,
       positions: HashMap::new(),
       history: Vec::new(),
       inputs: Vec::new(),
   }) 
});

#[no_mangle]
pub extern "C" fn reset_simulation() {
    let mut sim = SIM.lock().unwrap();
    sim.current_tick = 0;
    sim.positions.clear();
    sim.history.clear();
    sim.inputs.clear();
}

#[no_mangle]
pub extern "C" fn register_entity(entity_id: u32, x: f32, y: f32) {
    let mut sim = SIM.lock().unwrap();
    sim.positions.insert(entity_id, Vec2 { x, y });
}

#[no_mangle]
pub extern "C" fn advance_tick(entity_id: u32, direction: InputDirection) {
    let delta = match direction {
        InputDirection::Up => Vec2 { x: 0.0, y: 1.0 },
        InputDirection::Down => Vec2 { x: 0.0, y: -1.0 },
        InputDirection::Left => Vec2 { x: -1.0, y: 0.0 },
        InputDirection::Right => Vec2 { x: 1.0, y: 0.0 },
        InputDirection::None => Vec2 { x: 0.0, y: 0.0 },
    };

    let mut sim = SIM.lock().unwrap();

    if let Some(position) = sim.positions.get_mut(&entity_id) {
        position.x = delta.x;
        position.y = delta.y;
    }
    
    let current_tick = sim.current_tick;
    let positions = sim.positions.clone();

    sim.inputs.push(TickInput {
        tick: current_tick,
        entity_id,
        direction,
    });

    sim.history.push(TickState {
       tick: current_tick,
        positions,
    });

    sim.current_tick += 1;
}

#[no_mangle]
pub extern "C" fn get_position(entity_id: u32) -> Vec2 {
    let sim = SIM.lock().unwrap();
    sim.positions.get(&entity_id).copied().unwrap_or(Vec2 { x: 0.0, y: 0.0 })
}

#[no_mangle]
pub extern "C" fn get_tick_count() -> usize {
    let sim = SIM.lock().unwrap();
    sim.history.len()
}

#[no_mangle]
pub extern "C" fn get_position_at_tick(entity_id: u32, index: usize) -> Vec2 {
    let sim = SIM.lock().unwrap();

    if index < sim.history.len() {
        let tick_state = &sim.history[index];
        
        tick_state.positions.get(&entity_id).copied().unwrap_or(Vec2 { x: 0.0, y: 0.0 })
    } else {
        sim.positions.get(&entity_id).copied().unwrap_or(Vec2 { x: 0.0, y: 0.0 })
    }
}