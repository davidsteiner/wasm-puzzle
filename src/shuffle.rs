use crate::utils::Direction;
use js_sys::Math::random;
use std::collections::VecDeque;

pub fn shuffle(grid_size: i8, steps: i8) -> VecDeque<ShiftAction> {
    let mut actions: VecDeque<ShiftAction> = VecDeque::new();

    let mut horizontal_state = vec![false; grid_size as usize];
    let mut vertical_state = vec![false; grid_size as usize];
    let mut last_idx: u8 = 0;

    for _ in 0..steps {
        let is_row = random() > 0.5;
        let idx: u8 = {
            let r = randint(1, grid_size as u8);
            // Ensure that we are not picking the same index as last time
            if r >= last_idx {
                r + 1
            } else {
                r
            }
        };
        last_idx = idx;
        let vec_idx = (idx as usize) - 1;

        let direction = if is_row {
            if horizontal_state[vec_idx] {
                horizontal_state[vec_idx] = false;
                Direction::West
            } else {
                horizontal_state[vec_idx] = true;
                Direction::East
            }
        } else {
            if vertical_state[vec_idx] {
                vertical_state[vec_idx] = false;
                Direction::North
            } else {
                vertical_state[vec_idx] = true;
                Direction::South
            }
        };

        actions.push_back(ShiftAction::create(idx, direction));
    }

    // Arrange everything north/west to make the initial board prettier
    for (idx, is_east) in horizontal_state.iter().enumerate() {
        if *is_east {
            actions.push_back(ShiftAction::create((idx + 1) as u8, Direction::West));
        }
    }
    for (idx, is_south) in vertical_state.iter().enumerate() {
        if *is_south {
            actions.push_back(ShiftAction::create((idx + 1) as u8, Direction::North));
        }
    }

    actions
}

pub struct ShiftAction {
    pub current_offset: f64,
    pub remaining_time: f64,
    pub idx: u8,
    pub direction: Direction,
}

impl ShiftAction {
    fn create(idx: u8, direction: Direction) -> ShiftAction {
        ShiftAction {
            current_offset: 0.0,
            remaining_time: 500.0,
            idx: idx,
            direction: direction,
        }
    }
}

fn randint(from: u8, to: u8) -> u8 {
    let r = ((to - from) as f64 * random()) + from as f64;
    r as u8
}
