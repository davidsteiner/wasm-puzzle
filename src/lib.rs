mod event_manager;
mod utils;

use event_manager::{EventManager, MouseEvent};
use std::cell::RefCell;
use std::rc::Rc;
use utils::window;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WORLD_SIZE: f64 = 1000.0;

#[wasm_bindgen]
pub struct Game {
    event_manager: EventManager,
}

impl Game {
    fn create(event_manager: EventManager) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let game = Game { event_manager };
        let mut current_time = 0.0;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
            let dt = time - current_time;
            current_time = time;

            game.update(dt);

            loop {
                match game.event_manager.pop_event() {
                    Some(ev) => game.process_event(ev),
                    None => break,
                };
            }
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(_)>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn update(&self, dt: f64) {
        let _ = dt;
    }

    fn process_event(&self, event: MouseEvent) {
        match event {
            MouseEvent::Up(c) => utils::log(&format!("Mouse up ({}, {})", c.x, c.y)),
            MouseEvent::Down(c) => utils::log(&format!("Mouse down ({}, {})", c.x, c.y)),
        }
    }
}

#[wasm_bindgen]
pub fn setup_game() {
    utils::set_panic_hook();
    let (canvas, _) = get_context("kirako-canvas");
    let event_manager = EventManager::new(canvas.unchecked_into::<web_sys::HtmlElement>());

    Game::create(event_manager);
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn get_context(
    canvas_id: &str,
) -> (
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_font("30px Arial");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    let scale_x = canvas.width() as f64 / WORLD_SIZE;
    let scale_y = canvas.height() as f64 / WORLD_SIZE;

    ctx.scale(scale_x, scale_y).unwrap();

    return (canvas, ctx);
}
