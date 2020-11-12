cargo build --release --target wasm32-unknown-unknown || exit 1

cp target/wasm32-unknown-unknown/release/*.wasm main.wasm

basic-http-server -a 0.0.0.0:8080
