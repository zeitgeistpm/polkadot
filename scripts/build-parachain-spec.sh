#!/usr/bin/env bash

# Takes a staging network chain to create a new production-ready specification

set -euxo pipefail

OUTPUT_FILE=node/service/res/battery-station-relay.json
PROD_CHAIN_NAME="Rococo Battery Station Relay Testnet"
PROD_CHAIN_PROTOCOL_ID=battery_station_relay
PROD_CHAIN_ID=rococo_battery_station_relay_testnet
STAGE_CHAIN=rococo-staging

cargo build --release 
./target/release/polkadot build-spec --chain $STAGE_CHAIN --disable-default-bootnode > $OUTPUT_FILE

sed -i "s/\"id\": \".*\"/\"id\": \"$PROD_CHAIN_ID\"/" $OUTPUT_FILE
sed -i "s/\"name\": \".*\"/\"name\": \"$PROD_CHAIN_NAME\"/" $OUTPUT_FILE
sed -i "s/\"protocolId\": \".*\"/\"protocolId\": \"$PROD_CHAIN_PROTOCOL_ID\"/" $OUTPUT_FILE

./target/release/polkadot build-spec --chain $OUTPUT_FILE --disable-default-bootnode --raw > $OUTPUT_FILE.raw
rm -f $OUTPUT_FILE
mv $OUTPUT_FILE.raw $OUTPUT_FILE