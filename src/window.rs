use std::sync::mpsc::Receiver;

use crate::math::vector::*;

pub type Event = glfw::WindowEvent;

pub struct Window {
    pub handle: glfw::Window,
    events: Receiver<(f64, Event)>,
}

impl Window {
    pub fn new() -> Self {
        let init = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init system");
        let (handle, events) = init
            .create_window(800, 600, "Window", glfw::WindowMode::Windowed)
            .expect("Failed to create window");
        Self { handle, events }
    }

    pub fn set_size<T: Into<Vec2i>>(mut self, size: T) -> Self {
        let size = size.into();
        self.handle.set_size(size[X], size[Y]);
        self
    }

    pub fn set_pos<T: Into<Vec2i>>(mut self, pos: T) -> Self {
        let pos = pos.into();
        self.handle.set_pos(pos[X], pos[Y]);
        self
    }

    pub fn is_open(&self) -> bool {
        self.handle.should_close() == false
    }
}

impl<'a> IntoIterator for &'a mut Window {
    type Item = (f64, Event);

    type IntoIter = EventIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.handle.glfw.poll_events();
        EventIter {
            events: &mut self.events,
        }
    }
}

pub struct EventIter<'a> {
    events: &'a mut Receiver<(f64, Event)>,
}

impl Iterator for EventIter<'_> {
    type Item = (f64, Event);

    fn next(&mut self) -> Option<Self::Item> {
        self.events.try_recv().ok()
    }
}

#[test]
fn window_test() {
    let mut window = Window::new();
    while window.is_open() {
        for event in &mut window.into_iter() {
            match event.1 {
                Event::MouseButton(mb, a, md) => {}
                Event::CursorPos(x, y) => {
                    println!("x: {}, y:{}", x, y);
                }
                _ => {}
            }
        }
    }
}
