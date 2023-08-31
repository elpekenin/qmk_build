// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

// This is just a PoC and has some limitations:
// - Since it uses `,` as separator, it doesnt work with compounds like `LT(1, KC_A)` and you would need some alias for those
// - Need this extra file, apart from the keymap... not the most convenient thing ever, but not terrible either imo
// - Would break (or at least miss some keys) if you have `KC_NO` on layer0 anywhere but on the empty spots of the matrix

// Note: KC_4 has custom logic defined on userspace
LAYER(_QWERTY,
    KC_ESC,  KC_1,    KC_2,    KC_3,    KC_4,    KC_5,           KC_6,    KC_7,    KC_8,    KC_9,    KC_0,    KC_BSPC,
    KC_TAB,  KC_Q,    KC_W,    KC_E,    KC_R,    KC_T,           KC_Y,    KC_U,    KC_I,    KC_O,    KC_P,    ES_PLUS,
    KC_CAPS, KC_A,    KC_S,    KC_D,    KC_F,    KC_G,           KC_H,    KC_J,    KC_K,    KC_L,    TD_NTIL, KC_ENT,
    KC_LSFT, TD_Z,    KC_X,    KC_C,    KC_V,    KC_B,           KC_N,    KC_M,    KC_COMM, KC_DOT,  KC_UP,   KC_VOLU,
	KC_LCTL, KC_LGUI, TL_UPPR, KC_LALT,     TD_SPC,                  R_SPC,        TL_LOWR, KC_LEFT, KC_DOWN, KC_RIGHT
)

// LOWER
LAYER(_FN1,
    XXXXXXX, KC_F1,   KC_F2,   KC_F3,   KC_F4,   KC_F5,          KC_F6,   KC_F7,   KC_F8,   KC_9,    KC_F10,  ES_BSLS,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, ES_LBRC, ES_RBRC, PK_CPYR,
    _______, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, RGB_VAI, RGB_MOD,
    XXXXXXX, XXXXXXX, _______, XXXXXXX,    PK_UCIS,                  XXXXXXX,      _______, RGB_SPD, RGB_VAD, RGB_SPI
)

// UPPER
// Note: Using number row keycodes instead of numpad, so we dont care about numlock
LAYER(_FN2,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        KC_7,    KC_8,    KC_9,    XXXXXXX, XXXXXXX, XXXXXXX,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        KC_4,    KC_5,    KC_6,    XXXXXXX, XXXXXXX, XXXXXXX,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        KC_1,    KC_2,    KC_3,    XXXXXXX, RGB_VAI, XXXXXXX,
    XXXXXXX, XXXXXXX, _______, XXXXXXX,     XXXXXXX,                  KC_0,        _______, RGB_SPD, RGB_VAD, RGB_SPI
)

LAYER(_FN3,
    XXXXXXX, ES_PIPE, ES_AT,   ES_HASH, ES_TILD, ES_EURO,        ES_NOT,  XXXXXXX, XXXXXXX, XXXXXXX, ES_QUOT, ES_BSLS,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, TD_GRV,  XXXXXXX,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, ES_LCBR, ES_RCBR, XXXXXXX,
    KC_LSFT, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, ES_MINS, XXXXXXX, XXXXXXX,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,      XXXXXXX,                 _______,     XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX
)

// ADJUST
LAYER(_RST,
    QK_BOOT, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, EE_CLR,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,
    PK_QCLR, AC_TOGG, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        PK_KLOG, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, QK_RBT,
    XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,        XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX, XXXXXXX,
    XXXXXXX, XXXXXXX, _______, XXXXXXX,     DB_TOGG,                 DB_TOGG,      _______, XXXXXXX, XXXXXXX, XXXXXXX
)

// Used to inverse the layout (ie: map from order in which keycodes are writen into row/col). starts at 1 to tell apart from KC_NO
__DUMMY_LAYER(
          1,       2,       3,       4,       5,       6,              7,       8,       9,      10,      11,      12,
         13,      14,      15,      16,      17,      18,             19,      20,      21,      22,      23,      24,
         25,      26,      27,      28,      29,      30,             31,      32,      33,      34,      35,      36,
         37,      38,      39,      40,      41,      42,             43,      44,      45,      46,      47,      48,
         49,      50,      51,      52,          53,                      54,           55,      56,      57,      58
)
