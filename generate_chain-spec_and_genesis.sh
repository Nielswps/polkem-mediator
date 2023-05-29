target/release/polkem-mediator-node build-spec --disable-default-bootnode --chain local > mediator-spec.json
target/release/polkem-mediator-node build-spec --disable-default-bootnode --chain mediator-spec.json --raw > raw-mediator-spec.json
cp raw-mediator-spec.json ../polkem/chain-specs/mediator-spec.json

target/release/polkem-mediator-node export-genesis-state --chain raw-mediator-spec.json mediator-genesis-state
target/release/polkem-mediator-node export-genesis-wasm --chain raw-mediator-spec.json mediator-genesis-wasm