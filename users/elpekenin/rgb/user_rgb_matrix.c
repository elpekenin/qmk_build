// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "quantum.h"

#include "placeholders.h"
#include "user_layers.h"

bool rgb_matrix_indicators_user(void) {
    if (!rgb_matrix_indicators_keymap()) {
        return false;
    }

    uint8_t layer = get_highest_layer(layer_state);

    if (layer == _RST) {
        rgb_matrix_set_color_all(RGB_OFF);

        for (uint8_t row = 0; row < MATRIX_ROWS; ++row) {
            for (uint8_t col = 0; col < MATRIX_COLS; ++col) {
                uint8_t index = g_led_config.matrix_co[row][col];

                switch (keymap_key_to_keycode(layer, (keypos_t){col,row})) {
                    case QK_BOOT:
                        rgb_matrix_set_color(index, RGB_RED);
                        break;

                    case QK_RBT:
                        rgb_matrix_set_color(index, RGB_BLUE);
                        break;

                    case EE_CLR:
                        rgb_matrix_set_color(index, RGB_ORANGE);
                        break;

                    case DB_TOGG:
                        rgb_matrix_set_color(
                            index,
                            RGB_MATRIX_MAXIMUM_BRIGHTNESS,
                            RGB_MATRIX_MAXIMUM_BRIGHTNESS,
                            RGB_MATRIX_MAXIMUM_BRIGHTNESS
                        );
                        break;

                    default:
                        break;
                }
            }
        }
    }

    return true;
}

bool led_update_user(led_t led_state) {
    // bodge for layer_tap calling this func
    static uint8_t old_state = 0;
    if (old_state == led_state.raw) {
        return true;
    }
    old_state = led_state.raw;

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
