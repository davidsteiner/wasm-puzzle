pub const BOARD_SIZE: f64 = 1000.0;

pub struct Board {
    tiles: Vec<Vec<Option<Tile>>>,
}

impl Board {
    pub fn new(size: i8) -> Board {
        let mut tiles = Vec::new();
        for x in 0..size - 1 {
            let mut row: Vec<Option<Tile>> = Vec::new();
            for y in 0..size - 1 {
                let tile = Tile {
                    x,
                    y,
                    label: format!("{}", (x + (size - 1) * y) + 1),
                };
                row.push(Some(tile));
            }
            row.push(None);
            tiles.push(row);
        }
        let empty_row = std::iter::repeat_with(|| None)
            .take(size as usize)
            .collect();
        tiles.push(empty_row);

        Board { tiles }
    }

    pub fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let tile_size = BOARD_SIZE / (self.tiles.len() as f64);

        ctx.set_fill_style(&"rgb(255,255,255)".into());
        ctx.fill_rect(0.0, 0.0, BOARD_SIZE, BOARD_SIZE);

        for row in &self.tiles {
            for tile in row {
                if let Some(t) = tile {
                    t.render(ctx, tile_size);
                }
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

        ctx.set_fill_style(&"rgb(0,30,80)".into());
        ctx.fill_rect(x * size, y * size, size, size);
        ctx.stroke_rect(x * size, y * size, size, size);

        ctx.set_stroke_style(&"rgb(255,255,255)".into());
        ctx.stroke_text(&self.label, (x + 0.5) * size, (y + 0.5) * size)
            .unwrap();
    }
}
