#!/bin/bash

curl https://raw.githubusercontent.com/lightningnetwork/lnd/v0.13.1-beta/lnrpc/rpc.proto -o src/protos/rpc.proto
curl https://raw.githubusercontent.com/lightningnetwork/lnd/v0.13.1-beta/lnrpc/stateservice.proto -o src/protos/stateservice.proto
curl https://raw.githubusercontent.com/lightningnetwork/lnd/v0.13.1-beta/lnrpc/walletunlocker.proto -o src/protos/walletunlocker.proto

curl https://raw.githubusercontent.com/lightninglabs/loop/v0.14.2-beta/looprpc/client.proto -o src/protos/loop/client.proto
curl https://raw.githubusercontent.com/lightninglabs/loop/v0.14.2-beta/looprpc/common.proto -o src/protos/loop/common.proto
