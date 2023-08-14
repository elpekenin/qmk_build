// THIS FILE WAS GENERATED

// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "color.h"

#include "generated_features.h"
#include "graphics.h"
#include "user_data.h"

void draw_features(painter_device_t device) {
    enabled_features_t    features    = user_data.features;
    painter_font_handle_t font        = qp_fonts[1];
    uint8_t               font_height = font->line_height;
    uint8_t               x           = 0;
    uint8_t               y           = 0;

    uint16_t width;
    uint16_t height;
    qp_get_geometry(device, &width, &height, NULL, NULL, NULL);

    bool shifted = false;

    qp_drawtext_recolor(device, x, y, font, features.audio ? "Audio: On " : "Audio: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.autocorrect ? "Autocorrect: On " : "Autocorrect: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.bootmagic ? "Bootmagic: On " : "Bootmagic: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.combo ? "Combo: On " : "Combo: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.extrakey ? "Extrakey: On " : "Extrakey: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.key_override ? "Key Override: On " : "Key Override: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.mousekey ? "Mousekey: On " : "Mousekey: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.nkro ? "Nkro: On " : "Nkro: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.qp_xap ? "Qp Xap: On " : "Qp Xap: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.quantum_painter ? "Painter: On " : "Painter: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.rgb_matrix ? "Rgb Matrix: On " : "Rgb Matrix: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.sipo_pins ? "Sipo Pins: On " : "Sipo Pins: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.tap_dance ? "Tap Dance: On " : "Tap Dance: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.touch_screen ? "Touch Screen: On " : "Touch Screen: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.unicode_common ? "Unicode: On " : "Unicode: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.wpm ? "Wpm: On " : "Wpm: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }

    qp_drawtext_recolor(device, x, y, font, features.xap ? "Xap: On " : "Xap: Off", HSV_BLACK, HSV_WHITE);
    y += font_height;
    // next text doesnt fit vertically
    if ((y + font_height) > height) {
        // shift half the screen to the right, if not done already
        if (!shifted) {
            shifted = true;
            x = width / 2;
            y = 0;
        } else {
           qp_dprintf("Cant fit more features on the display\n");
           return;
        }
    }
}