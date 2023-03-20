#[cfg(not(target_arch = "wasm32"))]
mod game;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    game::run();
}
