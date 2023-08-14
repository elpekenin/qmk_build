// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include QMK_KEYBOARD_H

#include "elpekenin.h"
#include "graphics.h"
#include "user_xap.h"

// Create the keymap
#undef LAYER
#define LAYER(layer_name, ...) [layer_name] = LAYOUT(__VA_ARGS__),
#undef __DUMMY_LAYER
#define __DUMMY_LAYER(...)
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
#include KEYMAP_LAYERS_H
};

void keyboard_post_init_keymap(void) {
#if defined(QUANTUM_PAINTER_ENABLE)
#    if defined(INIT_EE_HANDS_LEFT)
    load_display(il91874);
    qp_log_target_device = NULL;
#    else
    load_display(ili9163);
    load_display(ili9341);
    qp_log_target_device = ili9341;
#    endif // defined(INIT_EE_HANDS_LEFT)
#endif // defined(QUANTUM_PAINTER_ENABLE)
}

void user_data_sync_keymap_callback(void) {
#if defined(INIT_EE_HANDS_LEFT) && defined(QUANTUM_PAINTER_ENABLE)
    draw_commit(il91874);
    draw_features(il91874);
#endif // defined(INIT_EE_HANDS_LEFT) && defined(QUANTUM_PAINTER_ENABLE)
}

#if defined(QUANTUM_PAINTER_ENABLE) && defined (TOUCH_SCREEN_ENABLE) && defined(INIT_EE_HANDS_RIGHT)
void housekeeping_task_keymap(void) {
    uint32_t now = timer_read32();

    static uint32_t touch_timer = 0;

    // We only read once in a while
    if (TIMER_DIFF_32(now, touch_timer) < TOUCH_MS)
        return;

    touch_timer = now;

    // Do nothing until sensor initialised or when screen isn't pressed
    if (!ili9341_touch || !ili9341_pressed) {
        xap_screen_released(ILI9341_ID);
        return;
    }

    // Make a read and send it to Tauri
    touch_report_t ili9341_touch_report = get_spi_touch_report(ili9341_touch, false);

    xap_screen_pressed(ILI9341_ID, ili9341_touch_report);

#    if defined(ONE_HAND_ENABLE)
    screen_one_hand(touch_report);
#    endif // ONE_HAND_ENABLE
}
#endif // defined(QUANTUM_PAINTER_ENABLE) && defined (TOUCH_SCREEN_ENABLE) && defined(INIT_EE_HANDS_RIGHT)
