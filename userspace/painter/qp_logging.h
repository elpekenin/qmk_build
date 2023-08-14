// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

#include "deferred_exec.h"

#if !defined(LOG_N_LINES)
#    define LOG_N_LINES 9
#endif // !defined(LOG_N_LINES)

#if !defined(LOG_N_CHARS)
#    define LOG_N_CHARS 60
#endif // !defined(LOG_N_CHARS)

#undef QUANTUM_PAINTER_CONCURRENT_SCROLLING_TEXTS
#define QUANTUM_PAINTER_CONCURRENT_SCROLLING_TEXTS (LOG_N_LINES + 5)

extern deferred_token   qp_log_tokens[LOG_N_LINES];
extern char            *qp_log_pointers[LOG_N_LINES];
extern bool             qp_log_redraw;

bool elpekenin_sendchar_hook(uint8_t c);
int8_t elpekenin_sendchar(uint8_t c);
