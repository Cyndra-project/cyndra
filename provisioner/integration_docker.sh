#!/usr/bin/env bash
cargo test -p cyndra-provisioner --all-features --test '*' -- needs_docker --nocapture
