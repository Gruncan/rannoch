#!/bin/bash
sudo dd if=target/x86_64-rannoch/debug/bootimage-rannoch.bin of=$1 && sync
