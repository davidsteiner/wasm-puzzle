mod board;
mod event_manager;
mod utils;

use event_manager::{EventManager, MouseEvent};
use std::cell::RefCell;
use std::rc::Rc;
use utils::{window, Point};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    event_manager: EventManager,
    ctx: web_sys::CanvasRenderingContext2d,
    board: board::Board,
    game_state: GameState,
}

enum GameState {
    Idle,
    Dragging(Point<i32>),
}

impl Game {
    fn create(event_manager: EventManager, ctx: web_sys::CanvasRenderingContext2d) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut game = Game {
            event_manager,
            ctx,
            board: board::Board::new(3),
            game_state: GameState::Idle,
        };
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

        self.render()
    }

    fn render(&self) {
        self.board.render(&self.ctx);
    }

    fn process_event(&mut self, event: MouseEvent) {
        match self.game_state {
            GameState::Idle => {
                if let MouseEvent::Down(point) = event {
                    self.game_state = GameState::Dragging(point);
                }
            }
            GameState::Dragging(from) => match event {
                MouseEvent::Move(to) => self.process_dragging(from, to),
                MouseEvent::Up(to) => self.process_drag_over(from, to),
                MouseEvent::Down(to) => self.process_drag_over(from, to),
            },
        }
    }

    fn process_dragging(&mut self, from: Point<i32>, to: Point<i32>) {
        self.board.shift(
            &self.to_board_point(&from),
            &self.to_board_point(&to),
            false,
        );
    }

    fn process_drag_over(&mut self, from: Point<i32>, to: Point<i32>) {
        self.board
            .shift(&self.to_board_point(&from), &self.to_board_point(&to), true);
        self.game_state = GameState::Idle;
    }

    fn to_board_point(&self, point: &Point<i32>) -> Point<f64> {
        let element: web_sys::HtmlElement = self.ctx.canvas().unwrap().unchecked_into();
        let canvas_rect = element.get_bounding_client_rect();

        // Calculate the origin coordinates in the boards coordinate system
        let x = ((point.x as f64 - canvas_rect.x()) / canvas_rect.width()) * board::BOARD_SIZE;
        let y = ((point.y as f64 - canvas_rect.y()) / canvas_rect.height()) * board::BOARD_SIZE;

        Point { x, y }
    }
}

#[wasm_bindgen]
pub fn setup_game() {
    utils::set_panic_hook();
    let (canvas, ctx) = get_context("kirako-canvas");
    let event_manager = EventManager::new(canvas.unchecked_into::<web_sys::HtmlElement>());

    Game::create(event_manager, ctx);
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

    ctx.set_font("64px Arial");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");

    let scale_x = canvas.width() as f64 / board::BOARD_SIZE;
    let scale_y = canvas.height() as f64 / board::BOARD_SIZE;

    ctx.scale(scale_x, scale_y).unwrap();

    return (canvas, ctx);
}
