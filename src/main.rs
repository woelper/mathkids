// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = mathkids::GameState::default();
    eframe::run_native(Box::new(app));
}
