#!/usr/bin/env bash
cargo test -p cyndra-runtime --all-features --test '*' -- --skip needs_docker --nocapture
