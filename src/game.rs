use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Object;


#[wasm_bindgen]
extern "C" {
    pub type Memory;

    #[wasm_bindgen(method, getter)]
    pub fn role(this: &Memory) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_role(this: &Memory, role: Option<&str>);
}

impl Memory {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}


#[wasm_bindgen]
pub enum SpawnError {
    Busy,
    NotEnoughEnergy,
    NotEnoughRcl,
}

impl SpawnError {
    pub fn from_code(code: i32) -> Result<(), Self> {
        match code {
            0 => Ok(()),
            -4 => Err(Self::Busy),
            -6 => Err(Self::NotEnoughEnergy),
            -14 => Err(Self::NotEnoughRcl),
            code => panic!("Invalid code {}", code),
        }
    }
}


#[wasm_bindgen]
extern "C" {
    pub type Spawn;

    #[wasm_bindgen(method, js_name = spawnCreep)]
    fn spawn_creep_(this: &Spawn) -> i32;
}

impl Spawn {
    pub fn spawn_creep(&self) -> Result<(), SpawnError> {
        SpawnError::from_code(self.spawn_creep_())
    }
}


#[wasm_bindgen]
extern "C" {
    pub type Creep;

    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &Creep) -> Memory;
}


#[wasm_bindgen]
extern "C" {
    pub type Game;

    pub static Game: Game;

    #[wasm_bindgen(method, getter)]
    pub fn creeps(this: &Game) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn structures(this: &Game) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn spawns(this: &Game) -> Object;
}
