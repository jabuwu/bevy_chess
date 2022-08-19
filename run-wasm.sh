set -e
rm -rf wasm
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --out-name chess --out-dir wasm --target web target/wasm32-unknown-unknown/release/bevy_chess.wasm
cp -r assets wasm/
cp static/* wasm/
(cd wasm && npx serve)
