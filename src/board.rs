use crate::utils::Coordinate;

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

        for x in 1..size + 1 {
            // Create number tiles
            for y in 1..size + 1 {
                let tile = Tile {
                    x,
                    y,
                    label: format!("{}", (x + size * (y - 1))),
                };
                tiles.push(tile);
            }
            // Create empty tiles
            tiles.push(Tile {
                x: x,
                y: 0,
                label: "".to_string(),
            });
            tiles.push(Tile {
                x: 0,
                y: x,
                label: "".to_string(),
            });
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

    pub fn shift(&mut self, origin: (f64, f64), vec: &Coordinate) {
        let tile_size = self.tile_size();
        let col = (origin.0 / tile_size) as i8;
        let row = (origin.1 / tile_size) as i8;

        match Direction::from_coordinate(vec) {
            Direction::North => self.shift_north(col),
            Direction::West => self.shift_west(row),
            Direction::South => self.shift_south(col),
            Direction::East => self.shift_east(row),
        }
    }

    fn tile_size(&self) -> f64 {
        BOARD_SIZE / ((self.size + 2) as f64)
    }

    fn get_row(&mut self, row: i8) -> Option<Vec<&mut Tile>> {
        if row > 0 && row < self.size + 1 {
            Some(self.tiles.iter_mut().filter(|t| t.y == row).collect())
        } else {
            None
        }
    }

    fn get_col(&mut self, col: i8) -> Option<Vec<&mut Tile>> {
        if col > 0 && col < self.size + 1 {
            Some(self.tiles.iter_mut().filter(|t| t.x == col).collect())
        } else {
            None
        }
    }

    fn shift_north(&mut self, col: i8) {
        if let Some(tiles) = self.get_col(col) {
            for t in &tiles {
                if t.y == 0 {
                    return;
                }
            }
            for t in tiles {
                t.y -= 1;
            }
        }
    }

    fn shift_south(&mut self, col: i8) {
        let size;
        {
            size = self.size;
        }

        if let Some(tiles) = self.get_col(col) {
            for t in &tiles {
                if t.y == size + 1 {
                    return;
                }
            }
            for t in tiles {
                t.y += 1;
            }
        }
    }

    fn shift_east(&mut self, row: i8) {
        let size;
        {
            size = self.size;
        }

        if let Some(tiles) = self.get_row(row) {
            for t in &tiles {
                if t.x == size + 1 {
                    return;
                }
            }
            for t in tiles {
                t.x += 1;
            }
        }
    }

    fn shift_west(&mut self, row: i8) {
        if let Some(tiles) = self.get_row(row) {
            for t in &tiles {
                if t.x == 0 {
                    return;
                }
            }
            for t in tiles {
                t.x -= 1;
            }
        }
    }
}

pub struct Tile {
    x: i8,
    y: i8,
    label: String,
}

impl Tile {
    fn render(&self, ctx: &web_sys::CanvasRenderingContext2d, size: f64) {
        let x = self.x as f64;
        let y = self.y as f64;

        if self.label.is_empty() {
            ctx.set_fill_style(&EMPTY_TILE_COLOUR.into());
        } else {
            ctx.set_fill_style(&TILE_COLOUR.into());
        }
        ctx.fill_rect(x * size, y * size, size, size);
        ctx.stroke_rect(x * size, y * size, size, size);

        ctx.set_stroke_style(&"rgb(255,255,255)".into());
        ctx.stroke_text(&self.label, (x + 0.5) * size, (y + 0.5) * size)
            .unwrap();
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn from_coordinate(c: &Coordinate) -> Direction {
        if c.x.abs() > c.y.abs() {
            if c.x < 0 {
                Direction::West
            } else {
                Direction::East
            }
        } else {
            if c.y < 0 {
                Direction::North
            } else {
                Direction::South
            }
        }
    }
}
