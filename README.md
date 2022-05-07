# Kallisti

...to the Prettiest!

(this is mostly a toy project but who knows what might happen)

## Development

### Dependencies

* `rustup target add wasm32-unknown-unknown`
* `cargo install trunk --locked`
* `cargo install wasm-bindgen-cli`
* `cargo install tauri-cli --locked --version ^1.0.0-rc`

### Build & Run

* `cargo make dev` for auto-reloading dev server
* `cargo make build` for final release
