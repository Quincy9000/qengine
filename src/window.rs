use std::sync::mpsc::Receiver;

use glfw::{Context, FlushedMessages};

use crate::math::vector::*;

pub type Event = glfw::WindowEvent;

pub struct Window {
    handle: glfw::Window,
    events: Receiver<(f64, Event)>,
}

impl Window {
    pub fn new() -> Self {
        let init = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init system");
        let (mut handle, events) = init
            .create_window(800, 600, "Window", glfw::WindowMode::Windowed)
            .expect("Failed to create window");
        handle.make_current();
        handle.set_all_polling(true);
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

    pub fn draw(&mut self) {
        self.handle.swap_buffers()
    }

    pub fn poll(&mut self) -> EventIter {
        self.handle.glfw.poll_events();
        let iter = glfw::flush_messages(&self.events);
        EventIter { events: iter }
    }
}

pub struct EventIter<'a> {
    events: FlushedMessages<'a, (f64, Event)>,
}

impl Iterator for EventIter<'_> {
    type Item = (f64, Event);

    fn next(&mut self) -> Option<Self::Item> {
        self.events.next()
    }
}

#[test]
fn window_test() {
    let mut window = Window::new();
    while window.is_open() {
        let mut events = window.poll();
        for event in events {
            match event.1 {
                Event::MouseButton(mb, a, md) => {
                    println!("{}", "button pressed")
                }
                Event::CursorPos(x, y) => {
                    println!("x: {}, y:{}", x, y);
                }
                _ => {}
            }
        }
        window.draw();
    }
}
