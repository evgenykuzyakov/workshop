use crate::*;

#[near_bindgen]
impl Contract {
    /// Renders a circle with a line.
    /// To call this method you need to call `render_circle`, e.g.
    /// `render_circle '{"center_x": 10, "center_y": 20, "radius": 5, "color": 255}'`
    pub fn render_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: u32,
        color: u32,
    ) -> Promise {
        let pixels = internal_render_circle(center_x, center_y, radius, color);
        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }
}

pub(crate) fn internal_render_circle(
    center_x: i32,
    center_y: i32,
    radius: u32,
    color: u32,
) -> Vec<SetPixelRequest> {
    // Optionally check the validity of the input.
    assert!(color <= 0xffffff, "color is out of bounds [0..0xffffff]");

    // Creates an empty vector
    let mut pixels = vec![];

    let r2 = (radius * radius) as i32;
    let max_delta = radius as i32;

    // Iterate over pixels of the rectangle.
    for i in std::cmp::max(0, center_y - radius as i32)
        ..=std::cmp::min(BOARD_HEIGHT as i32 - 1, center_y + radius as i32)
    {
        for j in std::cmp::max(0, center_x - radius as i32)
            ..=std::cmp::min(BOARD_WIDTH as i32 - 1, center_x + radius as i32)
        {
            let dx = i - center_x;
            let dy = j - center_x;
            let d2 = dx * dx + dy * dy;
            if d2 >= r2 - max_delta && d2 <= r2 + max_delta {
                pixels.push(SetPixelRequest {
                    x: j as u32,
                    y: i as u32,
                    color,
                });
            }
        }
    }

    pixels
}

#[cfg(not(target = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_circle_small() {
        let pixels = internal_render_circle(4, 5, 3, 0xffffff);
        let board = debug_print_pixels(&pixels);
        assert_eq!(&board[0][..15], "...............");
        assert_eq!(&board[1][..15], "...............");
        assert_eq!(&board[2][..15], "...XXX.........");
        assert_eq!(&board[3][..15], "..X...X........");
        assert_eq!(&board[4][..15], ".X.....X.......");
        assert_eq!(&board[5][..15], ".X.....X.......");
        assert_eq!(&board[6][..15], ".X.....X.......");
        assert_eq!(&board[7][..15], "..X...X........");
        assert_eq!(&board[8][..15], "...XXX.........");
        assert_eq!(&board[9][..15], "...............");
    }
}
