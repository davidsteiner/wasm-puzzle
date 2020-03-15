use crate::utils::Point;

pub const BOARD_SIZE: f64 = 1000.0;

const BACKGROUND_COLOUR: &str = "rgb(150,200,255)";
const TILE_COLOUR: &str = "rgb(40,130,0)";
const EMPTY_TILE_COLOUR: &str = "rgb(120,0,20)";
const CORNER_COLOUR: &str = "rgb(20,20,60)";
const BORDER_ALPHA: f64 = 0.2;

pub struct Board {
    tiles: Vec<Tile>,
    size: i8,
}

impl Board {
    pub fn new(size: i8) -> Board {
        let mut tiles = Vec::new();
        let tile_size = BOARD_SIZE / ((size + 2) as f64);

        for x in 1..size + 1 {
            // Create number tiles
            for y in 1..size + 1 {
                let label = format!("{}", (x + size * (y - 1)));
                let tile = Tile::create(x, y, label, tile_size);
                tiles.push(tile);
            }
            // Create empty tiles
            tiles.push(Tile::create(x, 0, "".to_string(), tile_size));
            tiles.push(Tile::create(0, x, "".to_string(), tile_size));
        }

        Board { tiles, size }
    }

    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let tile_size = self.tile_size();
        ctx.set_fill_style(&BACKGROUND_COLOUR.into());
        ctx.fill_rect(0.0, 0.0, BOARD_SIZE, BOARD_SIZE);

        for tile in &self.tiles {
            tile.render(ctx, tile_size);
        }

        // Add shading to the border tiles
        ctx.set_fill_style(&"rgb(0,0,0".into());
        let current_alpha = ctx.global_alpha();
        ctx.set_global_alpha(BORDER_ALPHA);
        ctx.fill_rect(tile_size, 0.0, BOARD_SIZE - tile_size * 2.0, tile_size); // top
        ctx.fill_rect(0.0, tile_size, tile_size, BOARD_SIZE - tile_size * 2.0); // left
        ctx.fill_rect(
            BOARD_SIZE - tile_size,
            tile_size,
            tile_size,
            BOARD_SIZE - tile_size * 2.0,
        ); // right
        ctx.fill_rect(
            tile_size,
            BOARD_SIZE - tile_size,
            BOARD_SIZE - 2.0 * tile_size,
            tile_size,
        ); // bottom
        ctx.set_global_alpha(current_alpha);

        // Add corners
        ctx.set_fill_style(&CORNER_COLOUR.into());
        ctx.fill_rect(0.0, 0.0, tile_size, tile_size);
        ctx.fill_rect(0.0, BOARD_SIZE - tile_size, tile_size, tile_size);
        ctx.fill_rect(BOARD_SIZE - tile_size, 0.0, tile_size, tile_size);
        ctx.fill_rect(
            BOARD_SIZE - tile_size,
            BOARD_SIZE - tile_size,
            tile_size,
            tile_size,
        );
    }

    pub fn shift(&mut self, from: &Point<f64>, to: &Point<f64>, end: bool) {
        let tile_size = self.tile_size();
        let shift_vector = self.get_shift_vector(from, to);
        let row = (from.y / tile_size) as i8;
        let col = (from.x / tile_size) as i8;

        // horizontal shift
        if shift_vector.x.abs() > shift_vector.y.abs() {
            let grid_distance: i8;
            let distance: f64;
            if end {
                grid_distance = self.grid_distance(shift_vector.x);
                distance = grid_distance as f64 * tile_size;
            } else {
                grid_distance = 0;
                distance = shift_vector.x.min(tile_size).max(-tile_size); // clamp on f64 is unstable
            };
            for t in self.get_row(row) {
                t.render_position.x = t.grid_position.x as f64 * tile_size + distance;
                t.grid_position.x += grid_distance;
            }
            for t in self.get_col(col) {
                t.render_position.y = t.grid_position.y as f64 * tile_size;
            }
        }
        // vertical shift
        else {
            let grid_distance: i8;
            let distance: f64;
            if end {
                grid_distance = self.grid_distance(shift_vector.y);
                distance = grid_distance as f64 * tile_size;
            } else {
                grid_distance = 0;
                distance = shift_vector.y.min(tile_size).max(-tile_size); // clamp on f64 is unstable
            };
            for t in self.get_row(row) {
                t.render_position.x = t.grid_position.x as f64 * tile_size;
            }
            for t in self.get_col(col) {
                t.render_position.y = t.grid_position.y as f64 * tile_size + distance;
                t.grid_position.y += grid_distance;
            }
        }
    }

    fn get_shift_vector(&mut self, from: &Point<f64>, to: &Point<f64>) -> Point<f64> {
        let tile_size = self.tile_size();
        let mut v = Point {
            x: to.x - from.x,
            y: to.y - from.y,
        };

        // Figure out which way we cannot move horizontally, and update the vector accordingly
        let row = (from.y / tile_size) as i8;
        if row > 0 && row < self.size + 1 {
            if self.get_row(row).iter().any(|t| t.grid_position.x == 0) {
                v.x = v.x.max(0.0); // If there is a tile in the left column, we cannot move left
            } else {
                v.x = v.x.min(0.0); // Otherwise, there must be a tile in the right column and we cannot move right
            }
        } else {
            // If we are in the first or last row, we cannot move horizontally at all
            v.x = 0.0;
        }

        // Figure out which way we cannot move vertically, and update the vector accordingly
        let col = (from.x / tile_size) as i8;
        if col > 0 && col < self.size + 1 {
            if self.get_col(col).iter().any(|t| t.grid_position.y == 0) {
                v.y = v.y.max(0.0); // If there is a tile in the top column, we cannot move up
            } else {
                v.y = v.y.min(0.0); // Otherwise, there must be a tile in the bottom column and we cannot move down
            }
        } else {
            // If we are in the first or last column, we cannot move vertically at all
            v.y = 0.0;
        }

        v
    }

    fn grid_distance(&self, distance: f64) -> i8 {
        let threshold = self.tile_size() / 2.0;
        if distance > threshold {
            1
        } else {
            if distance < -threshold {
                -1
            } else {
                0
            }
        }
    }

    pub fn tile_size(&self) -> f64 {
        BOARD_SIZE / ((self.size + 2) as f64)
    }

    fn get_row(&mut self, row: i8) -> Vec<&mut Tile> {
        self.tiles
            .iter_mut()
            .filter(|t| t.grid_position.y == row)
            .collect()
    }

    fn get_col(&mut self, col: i8) -> Vec<&mut Tile> {
        self.tiles
            .iter_mut()
            .filter(|t| t.grid_position.x == col)
            .collect()
    }
}

pub struct Tile {
    grid_position: Point<i8>,
    render_position: Point<f64>,
    label: String,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile({}, {}, label={})",
            self.grid_position.x, self.grid_position.y, &self.label
        )
    }
}

impl Tile {
    fn create(x: i8, y: i8, label: String, tile_size: f64) -> Tile {
        let render_position = Point {
            x: x as f64 * tile_size,
            y: y as f64 * tile_size,
        };
        Tile {
            grid_position: Point { x, y },
            render_position,
            label,
        }
    }

    fn render(&self, ctx: &web_sys::CanvasRenderingContext2d, size: f64) {
        let x = self.render_position.x;
        let y = self.render_position.y;

        if self.label.is_empty() {
            ctx.set_fill_style(&EMPTY_TILE_COLOUR.into());
        } else {
            ctx.set_fill_style(&TILE_COLOUR.into());
        }
        ctx.fill_rect(x, y, size, size);
        ctx.stroke_rect(x, y, size, size);

        ctx.set_stroke_style(&"rgb(255,255,255)".into());
        ctx.set_fill_style(&"rgb(255,255,255)".into());
        ctx.fill_text(&self.label, x + 0.5 * size, y + 0.5 * size)
            .unwrap();
        ctx.stroke_text(&self.label, x + 0.5 * size, y + 0.5 * size)
            .unwrap();
    }
}
