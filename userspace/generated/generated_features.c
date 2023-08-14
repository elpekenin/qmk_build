// THIS FILE WAS GENERATED

// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "generated_features.h"

enabled_features_t get_enabled_features(void) {
    enabled_features_t features;

    features.raw = 0;

    #if defined(AUDIO_ENABLE)
        features.audio = true;
    #endif // defined(AUDIO_ENABLE)

    #if defined(AUTOCORRECT_ENABLE)
        features.autocorrect = true;
    #endif // defined(AUTOCORRECT_ENABLE)

    #if defined(BOOTMAGIC_ENABLE)
        features.bootmagic = true;
    #endif // defined(BOOTMAGIC_ENABLE)

    #if defined(COMBO_ENABLE)
        features.combo = true;
    #endif // defined(COMBO_ENABLE)

    #if defined(EXTRAKEY_ENABLE)
        features.extrakey = true;
    #endif // defined(EXTRAKEY_ENABLE)

    #if defined(KEY_OVERRIDE_ENABLE)
        features.key_override = true;
    #endif // defined(KEY_OVERRIDE_ENABLE)

    #if defined(MOUSEKEY_ENABLE)
        features.mousekey = true;
    #endif // defined(MOUSEKEY_ENABLE)

    #if defined(NKRO_ENABLE)
        features.nkro = true;
    #endif // defined(NKRO_ENABLE)

    #if defined(QP_XAP_ENABLE)
        features.qp_xap = true;
    #endif // defined(QP_XAP_ENABLE)

    #if defined(QUANTUM_PAINTER_ENABLE)
        features.quantum_painter = true;
    #endif // defined(QUANTUM_PAINTER_ENABLE)

    #if defined(RGB_MATRIX_ENABLE)
        features.rgb_matrix = true;
    #endif // defined(RGB_MATRIX_ENABLE)

    #if defined(SIPO_PINS_ENABLE)
        features.sipo_pins = true;
    #endif // defined(SIPO_PINS_ENABLE)

    #if defined(TAP_DANCE_ENABLE)
        features.tap_dance = true;
    #endif // defined(TAP_DANCE_ENABLE)

    #if defined(TOUCH_SCREEN_ENABLE)
        features.touch_screen = true;
    #endif // defined(TOUCH_SCREEN_ENABLE)

    #if defined(UNICODE_COMMON_ENABLE)
        features.unicode_common = true;
    #endif // defined(UNICODE_COMMON_ENABLE)

    #if defined(WPM_ENABLE)
        features.wpm = true;
    #endif // defined(WPM_ENABLE)

    #if defined(XAP_ENABLE)
        features.xap = true;
    #endif // defined(XAP_ENABLE)

    return features;
}
