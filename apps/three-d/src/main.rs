#[cfg(not(target_arch = "wasm32"))]
mod services;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn main() {
    services::game::run().await;
}
