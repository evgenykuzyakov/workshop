use crate::*;

#[near_bindgen]
impl Contract {
    /// Renders a solid color filled rectangle.
    /// To call this method you need to call `render_rect`, e.g.
    /// `./call.sh render_rect '{"left": 10, "top": 20, "width": 10, "height": 5, "color": 16711680}'`
    pub fn render_rect(
        &mut self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        color: u32,
    ) -> Promise {
        let pixels = internal_render_rect(left, top, width, height, color);
        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }
}

pub(crate) fn internal_render_rect(
    left: u32,
    top: u32,
    width: u32,
    height: u32,
    color: u32,
) -> Vec<SetPixelRequest> {
    // Optionally check the validity of the input.
    assert!(left < BOARD_WIDTH, "left is out of bounds [0..49]");
    assert!(top < BOARD_HEIGHT, "top is out of bounds [0..49]");
    assert!(color <= 0xffffff, "color is out of bounds [0..0xffffff]");
    assert!(
        width > 0 && width <= BOARD_WIDTH,
        "width is out of bounds [1..50]"
    );
    assert!(
        height > 0 && height <= BOARD_HEIGHT,
        "height is out of bounds [1..50]"
    );
    assert!(
        left + width <= BOARD_WIDTH,
        "left + top is out of bounds [0..49]"
    );
    assert!(
        top + height <= BOARD_HEIGHT,
        "top + height is out of bounds [0..49]"
    );

    // Creates an empty vector
    let mut pixels = vec![];

    // Iterate over pixels of the rectangle.
    for i in top..top + width {
        for j in left..left + height {
            let pixel = SetPixelRequest { x: i, y: j, color };
            // Verify that the pixel request is valid.
            // NOTE: That this check is not necessary, because we've verified the input above.
            pixel.assert_valid();

            pixels.push(pixel);
        }
    }

    pixels
}

#[cfg(not(target = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rect_small() {
        let pixels = internal_render_rect(1, 1, 2, 1, 0xffffff);
        let board = debug_print_pixels(&pixels);
        assert_eq!(&board[0][..15], "...............", "Line 0");
        assert_eq!(&board[1][..15], ".XX............", "Line 1");
        assert_eq!(&board[2][..15], "...............", "Line 2");
    }

    #[test]
    pub fn test_rect_complex() {
        let pixels = internal_render_rect(3, 5, 4, 4, 0xffffff);
        let board = debug_print_pixels(&pixels);
        assert_eq!(&board[0][..15], "...............", "Line 0");
        assert_eq!(&board[1][..15], "...............", "Line 1");
        assert_eq!(&board[2][..15], "...............", "Line 2");
        assert_eq!(&board[3][..15], "...............", "Line 3");
        assert_eq!(&board[4][..15], "...............", "Line 4");
        assert_eq!(&board[5][..15], "...XXXX........", "Line 5");
        assert_eq!(&board[6][..15], "...XXXX........", "Line 6");
        assert_eq!(&board[7][..15], "...XXXX........", "Line 7");
        assert_eq!(&board[8][..15], "...XXXX........", "Line 8");
        assert_eq!(&board[9][..15], "...............", "Line 9");
        assert_eq!(pixels.len(), 4 * 4);
    }
}
