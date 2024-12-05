#!/usr/bin/env bash
cargo test -p cyndra-proto --all-features --test '*' -- --skip needs_docker --nocapture
