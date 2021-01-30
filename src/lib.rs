use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Promise};

pub mod berryclub;
pub use crate::berryclub::*;
use near_sdk::json_types::Base64VecU8;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    /// Renders a solid color filled rectangle.
    /// To call this method you need to call `render_rect`, e.g.
    /// `render_rect '{"left": 10, "top": 20, "width": 10, "height": 5, "color": 16711680}'`
    pub fn render_rect(
        &mut self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        color: u32,
    ) -> Promise {
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
            top + height <= BOARD_WIDTH,
            "left + top is out of bounds [0..49]"
        );

        // Creates an empty vector
        let mut pixels = vec![];

        // Iterate over pixels of the rectangle.
        for i in top..top + height {
            for j in left..left + width {
                let pixel = SetPixelRequest { x: j, y: i, color };
                // Verify that the pixel request is valid.
                // NOTE: That this check is not necessary, because we've verified the input above.
                pixel.assert_valid();

                pixels.push(pixel);
            }
        }

        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }

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
                let dx = i - center_y;
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

        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }

    /// This method requires the current board colors to compute then inverse colors.
    /// To call this method you need to call `render` and pass `method_name`, e.g.
    /// `render '{"method_name": "invert_rect", "left": 20, "top": 10, "width": 20, "height": 20}'`
    /// This is because you first need to get the lines from the berryclub contract.
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
                let old_color = line[j as usize];
                let color = 0xffffff - old_color;

                let pixel = SetPixelRequest { x: j, y: i, color };
                pixels.push(pixel);
            }
        }

        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }

    /**************/
    /* TODO BELOW */
    /**************/

    /// TODO: Implement code in this method as part of tutorial.
    /// Renders a rainbow colored filled rectangle.
    /// To call this method you need to call `render_rainbow_rect`, e.g.
    /// `render_rainbow_rect '{"left": 10, "top": 20, "width": 10, "height": 5}'`
    pub fn render_rainbow_rect(&mut self, left: u32, top: u32, width: u32, height: u32) -> Promise {
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
        for j in 0..width {
            // Color is a RGB value from [0, 0xffffff].
            // RED is `0xff0000` or `255 * 256 * 256` or `255 << 16`
            // GREEN is `0x00ff00` or `255 * 256` or `255 << 8`
            // BLUE is `0x0000ff` or `255` or `255`
            // The values can be combined, e.g. PURPLE is `0xff00ff` or YELLOW is `0xffff00`
            // To mix 75% RED and 50% GREEN, you can do the following: `(196 << 16) + (128 << 8)`
            // TODO: Implement rainbow color selection based on `j`.
            let rainbow_color = 0;
            for i in top..top + height {
                let pixel = SetPixelRequest {
                    x: j + left,
                    y: i,
                    color: rainbow_color,
                };
                // Verify that the pixel request is valid.
                // NOTE: That this check is not necessary, because we've verified the input above.
                pixel.assert_valid();

                pixels.push(pixel);
            }
        }

        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }
}
