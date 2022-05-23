which cargo >/dev/null 2>&1 || (echo "Please install Rust toolchain" && exit 1)

cargo build --release --target wasm32-unknown-unknown || exit 1

cp target/wasm32-unknown-unknown/release/*.wasm main.wasm

which basic-http-server >/dev/null 2>&1 || (echo "Missing basic-http-server binary, installing..." && cargo install basic-http-server)

basic-http-server -a 0.0.0.0:8080
