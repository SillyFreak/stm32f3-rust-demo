#include "common.h"

uint32_t SysTick_Config_wrapper(uint32_t ticks) {
    return SysTick_Config(ticks);
}

