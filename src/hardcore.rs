//! As part of the workshop you don't need to touch this contract.
//! It has some hardcode stuff that is needed for you to have fun.
//! But I'm glad you're looking for more.

use crate::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, ext_contract, Balance, Gas};

pub type AccountIndex = u32;

pub const BERRYCLUB_CONTRACT_ID: &str = "berryclub.ncd.testnet";

pub const NO_DEPOSIT: Balance = 0;

pub const BUY_TOKENS_GAS: Gas = 5_000_000_000_000;
pub const GET_LINES_GAS: Gas = 50_000_000_000_000;
pub const BASE_DRAW_GAS: Gas = 50_000_000_000_000;
pub const GAS_FOR_RENDER_WITH: Gas = GET_LINES_GAS + 20_000_000_000_000;
pub const GAS_PER_PIXEL: Gas = 50_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Copy, Clone)]
pub struct Pixel {
    pub color: u32,
    pub owner_id: AccountIndex,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            color: 0xffffff,
            owner_id: 0,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PixelLine(pub Vec<Pixel>);

impl Default for PixelLine {
    fn default() -> Self {
        Self(vec![Pixel::default(); BOARD_WIDTH as usize])
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SetPixelRequest {
    pub x: u32,
    pub y: u32,
    pub color: u32,
}

impl SetPixelRequest {
    pub fn is_valid(&self) -> bool {
        return self.x < BOARD_WIDTH && self.y < BOARD_HEIGHT && self.color <= 0xffffff;
    }

    pub fn assert_valid(&self) {
        assert!(self.x < BOARD_WIDTH, "X is out of bounds");
        assert!(self.y < BOARD_HEIGHT, "Y is out of bounds");
        assert!(self.color <= 0xffffff, "Color is out of bounds");
    }
}

#[ext_contract(ext_berryclub)]
trait BerryclubContract {
    fn get_lines(&self, lines: Vec<u32>) -> Vec<Base64VecU8>;

    fn buy_tokens(&mut self);

    fn draw(&mut self, pixels: Vec<SetPixelRequest>);
}

pub(crate) fn draw(pixels: Vec<SetPixelRequest>) -> Promise {
    let mut board = [[b'.'; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];
    let mut unique_pixels = vec![];
    for pixel in pixels.into_iter().rev() {
        if board[pixel.y as usize][pixel.x as usize] == b'.' {
            board[pixel.y as usize][pixel.x as usize] = b'X';
            unique_pixels.push(pixel);
        }
    }
    for line in &board {
        env::log(line);
    }

    #[cfg(feature = "for_real")]
    {
        let gas = BASE_DRAW_GAS + (unique_pixels.len() as u64) * GAS_PER_PIXEL;
        ext_berryclub::draw(
            unique_pixels,
            &BERRYCLUB_CONTRACT_ID.to_string(),
            NO_DEPOSIT,
            gas,
        )
    }
    #[cfg(not(feature = "for_real"))]
    {
        Promise::new(env::current_account_id())
    }
}

pub(crate) fn decode_board(lines: Vec<Base64VecU8>) -> Vec<Vec<u32>> {
    lines
        .into_iter()
        .map(|bytes| {
            PixelLine::try_from_slice(&bytes.0)
                .unwrap()
                .0
                .into_iter()
                .map(|p| p.color)
                .collect()
        })
        .collect()
}

#[near_bindgen]
impl Contract {
    #[cfg(feature = "for_real")]
    pub fn buy_avocado(&mut self) -> Promise {
        ext_berryclub::buy_tokens(
            &BERRYCLUB_CONTRACT_ID.to_string(),
            50 * 10u128.pow(24),
            BUY_TOKENS_GAS,
        )
    }

    pub fn render(method_name: String) -> Promise {
        ext_berryclub::get_lines(
            (0..BOARD_HEIGHT).collect(),
            &BERRYCLUB_CONTRACT_ID.to_string(),
            NO_DEPOSIT,
            GET_LINES_GAS,
        )
        .then(Promise::new(env::current_account_id()).function_call(
            method_name.into_bytes(),
            env::input().unwrap(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_RENDER_WITH,
        ))
    }
}

#[cfg(not(target = "wasm32"))]
#[cfg(test)]
pub(crate) fn debug_print_pixels(pixels: &[SetPixelRequest]) -> Vec<String> {
    let mut board = vec![vec![b'.'; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize];
    for pixel in pixels {
        board[pixel.y as usize][pixel.x as usize] = b'X';
    }
    let board = board
        .into_iter()
        .map(|line| String::from_utf8(line.to_vec()).unwrap())
        .collect::<Vec<_>>();
    println!();
    for (i, line) in board.iter().enumerate() {
        println!("{:02} {}", i, line);
    }
    board
}
