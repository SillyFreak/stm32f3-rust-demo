PROJ_NAME = stm32f3

# Set toolchain
TC = arm-none-eabi

# Set Tools
CC      = $(TC)-gcc
AR      = $(TC)-ar
OBJCOPY = $(TC)-objcopy
OBJDUMP = $(TC)-objdump
SIZE    = $(TC)-size
RUSTC   = rustc


# Set Sources
STM32F3_C_SRCS = $(wildcard stm32f3/Libraries/STM32F30x_StdPeriph_Driver/src/*.c)
STM32F3_C_SRCS+= $(wildcard stm32f3/src/*.c)
STM32F3_S_SRCS = $(wildcard stm32f3/src/*.s)

C_SRCS  = $(wildcard src/*.c)
RS_SRCS = src/main.rs

# Set Objects
STM32F3_OBJS = $(STM32F3_C_SRCS:.c=.o) $(STM32F3_S_SRCS:.s=.o)
OBJS         = $(C_SRCS:.c=.o) $(RS_SRCS:.rs=.o)

# Set Include Paths
INCLUDES = -Istm32f3/Libraries/STM32F30x_StdPeriph_Driver/inc/ \
           -Istm32f3/Libraries/CMSIS/Include \
           -Istm32f3/Libraries/CMSIS/Device/ST/STM32F30x/Include \
           -Istm32f3/inc/
			
# Set Libraries
LIBS = -lm -lc


# Set Board
MCU     = -mthumb -mcpu=cortex-m4
FPU     = -mfpu=fpv4-sp-d16 -mfloat-abi=softfp
DEFINES = -DSTM32F3XX -DUSE_STDPERIPH_DRIVER

# Set Compilation and Linking Flags
CFLAGS  = $(MCU) $(FPU) $(DEFINES) $(INCLUDES) \
          -g -Wall -std=gnu90 -O0 -ffunction-sections -fdata-sections
ASFLAGS = $(MCU) $(FPU) -g -Wa,--warn -x assembler-with-cpp
LDFLAGS = $(MCU) $(FPU) -g -gdwarf-2\
          -Tstm32f3/stm32f30_flash.ld \
          -Xlinker --gc-sections -Wl,-Map=$(PROJ_NAME).map \
          $(LIBS) \
          -o $(PROJ_NAME).elf

RUSTFLAGS = -C opt-level=2 -Z no-landing-pads \
            --target thumbv7em-none-eabi -g --emit obj \
            -L libcore-thumbv7m \
            -L librustc_bitflags-thumbv7m


# Preparation

unpack-stm32f3.tar.gz:
	rm -rf stm32f3
	tar -xzf stm32f3.tar.gz
	rm stm32f3/src/main.c
	@echo "(main.c is not used)"

libs:
	rm -rf libcore-thumbv7m librustc_bitflags-thumbv7m
	mkdir libcore-thumbv7m librustc_bitflags-thumbv7m
	rustc -C opt-level=2 -Z no-landing-pads --target thumbv7em-none-eabi -g rust/src/libcore/lib.rs --out-dir libcore-thumbv7m
	rustc -C opt-level=2 -Z no-landing-pads --target thumbv7em-none-eabi -g rust/src/librustc_bitflags/lib.rs   --out-dir librustc_bitflags-thumbv7m -L libcore-thumbv7m/


# Build

%.o: %.c
	@$(CC) $(CFLAGS) -c -o $@ $<
	@echo $@

%.o: %.s
	@$(CC) $(ASFLAGS) -c -o $@ $<
	@echo $@

%.o: %.rs
	@$(RUSTC) $(RUSTFLAGS) -o ${@} ${<}
	@echo $@

$(PROJ_NAME).elf: $(STM32F3_OBJS) $(OBJS)
	@$(CC) $(STM32F3_OBJS) $(OBJS) $(LDFLAGS)
	@echo $@

$(PROJ_NAME).bin: $(PROJ_NAME).elf
	@$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin
	@echo $@

info: $(PROJ_NAME).elf
	@$(SIZE) --format=berkeley $(PROJ_NAME).elf

all: $(PROJ_NAME).bin info

# Deployment

flash:
	openocd -f /usr/share/openocd/scripts/board/stm32f3discovery.cfg -c "program $(PROJ_NAME).elf verify reset" -c "shutdown"


# Cleanup

clean:
	rm -f $(STM32F3_OBJS)
	rm -f $(OBJS)
	rm -f $(PROJ_NAME).elf
	rm -f $(PROJ_NAME).bin
	rm -f $(PROJ_NAME).map

