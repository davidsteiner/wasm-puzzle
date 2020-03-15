use num::Num;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[derive(Copy, Clone, Debug)]
pub struct Point<T: Copy + Num> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Num> Point<T> {
    pub fn add_direction(&self, direction: &Direction, distance: T) -> Point<T> {
        match direction {
            Direction::North => Point {
                x: self.x,
                y: self.y - distance,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + distance,
            },
            Direction::West => Point {
                x: self.x - distance,
                y: self.y,
            },
            Direction::East => Point {
                x: self.x + distance,
                y: self.y,
            },
        }
    }
}

impl<T: Copy + Num> std::ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + Num> std::ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Num> std::ops::Div<T> for Point<T> {
    type Output = Point<T>;
    fn div(self, rhs: T) -> Point<T> {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

pub enum Direction {
    North,
    South,
    West,
    East,
}
