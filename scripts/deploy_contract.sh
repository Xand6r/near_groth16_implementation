export NEAR_ACCOUNT=aplayground.testnet

cargo build --target wasm32-unknown-unknown --release
near contract deploy $NEAR_ACCOUNT use-file target/wasm32-unknown-unknown/release/near_groth.wasm without-init-call network-config testnet sign-with-keychain send