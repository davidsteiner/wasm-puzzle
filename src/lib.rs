mod board;
mod event_manager;
mod shuffle;
mod utils;

use event_manager::{EventManager, MouseEvent};
use shuffle::{shuffle, ShiftAction};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use utils::{window, Direction, Point};
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
    actions: VecDeque<ShiftAction>,
}

enum GameState {
    Idle,
    Dragging(Point<i32>),
    Processing,
}

impl Game {
    fn create(event_manager: EventManager, ctx: web_sys::CanvasRenderingContext2d) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let board_size = 3;
        let mut game = Game {
            event_manager,
            ctx,
            board: board::Board::new(board_size),
            game_state: GameState::Idle,
            actions: shuffle(board_size, 10),
        };
        let mut current_time = 0.0;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
            let dt = time - current_time;
            current_time = time;

            game.update(dt);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(_)>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn update(&mut self, dt: f64) {
        self.process_actions(dt);

        loop {
            match self.event_manager.pop_event() {
                Some(ev) => self.process_event(ev),
                None => break,
            };
        }

        self.render()
    }

    fn process_actions(&mut self, dt: f64) {
        if let Some(action) = self.actions.front_mut() {
            self.game_state = GameState::Processing;

            let tile_size = self.board.tile_size();
            action.current_offset +=
                ((dt / action.remaining_time) * (tile_size - action.current_offset)).min(tile_size);
            let from = match action.direction {
                Direction::North | Direction::South => Point {
                    x: ((action.idx as f64) + 0.5) * tile_size,
                    y: 1.5 * tile_size,
                },
                Direction::West | Direction::East => Point {
                    x: 1.5 * tile_size,
                    y: ((action.idx as f64) + 0.5) * tile_size,
                },
            };
            let to = from.add_direction(&action.direction, action.current_offset);
            action.remaining_time -= dt;

            if action.remaining_time < 0.0 {
                self.board.shift(&from, &to, true);
                self.actions.pop_front();
                if self.actions.is_empty() {
                    self.game_state = GameState::Idle;
                }
            } else {
                self.board.shift(&from, &to, false);
            }
        }
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
            _ => (),
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
