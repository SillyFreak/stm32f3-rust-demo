### Rust on STM32F3

This repository contains an example on how to use Rust on an STM32F3 microcontroller.
It was tested on an STM32F3 Discovery board.

Information from the following sources was used for setting up this project:

* [Engineering(DIY): STM32F3 Discovery + Eclipse + OpenOCD][1]
* [Rust bare metal on ARM microcontroller][2]

[1]: http://engineering-diy.blogspot.co.at/2012/11/stm32f3-discovery-eclipse-openocd.html
[2]: http://antoinealb.net/programming/2015/05/01/rust-on-arm-microcontroller.html

## Required Software

Until I find some equivalent code that I am certain I can legally include in this repository, you will need to download `stm32f3.tar.gz` from [here][3] into the project directory, and the execute this command:

    make unpack-stm32f3.tar.gz

[3]: https://drive.google.com/drive/u/0/folders/0B__Rs5JF53-kfjVPd2l2U0ZVaUtEZG1GcW1NZ0VZRWcyMUl1WXV2ZFFzQkF3aWw0NG9IVDQ

