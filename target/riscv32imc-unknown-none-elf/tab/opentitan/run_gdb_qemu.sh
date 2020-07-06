#!/bin/bash

#Supply a file to be debugged
if [ -z "$1" ]
then
echo "No elf file was supplied"
echo "Usage "$0" <.elf file>"
exit 0
fi

riscv32-unknown-elf-gdb "$1" -ex "target remote :1234"

