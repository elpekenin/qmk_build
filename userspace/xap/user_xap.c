#include "quantum.h"
#include "user_utils.h"
#include "user_xap.h"

// for keycode as str
#if defined(KEYLOG_ENABLE)
#    include "user_keylog.h"
#endif // defined(KEYLOG_ENABLE)

void xap_screen_pressed(uint8_t screen_id, touch_report_t report) {
    screen_pressed_msg_t msg = {
        .msg_id = _SCREEN_PRESSED,
        .screen_id = screen_id,
        .x = report.x,
        .y = report.y
    };

    xap_broadcast_user(&msg, sizeof(msg));
}

void xap_screen_released(uint8_t screen_id) {
    screen_released_msg_t msg = {
        .msg_id = _SCREEN_RELEASED,
        .screen_id = screen_id
    };

    xap_broadcast_user(&msg, sizeof(msg));
}

void xap_layer(layer_state_t state) {
    layer_change_msg_t msg = {
        .msg_id = _LAYER_CHANGE,
        .layer = get_highest_layer(state)
    };

    xap_broadcast_user(&msg, sizeof(msg));
}

void xap_keyevent(uint16_t keycode, keyrecord_t *record) {
    keyevent_msg_t msg = {
        .base.msg_id = _KEYEVENT,
        .base.keycode = keycode,
        .base.pressed = record->event.pressed,
        .base.layer = get_highest_layer(layer_state),
        .base.row = record->event.key.row,
        .base.col = record->event.key.col,
        .base.mods = MODIFIERS()
    };

#if defined(KEYLOG_ENABLE)
    strcpy(msg.str, get_keycode_str_at(msg.base.layer, msg.base.row, msg.base.col));
#endif // defined(KEYLOG_ENABLE)

    xap_broadcast_user(&msg, sizeof(msg));
}

void xap_shutdown(bool jump_to_bootloader) {
    shutdown_msg_t msg = {
        .msg_id = _SHUTDOWN,
        .bootloader = jump_to_bootloader,
    };

    xap_broadcast_user(&msg, sizeof(msg));
}
