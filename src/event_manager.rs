use crate::utils::{log, window, Point};

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[allow(dead_code)]
pub struct EventManager {
    // These are referenced so that they are not dropped until the game is dropped
    onmousedown: Closure<dyn FnMut(web_sys::MouseEvent)>,
    onmouseup: Closure<dyn FnMut(web_sys::MouseEvent)>,
    onmousemove: Closure<dyn FnMut(web_sys::MouseEvent)>,
    event_target: web_sys::HtmlElement,
    event_queue: Rc<RefCell<VecDeque<MouseEvent>>>,
}

pub enum MouseEvent {
    Down(Point<i32>),
    Up(Point<i32>),
    Move(Point<i32>),
}

impl EventManager {
    pub fn new(event_target: web_sys::HtmlElement) -> EventManager {
        let rc_event_queue = Rc::new(RefCell::new(VecDeque::new()));

        let eq_mousedown = rc_event_queue.clone();
        let cb = move |e: web_sys::MouseEvent| {
            eq_mousedown.borrow_mut().push_back(MouseEvent::Down(Point {
                x: e.client_x(),
                y: e.client_y(),
            }));
        };
        let onmousedown = Closure::wrap(Box::new(cb) as Box<dyn FnMut(_)>);
        event_target.set_onmousedown(Some(onmousedown.as_ref().unchecked_ref()));

        let eq_mouseup = rc_event_queue.clone();
        let cb = move |e: web_sys::MouseEvent| {
            eq_mouseup.borrow_mut().push_back(MouseEvent::Up(Point {
                x: e.client_x(),
                y: e.client_y(),
            }));
        };
        let onmouseup = Closure::wrap(Box::new(cb) as Box<dyn FnMut(_)>);
        window().set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));

        let eq_mousemove = rc_event_queue.clone();
        let cb = move |e: web_sys::MouseEvent| {
            eq_mousemove.borrow_mut().push_back(MouseEvent::Move(Point {
                x: e.client_x(),
                y: e.client_y(),
            }));
        };
        let onmousemove = Closure::wrap(Box::new(cb) as Box<dyn FnMut(_)>);
        window().set_onmousemove(Some(onmousemove.as_ref().unchecked_ref()));

        log("Added event manager");

        Self {
            onmousedown,
            onmouseup,
            onmousemove,
            event_target,
            event_queue: rc_event_queue,
        }
    }

    pub fn pop_event(&self) -> Option<MouseEvent> {
        (*self.event_queue.borrow_mut()).pop_front()
    }
}

impl Drop for EventManager {
    fn drop(&mut self) {
        self.event_target.set_onmousedown(None);
        window().set_onmouseup(None);
        window().set_onmousemove(None);
        log("Dropping event manager");
    }
}
