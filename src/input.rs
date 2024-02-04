use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Right,
    Down,
    Left,
    Up,

    Reset,
    DumpBgTiles,
    Step,
    ToggleStepping,
    Continue,

    ToggleBackground,
    ToggleWindow,
    ToggleObjects,
}

pub struct Input {
    key_states: HashMap<Button, bool>,
    prev_key_states: HashMap<Button, bool>,
    mapping: HashMap<Keycode, Button>,
}
fn get_default_config() -> HashMap<Keycode, Button> {
    let mut keys = HashMap::new();
    keys.insert(Keycode::Left, Button::Left);
    keys.insert(Keycode::Up, Button::Up);
    keys.insert(Keycode::Right, Button::Right);
    keys.insert(Keycode::Down, Button::Down);
    keys.insert(Keycode::Z, Button::B);
    keys.insert(Keycode::X, Button::A);
    keys.insert(Keycode::RShift, Button::Select);
    keys.insert(Keycode::Return, Button::Start);

    keys.insert(Keycode::R, Button::Reset);
    keys.insert(Keycode::D, Button::DumpBgTiles);
    keys.insert(Keycode::F8, Button::Step);
    keys.insert(Keycode::F9, Button::Continue);
    keys.insert(Keycode::F6, Button::ToggleStepping);

    keys.insert(Keycode::F1, Button::ToggleBackground);
    keys.insert(Keycode::F2, Button::ToggleWindow);
    keys.insert(Keycode::F3, Button::ToggleObjects);

    keys
}

impl Input {
    pub fn new() -> Input {
        let mut keys = HashMap::new();
        // Controller
        keys.insert(Button::A, false);
        keys.insert(Button::B, false);
        keys.insert(Button::Select, false);
        keys.insert(Button::Start, false);
        keys.insert(Button::Up, false);
        keys.insert(Button::Left, false);
        keys.insert(Button::Right, false);
        keys.insert(Button::Down, false);

        //System
        keys.insert(Button::Reset, false);
        keys.insert(Button::DumpBgTiles, false);
        keys.insert(Button::Step, false);
        keys.insert(Button::Continue, false);
        keys.insert(Button::ToggleStepping, false);

        keys.insert(Button::ToggleBackground, false);
        keys.insert(Button::ToggleWindow, false);
        keys.insert(Button::ToggleObjects, false);

        let mut i = Input {
            key_states: keys,
            prev_key_states: HashMap::new(),
            mapping: get_default_config(),
        };
        i.set_prev_keys();
        i
    }

    pub fn is_down(&self, b: &Button) -> bool {
        *self.key_states.get(b).unwrap()
    }
    pub fn is_new_down(&self, b: &Button) -> bool {
        let key_down = *self.key_states.get(b).unwrap();
        let prev_key_down = *self.prev_key_states.get(b).unwrap();
        key_down && !prev_key_down
    }
    fn set_button(&mut self, key: Option<Keycode>, is_down: bool) {
        let k = key.unwrap();
        match self.mapping.get(&k) {
            Some(b) => {
                let current = self.key_states.get(b).unwrap();
                self.prev_key_states.insert(*b,*current);
                self.key_states.insert(*b, is_down);
            }
            None => {}
        }
    }
    fn set_prev_keys(&mut self) {
        for state in &self.key_states {
            self.prev_key_states.insert(*state.0, *state.1);
        }
    }

    pub fn consume_keys(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode, .. } => self.set_button(keycode, true),
            Event::KeyUp { keycode, .. } => self.set_button(keycode, false),
            _ => {}
        };
    }
}
