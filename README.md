### Rust on STM32F3

This repository contains an example on how to use Rust on an STM32F3 microcontroller.
It was tested on an STM32F3 Discovery board.

Information from the following sources was used for setting up this project:

* [Engineering(DIY): STM32F3 Discovery + Eclipse + OpenOCD][1]
* [Rust bare metal on ARM microcontroller][2]

[1]: http://engineering-diy.blogspot.co.at/2012/11/stm32f3-discovery-eclipse-openocd.html
[2]: http://antoinealb.net/programming/2015/05/01/rust-on-arm-microcontroller.html

## Required Software

On debian-based systems, most of the required software can be installed via `apt-get`:

    sudo apt-get install build-essential gcc-arm-none-eabi openocd

As building this software requires a nightly version of Rust (as of 2015/09/11), I recommend multirust:

    curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
    multirust update nightly

Until I find some equivalent code that I am certain I can legally include in this repository, you will need to download `stm32f3.tar.gz` from [here][3] into the project directory, and the execute this command:

    make unpack-stm32f3.tar.gz

[3]: https://drive.google.com/drive/u/0/folders/0B__Rs5JF53-kfjVPd2l2U0ZVaUtEZG1GcW1NZ0VZRWcyMUl1WXV2ZFFzQkF3aWw0NG9IVDQ

Add an `udev` rule for the board:

    sudo sh -c 'echo ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE="0666" > /etc/udev/rules.d/99-stlink.rules'

