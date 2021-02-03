use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, Promise};

pub const BOARD_WIDTH: u32 = 50;
pub const BOARD_HEIGHT: u32 = 50;
pub const TOTAL_NUM_PIXELS: u32 = BOARD_WIDTH * BOARD_HEIGHT;

mod art;
mod circle;
mod hardcore;
mod invert;
mod rect;

pub use crate::hardcore::*;
use near_sdk::json_types::Base64VecU8;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}
