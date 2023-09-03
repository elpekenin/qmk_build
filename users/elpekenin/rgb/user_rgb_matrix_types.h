// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

#include "color.h"

/* Maximum keycode->color mappings per layer */
#if !defined(COLORS_PER_LAYER)
#    define COLORS_PER_LAYER 10
#endif // !defined(COLORS_PER_LAYER)

typedef struct {
    uint16_t keycode;
    RGB      color;
} keycode_color_map_t;

#define MAX_WHITE RGB_MATRIX_MAXIMUM_BRIGHTNESS, RGB_MATRIX_MAXIMUM_BRIGHTNESS, RGB_MATRIX_MAXIMUM_BRIGHTNESS
#define KC_COLOR(kc, col) {.keycode = kc, .color = (RGB){col}}
