/// This method is slightly different. It affects entire board and has to be called differently.
/// To call this method you need to call `render` and pass `method_name`, e.g.
/// `render '{"method_name": "pixel_shake", "left": 20, "top": 10, "width": 20, "height": 20}'`
/// This is because you first need to get the lines from the berryclub contract.
/// See `berryclub::render` for implementation details.
pub fn pixel_shake(&mut self, #[callback] lines: Vec<Base64VecU8>, shake_distance: u32) -> Promise {
    // Parse current board from BerryClub
    let board = decode_board(lines);

    // Creates an empty vector
    let mut pixels = vec![];

    // Query current random seed from the context. It's a vector of 32 random bytes.
    let mut random_seed = env::random_seed();
    // We need 50 random values. So we're going to extend this by a hash of the random seed.
    random_seed.extend(env::sha256(&random_seed));

    for (y, line) in board.into_iter().enumerate() {
        let x_offset =
            BOARD_WIDTH + (random_seed[y] as u32) % (shake_distance * 2 + 1) - shake_distance;
        for (x, color) in line.into_iter().enumerate() {
            let new_x = (x as u32 + x_offset) % BOARD_WIDTH;
            pixels.push(SetPixelRequest {
                x: new_x as u32,
                y: y as u32,
                color,
            })
        }
    }

    // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
    // promise from this method.
    draw(pixels)
}
