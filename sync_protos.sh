#!/bin/bash

curl https://raw.githubusercontent.com/lightningnetwork/lnd/v0.13.1-beta/lnrpc/rpc.proto -o src/protos/rpc.proto
curl https://raw.githubusercontent.com/lightningnetwork/lnd/v0.13.1-beta/lnrpc/stateservice.proto -o src/protos/stateservice.proto
curl https://raw.githubusercontent.com/lightningnetwork/lnd/v0.13.1-beta/lnrpc/walletunlocker.proto -o src/protos/walletunlocker.proto
