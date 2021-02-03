#!/bin/bash
set -e

near call $ACCOUNT_ID --accountId=$ACCOUNT_ID --gas=300000000000000 "$@"

