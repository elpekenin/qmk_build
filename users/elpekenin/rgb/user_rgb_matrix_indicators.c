// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "action_layer.h"
#include "action_util.h"
#include "keymap_common.h"
#include "rgb_matrix.h"

#include "elpekenin.h" // layers names and custom keycodes

#define MAX_WHITE RGB_MATRIX_MAXIMUM_BRIGHTNESS, RGB_MATRIX_MAXIMUM_BRIGHTNESS, RGB_MATRIX_MAXIMUM_BRIGHTNESS

// *********
// * Types *
// *********

typedef struct indicator_t indicator_t;
typedef bool(* indicator_fn_t)(uint8_t layer_num, uint16_t keycode, indicator_t *indicator);
struct indicator_t {
    uint16_t       keycode;
    rgb_led_t      color;
    indicator_fn_t check;

    union {
        uint8_t mod_bitmask;
        uint8_t layer_num;
    };
};

// **********
// * Checks *
// **********

__attribute__((unused)) static bool keycode_callback(uint8_t layer_num, uint16_t keycode, indicator_t *indicator) {
    return keycode == indicator->keycode;
}

static bool keycode_in_layer_callback(uint8_t layer_num, uint16_t keycode, indicator_t *indicator) {
    return (keycode == indicator->keycode) && (layer_num == indicator->layer_num);
}

static bool keycode_and_modifier_callback(uint8_t layer_num, uint16_t keycode, indicator_t *indicator) {
    return (keycode == indicator->keycode) && (get_mods() & indicator->mod_bitmask);
}

// **********
// * Macros *
// **********

// this crap is needed for RGB color macros to work nicely...
// relying on position aka: (rgb_led_t){_col} doesnt work
#define _RGB(_r, _g, _b) (rgb_led_t){ \
    .r = _r, \
    .g = _g, \
    .b = _b, \
}

#define KC(_kc, _col) { \
    .keycode = _kc, \
    .color = _RGB(_col), \
    .check = &keycode_callback, \
}

#define KC_LAYER(_kc, _col, _layer) { \
    .keycode = _kc, \
    .color = _RGB(_col), \
    .check = &keycode_in_layer_callback, \
    .layer_num = _layer, \
}

#define KC_MOD(_kc, _col, _mod_b) { \
    .keycode = _kc, \
    .color = _RGB(_col), \
    .check = &keycode_and_modifier_callback, \
    .mod_bitmask = _mod_b, \
}

// ***************
// * Definitions *
// ***************

static const indicator_t indicators[] = {
    KC_LAYER(QK_BOOT, RGB_RED,    _RST),
    KC_LAYER(QK_RBT,  RGB_GREEN,  _RST),
    KC_LAYER(EE_CLR,  RGB_YELLOW, _RST),
    KC_LAYER(DB_TOGG, MAX_WHITE,  _RST),

    KC_MOD(KC_C, RGB_RED, MOD_BIT(KC_LCTL)),
};

// ************
// * Callback *
// ************

bool draw_indicators(uint8_t led_min, uint8_t led_max) {
    uint8_t layer_num = get_highest_layer(layer_state);

    // iterate all keys
    for (uint8_t row = 0; row < MATRIX_ROWS; ++row) {
        for (uint8_t col = 0; col < MATRIX_COLS; ++col) {
            uint8_t index = g_led_config.matrix_co[row][col];

            // early exit if out of range
            if (index < led_min || index >= led_max) {
                continue;
            }

            uint16_t keycode = keymap_key_to_keycode(layer_num, (keypos_t){col,row});

            // iterate all indicators
            for (uint8_t i = 0; i < ARRAY_SIZE(indicators); ++i) {
                indicator_t indicator = indicators[i];

                // if check passed, draw
                if (indicator.check(layer_num, keycode, &indicator)) {
                    rgb_matrix_set_color(index, indicator.color.r, indicator.color.g, indicator.color.b);
                }
            }

        }
    }

    return false;
}