#!/bin/bash

(sleep 3 && anchor test --skip-local-validator ) &
solana-test-validator --reset -q


