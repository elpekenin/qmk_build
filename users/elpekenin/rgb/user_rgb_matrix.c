// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "quantum.h"

#include "placeholders.h"
#include "user_layers.h"
#include "user_rgb_matrix_types.h"

// Note: Can't map a color to KC_NO with current implementation, as unassigned elements would be filled with it(0)
static const keycode_color_map_t layer_mappings[][COLORS_PER_LAYER] = {
    [_RST] = {
        KC_COLOR(QK_BOOT, RGB_RED),
        KC_COLOR(QK_RBT, RGB_BLUE),
        KC_COLOR(EE_CLR, RGB_ORANGE),
        KC_COLOR(DB_TOGG, MAX_WHITE),
    }
};

static inline RGB get_color(uint8_t layer_num, uint16_t keycode) {
    for (uint8_t i=0; i < COLORS_PER_LAYER; ++i) {
        keycode_color_map_t map = layer_mappings[layer_num][i];
        if (map.keycode && map.keycode == keycode) {
            return map.color;
        }
    }

    return (RGB){RGB_OFF};
}

static void render_layer(uint8_t layer_num) {
    for (uint8_t row = 0; row < MATRIX_ROWS; ++row) {
        for (uint8_t col = 0; col < MATRIX_COLS; ++col) {
            uint8_t index = g_led_config.matrix_co[row][col];
            uint16_t keycode = keymap_key_to_keycode(layer_num, (keypos_t){col,row});

            RGB color = get_color(layer_num, keycode);
            rgb_matrix_set_color(index, color.r, color.g, color.b);
        }
    }
}

bool rgb_matrix_indicators_user(void) {
    if (!rgb_matrix_indicators_keymap()) {
        return false;
    }

    uint8_t layer_num = get_highest_layer(layer_state);
    switch (layer_num) {
        case _RST:
            render_layer(layer_num);
            break;

        default:
            break;
    }

    return true;
}

bool led_update_user(led_t led_state) {
    if (!led_update_keymap(led_state)) {
        return false;
    }

    // i dont really want debug here:
    //    - rgb matrix mode [NOEEPROM]: x
    //    - rgb matrix set hsv [NOEEPROM]: x, y, z
    bool old_debug_state = debug_enable;
    debug_enable         = false;
    if (led_state.caps_lock) {
        rgb_matrix_mode_noeeprom(RGB_MATRIX_SOLID_COLOR);
        rgb_matrix_sethsv_noeeprom(165, 255, rgb_matrix_get_val());
    } else {
        rgb_matrix_mode_noeeprom(RGB_MATRIX_CYCLE_LEFT_RIGHT);
    }
    debug_enable = old_debug_state;

    return false;
}
