#!/bin/sh

# Auth: Wilfred MK
# Purpose: To build the baremetal code, flash it to a nrf52 target
#          and to then jump to the reset vector. 
if [ $1 == "run" ]
    then
    cargo rustc -- -C link-arg=--script=linker.ld
    cd target/thumbv7em-none-eabi/debug/
    arm-none-eabi-objcopy -O binary nrf52_bm_rust ./app.bin
    nrfjprog --eraseall
    nrfjprog --program app.bin -f nrf52 --verify
    nrfjprog --reset 
fi

if [ $1 == "clean" ]
    then
    cargo clean
fi
