use crate::*;
#[allow(unused_imports)]
use circle::internal_render_circle;
#[allow(unused_imports)]
use rect::internal_render_rect;

#[near_bindgen]
impl Contract {
    /// Renders your art.
    /// To call this method you need to call `render_art`, e.g. `render_art '{}'`
    pub fn render_art(&mut self) -> Promise {
        let pixels = internal_render_art();
        // Issue a cross-contract call to the Berry Club contract to draw pixels and return the
        // promise from this method.
        draw(pixels)
    }
}

pub(crate) fn internal_render_art() -> Vec<SetPixelRequest> {
    // Creates an empty vector
    let mut pixels = vec![];

    // TODO: Implement your art by modifying code below.
    // You can also draw using implementations of rect and circle as examples.
    // E.g. the following will render 3 circles:
    pixels.extend(internal_render_circle(20, 20, 15, 0xff0000));
    pixels.extend(internal_render_circle(20, 20, 10, 0xffff00));
    pixels.extend(internal_render_circle(20, 20, 5, 0x00ff00));

    pixels
}

#[cfg(not(target = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn draw_art() {
        let pixels = internal_render_art();
        debug_print_pixels(&pixels);
    }
}
