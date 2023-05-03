# Builds `tonic-web-client`
build:
    @echo 'Building...'
    cargo build

# Builds `tonic-web-client` for wasm target
build-wasm:
    @echo 'Building for wasm...'
    cargo build --target wasm32-unknown-unknown

# Builds test `tonic-web` server
build-test-server:
    @echo 'Building test server...'
    cd test-suite/simple/server && cargo build

# Starts test `tonic-web` server
start-test-server:
    @echo 'Starting test server...'
    cd test-suite/simple/server && cargo run

# Runs tests for `tonic-web-client`
test:
    @echo 'Testing...'
    cargo test

# Runs browser tests for `tonic-web-client`
test-wasm:
    @echo 'Testing...'
    cd test-suite/simple/client && wasm-pack test --chrome

# Runs browser tests for `tonic-web-server` (in headless mode)
test-wasm-headless:
    @echo 'Testing...'
    cd test-suite/simple/client && wasm-pack test --headless --chrome

# Builds test `tonic-web` server (with compression enabled: gzip)
build-gzip-test-server:
    @echo 'Building test server...'
    cd test-suite/gzip/server && cargo build

# Starts test `tonic-web` server (with compression enabled: gzip)
start-gzip-test-server:
    @echo 'Starting test server...'
    cd test-suite/gzip/server && cargo run

# Runs tests for `tonic-web-client` (with compression enabled: gzip)
test-gzip:
    @echo 'Testing...'
    cargo test

# Runs browser tests for `tonic-web-client` (with compression enabled: gzip)
test-gzip-wasm:
    @echo 'Testing...'
    cd test-suite/gzip/client && wasm-pack test --chrome

# Runs browser tests for `tonic-web-server` (in headless mode) (with compression enabled: gzip)
test-gzip-wasm-headless:
    @echo 'Testing...'
    cd test-suite/gzip/client && wasm-pack test --headless --chrome
