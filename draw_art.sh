#!/bin/bash
set -e

cargo test art::tests::draw_art -- --exact --nocapture

