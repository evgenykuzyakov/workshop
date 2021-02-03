use crate::*;

#[near_bindgen]
impl Contract {
    /// This method requires the current board colors to compute then inverse colors.
    /// To call this method you need to call `render` and pass `method_name`, e.g.
    /// `render '{"method_name": "invert_rect", "left": 20, "top": 10, "width": 20, "height": 20}'`
    /// This is because you first need to get the lines from the berryclub contract.
    ///
    /// `[callback] lines: Vec<Base64VecU8>` is required to get the current state of the board.
    /// See `berryclub::render` for implementation details.
    pub fn invert_rect(
        &mut self,
        #[callback] lines: Vec<Base64VecU8>,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> Promise {
        // Parse current board from BerryClub
        let board = decode_board(lines);

        let pixels = internal_invert_rect(board, left, top, width, height);

        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }
}

pub fn internal_invert_rect(
    board: Vec<Vec<u32>>,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
) -> Vec<SetPixelRequest> {
    // Optionally check the validity of the input.
    assert!(left < BOARD_WIDTH, "left is out of bounds [0..49]");
    assert!(top < BOARD_HEIGHT, "top is out of bounds [0..49]");
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
        top + height <= BOARD_WIDTH,
        "left + top is out of bounds [0..49]"
    );

    // Creates an empty vector
    let mut pixels = vec![];

    // Iterate over pixels of the rectangle.
    for i in top..top + height {
        let line = &board[i as usize];
        for j in left..left + width {
            let color = line[j as usize];
            let new_color = 0xfffffff - color;

            let pixel = SetPixelRequest {
                x: j,
                y: i,
                color: new_color,
            };
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
    pub fn test_invert_rect() {
        // Red map
        let input_board = vec![vec![0xff0000u32; BOARD_WIDTH as usize]; BOARD_WIDTH as usize];
        let pixels = internal_invert_rect(input_board, 4, 2, 3, 4);
        let board = debug_print_pixels(&pixels);
        assert_eq!(&board[0][..15], "...............");
        assert_eq!(&board[1][..15], "...............");
        assert_eq!(&board[2][..15], "....XXX........");
        assert_eq!(&board[3][..15], "....XXX........");
        assert_eq!(&board[4][..15], "....XXX........");
        assert_eq!(&board[5][..15], "....XXX........");
        assert_eq!(&board[6][..15], "...............");
        assert_eq!(pixels.len(), 12);
        assert!(
            pixels.iter().all(|p| p.color == 0x00ffff),
            "Colors are invalid"
        );
    }
}
