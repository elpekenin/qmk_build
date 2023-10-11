// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "action_layer.h"
#include "action_util.h"
#include "keymap_common.h"
#include "rgb_matrix.h"

#include "elpekenin.h" // layers names and custom keycodes
#include "user_utils.h"

#define MAX_WHITE RGB_MATRIX_MAXIMUM_BRIGHTNESS, RGB_MATRIX_MAXIMUM_BRIGHTNESS, RGB_MATRIX_MAXIMUM_BRIGHTNESS

// *********
// * Types *
// *********

typedef struct indicator_t indicator_t;

// arguments passed from rgb_matrix_indicators to indicators_fn_t
typedef struct {
    uint8_t  layer;
    uint8_t  mods;
    uint16_t keycode;
} indicator_fn_args_t;

typedef bool(* indicator_fn_t)(indicator_t *indicator, indicator_fn_args_t *args);

// indicator specification: condition when it has to be drawn + color
struct indicator_t {
    // common config
    rgb_led_t      color;
    indicator_fn_t check;

    // conditions
    uint8_t  mods;
    uint8_t  layer;
    uint16_t keycode;
};

// **********
// * Checks *
// **********

#define __keycode() (args->keycode == indicator->keycode)
#define __layer()   (args->layer == indicator->layer)
#define __mods()    (args->mods & indicator->mods)

// draw the given keycode
UNUSED static bool keycode_callback(indicator_t *indicator, indicator_fn_args_t *args) {
    return __keycode();
}

// draw every key while on the given layer
UNUSED static bool layer_callback(indicator_t *indicator, indicator_fn_args_t *args) {
    return __layer();
}

// draw the given keycode while on the given layer
UNUSED static bool keycode_and_layer_callback(indicator_t *indicator, indicator_fn_args_t *args) {
    return __keycode() && __layer();
}

// draw every keycode configured (i.e. not KC_NO nor KC_TRNS) on the given layer
UNUSED static bool layer_and_configured_callback(indicator_t *indicator, indicator_fn_args_t *args) {
    return __layer() && indicator->keycode > KC_TRNS;
}

// draw the given keycode if given mods are set (i.e. display shortcuts)
UNUSED static bool keycode_and_mods_callback(indicator_t *indicator, indicator_fn_args_t *args) {
    return __keycode() && __mods();
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

#define LAYER(_layer, _col) { \
    .color = _RGB(_col), \
    .check = &layer_callback, \
    .layer = _layer, \
}

#define KC_LAYER(_kc, _layer, _col) { \
    .keycode = _kc, \
    .color = _RGB(_col), \
    .check = &keycode_and_layer_callback, \
    .layer = _layer, \
}

#define NO_TRNS(_layer, _col) { \
    .color = _RGB(_col), \
    .check = &layer_and_configured_callback, \
    .layer = _layer, \
}

#define KC_MOD(_kc, _mods, _col) { \
    .keycode = _kc, \
    .color = _RGB(_col), \
    .check = &keycode_and_mods_callback, \
    .mods = _mods, \
}

// ***************
// * Definitions *
// ***************

static const indicator_t indicators[] = {
    LAYER(_RST, RGB_OFF),
    KC_LAYER(QK_BOOT, _RST, RGB_RED),
    KC_LAYER(QK_RBT,  _RST, RGB_GREEN),
    KC_LAYER(EE_CLR,  _RST, RGB_YELLOW),
    KC_LAYER(DB_TOGG, _RST, MAX_WHITE),
};

// ************
// * Callback *
// ************

bool draw_indicators(uint8_t led_min, uint8_t led_max) {
    uint8_t mods  = get_mods();
    uint8_t layer = get_highest_layer(layer_state);

    indicator_fn_args_t args = {
        .mods = mods,
        .layer = layer,
    };

    // iterate all keys
    for (uint8_t row = 0; row < MATRIX_ROWS; ++row) {
        for (uint8_t col = 0; col < MATRIX_COLS; ++col) {
            uint8_t index = g_led_config.matrix_co[row][col];

            // early exit if out of range
            if (index < led_min || index >= led_max) {
                continue;
            }

            args.keycode = keymap_key_to_keycode(layer, (keypos_t){col,row});

            // iterate all indicators
            for (uint8_t i = 0; i < ARRAY_SIZE(indicators); ++i) {
                indicator_t indicator = indicators[i];

                // if check passed, draw
                if (indicator.check(&indicator, &args)) {
                    rgb_matrix_set_color(index, indicator.color.r, indicator.color.g, indicator.color.b);
                }
            }

        }
    }

    return false;
}