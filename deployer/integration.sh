#!/usr/bin/env bash
cargo test -p cyndra-deployer --all-features --test '*' -- --skip needs_docker --nocapture
