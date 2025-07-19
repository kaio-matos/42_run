use std::collections::HashMap;

use crate::{graphics::window::KeyEvent, prelude::*};

#[derive(Debug)]
pub struct EventHandler {
    pub hold_keys: HashMap<KeyEvent, bool>,
    pub events: Vec<glfw::WindowEvent>,
}
impl Resource for EventHandler {}
impl EventHandler {
    pub fn new() -> Self {
        Self {
            hold_keys: HashMap::new(),
            events: Vec::new(),
        }
    }

    pub fn update(&mut self, events: Vec<glfw::WindowEvent>) {
        self.events.clear();
        self.events = events;
    }

    pub fn process(&mut self) {
        for event in self.events.iter() {
            if let glfw::WindowEvent::Key(k, _, action, m) = *event {
                let key = KeyEvent {
                    key: k,
                    modifiers: m,
                };

                if action == glfw::Action::Press {
                    self.hold_keys.insert(key, true);
                } else if action == glfw::Action::Release {
                    // make sure we release all of them, even if the key was pressed by using a
                    // modifier
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::empty(),
                        },
                        false,
                    );
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::Alt,
                        },
                        false,
                    );
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::Shift,
                        },
                        false,
                    );
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::Control,
                        },
                        false,
                    );
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::Super,
                        },
                        false,
                    );
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::NumLock,
                        },
                        false,
                    );
                    self.hold_keys.insert(
                        KeyEvent {
                            key: k,
                            modifiers: glfw::Modifiers::CapsLock,
                        },
                        false,
                    );
                    self.hold_keys.insert(key, false);
                }
            }
            // match event {
            //     glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            //         gl::Viewport(0, 0, width, height)
            //     },
            //     glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
            //         self.window_handle.set_should_close(true)
            //     }
            //     _ => {}
            // }
        }
    }

    pub fn mouse_move(&self) -> Option<(f64, f64)> {
        for event in self.events.iter() {
            if let glfw::WindowEvent::CursorPos(x, y) = event {
                return Some((*x, *y));
            }
        }
        None
    }

    pub fn hold(&self, key: glfw::Key, modifiers: glfw::Modifiers) -> bool {
        let is_pressing = self.hold_keys.get(&KeyEvent { key, modifiers });

        match is_pressing {
            Some(value) => *value,
            None => false,
        }
    }

    pub fn released(&self, key: glfw::Key, modifiers: glfw::Modifiers) -> bool {
        let found = self.events.iter().find(|event| {
            if let glfw::WindowEvent::Key(k, _, action, m) = event {
                if k == &key && m == &modifiers && action == &glfw::Action::Release {
                    return true;
                }
            }
            false
        });
        found.is_some()
    }

    pub fn pressed(&self, key: glfw::Key, modifiers: glfw::Modifiers) -> bool {
        let found = self.events.iter().find(|event| {
            if let glfw::WindowEvent::Key(k, _, action, m) = event {
                if k == &key && m == &modifiers && action == &glfw::Action::Press {
                    return true;
                }
            }
            false
        });
        found.is_some()
    }
}
