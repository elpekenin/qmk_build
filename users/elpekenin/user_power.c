// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "debug.h"

#include "elpekenin.h"
#include "graphics.h"
#include "placeholders.h"
#include "user_xap.h"

bool shutdown_user(bool jump_to_bootloader) {
    if (!shutdown_keymap(jump_to_bootloader)) {
        return false;
    }

#if defined(QUANTUM_PAINTER_ENABLE)
    for (uint8_t i = 0; i < QUANTUM_PAINTER_NUM_DISPLAYS; ++i) {
        qp_power(qp_devices_pekenin[i], false);
    }
#endif // defined(QUANTUM_PAINTER_ENABLE)

#if defined(RGB_MATRIX_ENABLE)
    if (jump_to_bootloader) {
        // off for bootlaoder
        rgb_matrix_set_color_all(RGB_OFF);
    } else {
        // red for reboot
        rgb_matrix_set_color_all(RGB_MATRIX_MAXIMUM_BRIGHTNESS, 0, 0);
    }

    // force flushing -- otherwise will never happen
    void rgb_matrix_update_pwm_buffers(void);
    rgb_matrix_update_pwm_buffers();

    wait_ms(150);
#endif // RGB_MATRIX_ENABLE

#if defined(QP_XAP_ENABLE)
    xap_shutdown(jump_to_bootloader);
#endif // defined(QP_XAP_ENABLE)

    return true;
}

static bool suspend_changed     = true;
static bool suspend_debug_state = true;
static bool keyboard_booted     = false;
void suspend_power_down_user(void) {
    // good amount of logic would be lost, so we call housekeeping function while suspended
    if (!suspend_changed) {
        housekeeping_task();
        return;
    }

    // disable debug, saving current setting
    if (keyboard_booted) {
        suspend_debug_state = debug_enable;
        debug_enable        = false;
    }

    // only run suspend-specific code once per suspend duration
    suspend_changed = false;
    suspend_power_down_keymap();
}

void suspend_wakeup_init_user(void) {
    // enable debug after USB is init'ed on startup
    if (!keyboard_booted) {
        keyboard_booted = true;
        debug_enable    = true;
        return;
    }

    // flag the suspend callback to handle code on its next trigger
    suspend_changed = true;

    // restore debug and log event after suspend
    debug_enable = suspend_debug_state;
    dprintln("waking up...");

    suspend_wakeup_init_keymap();
}
