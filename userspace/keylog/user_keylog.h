// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

#include QMK_KEYBOARD_H
#include "user_layers.h"
#include "keymap_introspection.h"

#ifndef KEYLOG_SIZE
#    define KEYLOG_SIZE 40
#endif
// this is setup with an extra byte to try and ensure we always have a '\0'
extern char keylog[KEYLOG_SIZE + 1];

extern bool qp_log_redraw;

char *get_keycode_str_at(uint8_t layer_num, uint8_t row, uint8_t column);

bool index_to_row_col(uint8_t index, uint8_t *row, uint8_t *col);
bool row_col_to_index(uint8_t row, uint8_t col, uint8_t *index);

uint8_t number_of_keys(void);

void keylog_process(uint16_t keycode, keyrecord_t *record);

typedef struct {
    char *find;
    char *replace;
} str_replacement_t;
#define REPLACE(f, r) (str_replacement_t){.find = f, .replace = r}


typedef struct {
    str_replacement_t replace;
    uint8_t           mod_mask;
} mod_replacement_t;
#define MOD_REPLACE(f, r, m) (mod_replacement_t){.replace = REPLACE(f, r), .mod_mask = m}

#define REPLACE_ALGR(f, r) MOD_REPLACE(f, r, MOD_BIT(KC_ALGR))
#define REPLACE_SFT(f, r)  MOD_REPLACE(f, r, MOD_MASK_SHIFT)

void keycode_repr(char **str);
