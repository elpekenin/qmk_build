// THIS FILE WAS GENERATED

// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

#include <stdbool.h>
#include <stdint.h>

typedef union {
    uint32_t raw;
    struct {
        bool audio          : 1;
        bool autocorrect    : 1;
        bool bootmagic      : 1;
        bool combo          : 1;
        bool extrakey       : 1;
        bool key_override   : 1;
        bool mousekey       : 1;
        bool nkro           : 1;
        bool qp_xap         : 1;
        bool quantum_painter: 1;
        bool rgb_matrix     : 1;
        bool sipo_pins      : 1;
        bool tap_dance      : 1;
        bool touch_screen   : 1;
        bool unicode_common : 1;
        bool wpm            : 1;
        bool xap            : 1;
    };
} enabled_features_t;

enabled_features_t get_enabled_features(void);
