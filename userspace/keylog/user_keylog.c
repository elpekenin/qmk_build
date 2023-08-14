// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "user_keylog.h"
#include "user_utils.h"

// ========================
// *** Number of layers ***
// ========================
#undef __DUMMY_LAYER
#define __DUMMY_LAYER(...)
#undef LAYER
#define LAYER(layer_name, ...) __##layer_name,
enum {
#include KEYMAP_LAYERS_H
    __N_LAYERS
};

// ==========================
// *** Internal variables ***
// ==========================
char *keycode_names[__N_LAYERS][MATRIX_ROWS][MATRIX_COLS];
char keylog[KEYLOG_SIZE + 1]; // extra spaced for terminator

// =========================
// *** row/col -> string ***
// =========================
#undef __DUMMY_LAYER
#define __DUMMY_LAYER(...)
#undef LAYER
#define LAYER(layer_name, ...) [layer_name] = "_" #__VA_ARGS__,
char *keycode_names_raw[] = {
#include KEYMAP_LAYERS_H
};

void populate_keycode_names(void) {
    // we only want to do this once
    static bool inited = false;
    if (inited)
        return;
    inited = true;

    // actual implementation
    for (uint8_t layer = 0; layer < __N_LAYERS; ++layer) {
        // copy the string
        uint16_t len  = strlen(keycode_names_raw[layer]) + 1;
        char    *copy = malloc(len);
        memcpy(copy, keycode_names_raw[layer], len);

        // location
        uint8_t index = 0;
        uint8_t row   = 0;
        uint8_t col   = 0;

        // get first ocurrence
        const char *delim = ",";
        char *token = strtok(copy, delim);

        // get the rest
        while (token != NULL) {
            // convert conter into row/col position, LAYOUT aware
            index_to_row_col(index, &row, &col);

            if (keycode_at_keymap_location(0, row, col) != KC_NO) {
                // copy into array, would need +1 for '\0' and - 1 to remove space v
                //                                                           (KC_A, KC_B)
                // note that stringifying VA_ARGS makes all keycodes to be 1-space away, regardless of input "shape"
                uint8_t len     = strlen(token);
                char *allocated = malloc(len);

                memcpy(allocated, token + 1, len);
                keycode_names[layer][row][col] = allocated;
            }

            // advance
            index++;
            token = strtok(NULL, delim);
        }

        free(copy);
    }
}

char *get_keycode_str_at(uint8_t layer_num, uint8_t row, uint8_t column) {
    populate_keycode_names();
    return keycode_names[layer_num][row][column];
}

// =========================
// *** Index <-> row/col ***
// =========================
#undef __DUMMY_LAYER
#define __DUMMY_LAYER(...) LAYOUT(__VA_ARGS__)
#undef LAYER
#define LAYER(...)
uint8_t col_row_to_index_mapping[MATRIX_ROWS][MATRIX_COLS] =
#include KEYMAP_LAYERS_H
;

bool index_to_row_col(uint8_t index, uint8_t *row, uint8_t *col) {
    // we need this correction as the counting on map is not 0-based
    // it is 1-based and 0 represents no key on the position
    uint8_t corrected_index = index + 1;

    for (uint8_t i = 0; i < MATRIX_ROWS; ++i) {
        for (uint8_t j = 0; j < MATRIX_COLS; ++j) {
            if (col_row_to_index_mapping[i][j] == corrected_index) {
                *row = i;
                *col = j;
                return true;
            }
        }
    }

    return false;
}

bool row_col_to_index(uint8_t row, uint8_t col, uint8_t *index) {
    uint8_t i = col_row_to_index_mapping[row][col];

    if (i != KC_NO) {
        *index = i - 1;
    }

    return i != KC_NO;
}

// ======================
// *** Number of keys ***
// ======================
#undef LAYER
#define LAYER(...)
#undef __DUMMY_LAYER
#define __DUMMY_LAYER(...) __VA_ARGS__
uint8_t __indexes[] = {
#include KEYMAP_LAYERS_H
};
enum {
    __N_KEYS = ARRAY_SIZE(__indexes)
};

uint8_t number_of_keys(void) {
    return __N_KEYS;
}

// ==========================
// *** Formatting helpers ***
/// =========================
void remove_prefixes(char **str) {
    char *prefixes[] = { "KC_", "RGB_", "QK_", "ES_", "TD_" };

    for (uint8_t i = 0; i < ARRAY_SIZE(prefixes); ++i) {
        char *  prefix = prefixes[i];
        uint8_t len    = strlen(prefix);

        if (strncmp(prefix, *str, len) == 0) {
            *str += len;
            return;
        }
    }
}

bool find_and_replace(char **str, char *find, char *replace) {
    if (strstr(*str, find) != NULL) {
        *str = replace;
        return true;
    }

    return false;
}

void replace_symbols(char **str) {
    str_replacement_t replacements[] = {
        REPLACE("SPC", " "),

        // arrow (ish)
        REPLACE("TL_UPPR", "▲"),
        REPLACE("TL_LOWR", "▼"),
        REPLACE("TAB", "⇥"),
        REPLACE("BSPC", "⇤"),
        REPLACE("CAPS", "↕"),
        REPLACE("ENT", "↲"),
        REPLACE("UP", "↑"),
        REPLACE("DOWN", "↓"),
        REPLACE("RIGHT", "→"),
        REPLACE("LEFT", "←"),

        // various symbols
        REPLACE("PLUS", "+"),
        REPLACE("MINS", "-"),
        REPLACE("NTIL", "´"),
        REPLACE("QUOT", "'"),
        REPLACE("GRV", "`"),
        REPLACE("COMM", ","),
        REPLACE("DOT", "."),
        REPLACE("BSLS", "\\"),
        REPLACE("HASH", "#"),
        REPLACE("AT", "@"),
        REPLACE("PIPE", "|"),
        REPLACE("TILD", "~"),
        REPLACE("LBRC", "["),
        REPLACE("RBRC", "]"),
        REPLACE("LCBR", "{"),
        REPLACE("RCBR", "}"),
        REPLACE("VOLU", "♪"),

        // shorter aliases
        REPLACE("DB_TOGG", "DBG"),
        REPLACE("XXXXXXX", "XX"),
        REPLACE("_______", "__"),
    };

    for (uint8_t i = 0; i < ARRAY_SIZE(replacements); ++i) {
        str_replacement_t replacement = replacements[i];
        char *            find        = replacement.find;
        char *            replace     = replacement.replace;

        // if we find a replacement, stop iterating
        if (find_and_replace(str, find, replace)) {
            return;
        }
    }
}

void replace_mods(char **str) {
    mod_replacement_t replacements[] = {
        REPLACE_SFT("1", "!"),
        REPLACE_SFT("2", "\""),
        // REPLACE_SFT("3", "·"),
        REPLACE_SFT("4", "$"),
        REPLACE_SFT("5", "%"),
        REPLACE_SFT("6", "&"),
        REPLACE_SFT("7", "/"),
        REPLACE_SFT("8", "("),
        REPLACE_SFT("9", ")"),
        REPLACE_SFT("0", "="),
        REPLACE_SFT("+", "*"),
        REPLACE_SFT("'", "?"),
        REPLACE_SFT("`", "^"),
        REPLACE_SFT(".", ":"),
        REPLACE_SFT(",", ";"),
        REPLACE_SFT("-", "_"),

        REPLACE_ALGR("1", "|"),
        REPLACE_ALGR("2", "@"),
        REPLACE_ALGR("3", "#"),
        REPLACE_ALGR("4", "~"),
    };


    uint8_t mods = modifiers();
    for (uint8_t i = 0; i < ARRAY_SIZE(replacements); ++i) {
        mod_replacement_t replacement = replacements[i];
        uint8_t           mask        = replacement.mod_mask;
        char *            find        = replacement.replace.find;
        char *            replace     = replacement.replace.replace;

        // if we find a replacement, stop iterating
        if ((mods & mask) && find_and_replace(str, find, replace)) {
            return;
        }
    }
}

void keycode_repr(char **str) {
    remove_prefixes(str);
    replace_symbols(str);
    replace_mods(str);
}

// convert to lowercase based on shift/caps
// overengineered so it can also work on strings and whatnot on future
void apply_casing(char **str) {
    // not a single char
    if (strlen(*str) > 1) {
        return;
    }

    // not a letter
    if (**str < 'A' || **str > 'Z') {
        return;
    }

    uint8_t mods  = modifiers();
    bool    shift = mods & MOD_MASK_SHIFT;
    bool    caps  = host_keyboard_led_state().caps_lock;

    // if writing uppercase, string already is, just return
    if (shift ^ caps) {
        return;
    }

    char *lowercase_letters[] = { "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z" };

    *str = lowercase_letters[**str - 'A'];
}

// ========================
// *** Updating the log ***
// ========================
bool is_utf8_continuation(char c) {
    return (c & 0xC0) == 0x80;
}

bool is_utf8(char c) {
    return (c & 0xF0);
}

void keylog_clear(void) {
    // spaces (not 0) so `qp_drawtext` actually renders something
    memset(keylog, ' ', KEYLOG_SIZE);
    keylog[KEYLOG_SIZE] = '\0';
}

void _keylog_shift_right_byte(void) {
    for (uint8_t i = KEYLOG_SIZE - 1; i > 0; --i) {
        keylog[i] = keylog[i - 1];
    }
    keylog[0] = ' ';
}

void keylog_shift_right(void) {
    // pop all utf-continuation bytes
    while (is_utf8_continuation(keylog[KEYLOG_SIZE - 1])) {
       _keylog_shift_right_byte();
    }

    // this is either an ascii char or the heading byte of utf
    _keylog_shift_right_byte();
}

void keylog_shift_left(uint8_t len) {
    memmove(keylog, keylog + len, KEYLOG_SIZE - len);

    uint8_t counter = 0;
    while (is_utf8_continuation(keylog[0])) {
        memmove(keylog, keylog + 1, KEYLOG_SIZE - 1);
        ++counter;
    }

    // pad buffer to the right, to align after a utf8 symbol is deleted
    memmove(keylog + counter, keylog, KEYLOG_SIZE - counter);
    memset(keylog, ' ', counter);
}

void keylog_append(char *str) {
    uint8_t len = strlen(str);

    keylog_shift_left(len);
    for (uint8_t i = 0; i < len; ++i) {
        keylog[KEYLOG_SIZE - len + i] = str[i];
    }
}

// ======================
// *** Event handling ***
// ======================
void keylog_process(uint16_t keycode, keyrecord_t *record) {
    // initial setup
    static bool keylog_init = false;
    if (!keylog_init) {
        keylog_clear();
        keylog_init = true;
    }

    // nothing on release (for now)
    if (!record->event.pressed) {
        return;
    }

    // dont want to show some keycodes
    if ((IS_QK_LAYER_TAP(keycode) && !record->tap.count)
        || keycode >= QK_USER  // dont want my custom keycodes on keylog
        || IS_RGB_KEYCODE(keycode)
        || IS_QK_LAYER_MOD(keycode)
        || IS_QK_MOMENTARY(keycode)
        || IS_QK_DEF_LAYER(keycode)
        || IS_MODIFIER_KEYCODE(keycode)
       )
    {
        return;
    }

    // get the string representation of the pressed key
    uint8_t layer_num = get_highest_layer(layer_state);
    uint8_t row       = record->event.key.row;
    uint8_t column    = record->event.key.col;
    char *  str       = get_keycode_str_at(layer_num, row, column);

    uint8_t mods = modifiers();
    bool    ctrl = mods & MOD_MASK_CTRL;

    // delete from tail
    if (strstr(str, "BSPC") != NULL) {
        // ctrl + backspace clears whole log
        if (ctrl) {
            keylog_clear();
        }
        // backspace = remove last char
        else {
            keylog_shift_right();
        }
        return;
    }

    // convert string into symbols
    keycode_repr(&str);

    // casing is separate so that drawing keycodes on screen is always uppercase
    apply_casing(&str);

    keylog_append(str);
}
