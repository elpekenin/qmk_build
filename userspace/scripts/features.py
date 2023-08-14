#! /usr/bin/env python3
# -*- coding: utf-8 -*-

# Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
# SPDX-License-Identifier: GPL-2.0-or-later

import sys
from pathlib import Path
from typing import Callable

from scripts import *

# == User configuration here ==
TEXT_COLOR = "HSV_BLACK"
BACKGROUND_COLOR = "HSV_WHITE"

# Capitalization here doesnt matter, code will format it
FEATURES = sorted({
    "AUDIO",
    "AUTOCORRECT",
    "BOOTMAGIC",
    "COMBO",
    "EXTRAKEY",
    "KEY_OVERRIDE",
    "MOUSEKEY",
    "NKRO",
    "RGB_MATRIX",
    "SIPO_PINS",
    "TAP_DANCE",
    "TOUCH_SCREEN",
    "QP_XAP",
    "QUANTUM_PAINTER",
    "UNICODE_COMMON",
    "WPM",
    "XAP",
})
SHORT_NAMES = {
    "QUANTUM_PAINTER": "PAINTER",
    "UNICODE_COMMON": "UNICODE",
}

# == Do not edit past here ==
OUTPUT_NAME = "generated_features"
MAX_WIDTH = max(map(len, FEATURES))

# ===== Templates
H_FILE = "\n".join([
    H_HEADER,
    "",
    "#include <stdbool.h>",
    "#include <stdint.h>",
    "",
    "typedef union {{",
    "    {type} raw;",
    "    struct {{",
            "{generated_code}",
    "    }};",
    "}} enabled_features_t;",
    "",
    "enabled_features_t get_enabled_features(void);",
    ""
])

C_FILE = "\n".join([
    C_HEADER,
    "",
    f'#include "{OUTPUT_NAME}.h"',
    "",
    "enabled_features_t get_enabled_features(void) {{",
    "    enabled_features_t features;",
    "",
    "    features.raw = 0;",
    "",
        "{generated_code}",
    "    return features;",
    "}}",
    ""
])

DRAW_FILE = "\n".join([
    C_HEADER,
    "",
    '#include "color.h"',
    "",
    f'#include "{OUTPUT_NAME}.h"',
    '#include "graphics.h"',
    '#include "user_data.h"',
    "",
    "void draw_features(painter_device_t device) {{",
    "    enabled_features_t    features    = user_data.features;",
    "    painter_font_handle_t font        = qp_fonts[1];",
    "    uint8_t               font_height = font->line_height;",
    "    uint8_t               x           = 0;",
    "    uint8_t               y           = 0;",
    "",
    "    uint16_t width;",
    "    uint16_t height;",
    "    qp_get_geometry(device, &width, &height, NULL, NULL, NULL);",
    "",
    "    bool shifted = false;",
    "",
        "{generated_code}"  # no comma here intentionally
    "}}"
])


def _get_type() -> str:
    bits = len(FEATURES)
    for size in (8, 16, 32, 64):
        if bits <= size:
            return f"uint{size}_t"

    raise ValueError("Too many features, unsupported")


def _for_all_features(func: Callable) -> str:
    return "\n".join(
        func(feature)
        for feature in FEATURES
    )


def _h_generator(feature: str) -> str:
    return f"        bool {feature.lower().ljust(MAX_WIDTH)}: 1;"


def _c_generator(feature: str) -> str:
    return "\n".join([
        f"    #if defined({feature.upper()}_ENABLE)",
        f"        features.{feature.lower()} = true;",
        f"    #endif // defined({feature.upper()}_ENABLE)",
        ""
    ])


def _draw_generator(feature: str) -> str:
    # get alias or keep as is
    short_name = SHORT_NAMES.get(feature, feature)
    name = short_name.replace("_", " ").title()

    return "\n".join([
        f'    qp_drawtext_recolor(device, x, y, font, features.{feature.lower()} ? "{name}: On " : "{name}: Off", {TEXT_COLOR}, {BACKGROUND_COLOR});',
        #                         intentional space so it overwrites previous "Off"         ^^^
        "    y += font_height;",
        "    // next text doesnt fit vertically",
        "    if ((y + font_height) > height) {",
        "        // shift half the screen to the right, if not done already",
        "        if (!shifted) {",
        "            shifted = true;",
        "            x = width / 2;",
        "            y = 0;",
        "        } else {",
        '           qp_dprintf("Cant fit more features on the display\\n");',
        "           return;",
        "        }",
        "    }",
        "",
    ])


if __name__ == "__main__":
    # -- Handle args
    if len(sys.argv) != 2:
        print(f"{CLI_ERROR} {current_filename(__file__)} <output_path>")
        exit(1)

    output_dir = Path(sys.argv[1])
    if not output_dir.exists() or not output_dir.is_dir():
        print(f"Invalid path {output_dir}")
        exit(1)

    # Gen files
    type_ = _get_type()  # Work out the union type needed
    gen_h = _for_all_features(_h_generator)
    with open(output_dir / f"{OUTPUT_NAME}.h", "w") as f:
        f.write(H_FILE.format(type=type_, generated_code=gen_h))


    gen_c = _for_all_features(_c_generator)
    with open(output_dir / f"{OUTPUT_NAME}.c", "w") as f:
        f.write(C_FILE.format(generated_code=gen_c))

    gen_draw = _for_all_features(_draw_generator)
    with open(output_dir / f"{OUTPUT_NAME}_draw.c", "w") as f:
        f.write(DRAW_FILE.format(generated_code=gen_draw))

else:
    print("Dont try to import this")
    exit(1)