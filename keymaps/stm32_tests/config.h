// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

#define QUANTUM_PAINTER_DEBUG
#define FILESYSTEM_DEBUG
#define LFS_YES_TRACE

#define LED E3

/* SPI pins */
#define SPI_DRIVER SPID4
#define SPI_SCK_PIN E12
#define SPI_MOSI_PIN E14
#define SPI_MISO_PIN E5

/* LCD configuration */
#define LCD_RST_PIN NO_PIN
#define LCD_DC_PIN E13
#define LCD_CS_PIN E11
#define QUANTUM_PAINTER_DISPLAY_TIMEOUT 0
#define LCD_BL_PIN E10

/* SD pins */
#define SD_DETECTION D4 /* Unused */
#define SD_DRIVER SDCD1
