#!/bin/bash

LIBTOCKDIR="$HOME/Documents/MA/Userspace/libtock-rs"

cd $LIBTOCKDIR
make flash-opentitan EXAMPLE=client_process FEATURES=alloc

cd $LIBTOCKDIR/target/riscv32imc-unknown-none-elf/tab/opentitan
rm combined_app.tbf

cat server_process/rv32imc.tbf > combined_app.tbf  
cat client_process/rv32imc.tbf >> combined_app.tbf  
