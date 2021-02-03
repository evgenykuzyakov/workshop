#!/bin/bash
set -e

cargo test rect::tests::test_rect_small -- --exact
cargo test rect::tests::test_rect_complex -- --exact

cargo test circle::tests::test_circle_small -- --exact

cargo test invert::tests::test_invert_rect -- --exact

