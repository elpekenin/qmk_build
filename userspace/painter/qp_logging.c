// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include <string.h>

#include "qp_logging.h"

char                 *qp_log_pointers[LOG_N_LINES];
deferred_token        qp_log_tokens[LOG_N_LINES];
bool                  qp_log_redraw;

static char           qp_log[LOG_N_LINES][LOG_N_CHARS + 1];
static uint8_t        qp_log_current_col;

bool elpekenin_sendchar_hook(uint8_t c) {
    // Setup the arrays on the 1st go
    static bool initialized = false;
    if (!initialized) {
        memset(qp_log, 0, sizeof(qp_log));
        for (uint8_t i = 0; i < LOG_N_LINES; ++i) {
            qp_log_pointers[i] = qp_log[i];
            qp_log_tokens[i]   = INVALID_DEFERRED_TOKEN;
        }
        qp_log_redraw = false;
        initialized = true;
    }

    if (c == '\n') {
        // Add null pointer to current line
        qp_log_pointers[LOG_N_LINES - 1][qp_log_current_col] = 0;

        // Move everything 1 line upwards
        char *temp = qp_log_pointers[0];
        for (uint8_t i = 0; i < LOG_N_LINES - 1; ++i) {
            qp_log_pointers[i] = qp_log_pointers[i + 1];
        }
        qp_log_pointers[LOG_N_LINES - 1] = temp;

        // Reset stuff
        qp_log_current_col                                   = 0;
        qp_log_pointers[LOG_N_LINES - 1][qp_log_current_col] = 0;
        qp_log_redraw                                        = true;
    } else if (qp_log_current_col >= LOG_N_CHARS) {
        return false;
    } else {
        qp_log_pointers[LOG_N_LINES - 1][qp_log_current_col++] = c;
        qp_log_pointers[LOG_N_LINES - 1][qp_log_current_col]   = 0;
        qp_log_redraw                                          = true;
    }

    return true;
}

int8_t elpekenin_sendchar(uint8_t c) {
    // logging on QP
    elpekenin_sendchar_hook(c);

    // default logging (USB)
    extern int8_t sendchar(uint8_t c);
    return sendchar(c);
}
