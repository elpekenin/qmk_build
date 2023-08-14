// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: Some code here will break when certain features aren't enabled, will fix those once it happens

#include "version.h"

#include "elpekenin.h"
#include "user_data.h"
#include "user_keylog.h"
#include "placeholders.h"

// My generated files
#include "generated_features.h"

// Conditional imports
#if defined(QUANTUM_PAINTER_ENABLE)
#    include "graphics.h"
#    include "qp_logging.h"
#else
#    define load_qp_resources()
#    define qp_housekeeping(now)
#endif // defined(QUANTUM_PAINTER_ENABLE)

#if defined(SPLIT_KEYBOARD)
#    include "user_transactions.h"
#else
#    define split_sync_housekeeping(now)
#    define transactions_init()
#endif // defined(SPLIT_KEYBOARD)


void housekeeping_task_user(void) {
    uint32_t now  = timer_read32();
    qp_housekeeping(now);
    split_sync_housekeeping(now);

    housekeeping_task_keymap();
}

void keyboard_pre_init_user(void) {
#if defined(QUANTUM_PAINTER_ENABLE)
    print_set_sendchar(elpekenin_sendchar);
#endif // defined(QUANTUM_PAINTER_ENABLE)

    keyboard_pre_init_keymap();
}

user_data_t user_data = {0};
void keyboard_post_init_user(void) {
#if defined(AUTOCORRECT_ENABLE)
    autocorrect_enable();
#endif // defined(AUTOCORRECT_ENABLE)

    populate_keycode_names();

    load_qp_resources();

    transactions_init();
    // has to be after transactions_init, because it memset's user_data to 0
    if (is_keyboard_master()) {
        user_data = (user_data_t) {
            .commit   = QMK_GIT_HASH,
            .features = get_enabled_features(),
        };
    }

    configure_tri_layer();

    keyboard_post_init_keymap();
}
