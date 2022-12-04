use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Button {
    A,
    B,
    Select,
    Start,
    Right,
    Down,
    Left,
    Up,
}

pub struct Input {
    key_states: HashMap<Button, bool>,
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
    keys
}

impl Input {
    pub fn new() -> Input {
        let mut keys = HashMap::new();
        keys.insert(Button::A, false);
        keys.insert(Button::B, false);
        keys.insert(Button::Select, false);
        keys.insert(Button::Start, false);
        keys.insert(Button::Up, false);
        keys.insert(Button::Left, false);
        keys.insert(Button::Right, false);
        keys.insert(Button::Down, false);

        Input {
            key_states: keys,
            mapping: get_default_config(),
        }
    }

    fn set_button(&mut self, key: Option<Keycode>, is_down: bool) {
        let k = key.unwrap();
        let b = *self.mapping.get(&k).unwrap();
        self.key_states.insert(b, is_down);
    }

    pub fn consume_keys(&mut self, event: Event) {
        match event {
            Event::KeyDown { keycode, .. } => self.set_button(keycode, true),
            Event::KeyUp { keycode, .. } => self.set_button(keycode, false),

            _ => {}
        };
    }
}
