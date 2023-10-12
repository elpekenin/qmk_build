// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include QMK_KEYBOARD_H
#include "color.h"
#include "qp.h"
#include "qp_st77xx.h"

#include "fira_code.qff.h"

const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
    LAYOUT_ortho_1x1(KC_A)
};

painter_device_t lcd;
painter_font_handle_t font;

uint8_t buff[MMCSD_BLOCK_SIZE];
uint8_t *boot_count = &buff[10]; // random position in the middle of it

uint32_t deferred_init(uint32_t trigger_time, void *cb_arg) {
    char *fail = "Fail occurred";
    char count_buff[20] = {0};

    setPinOutput(LCD_BL_PIN);
    writePinLow(LCD_BL_PIN);

    setPinInput(SD_DETECTION);

    lcd = qp_st7735_make_spi_device(80, 160, LCD_CS_PIN, LCD_DC_PIN, LCD_RST_PIN, 128, 0);
    qp_init(lcd, QP_ROTATION_90);
    qp_st77xx_set_inversion(lcd, true);
    qp_rect(lcd, 0, 0, 160, 80, HSV_BLUE, true);

    font = qp_load_font_mem(font_fira_code);

    if (!fs_init()) {
        goto exit;
    }

    if (!fs_mount()) {
        goto exit;
    }

    fs_fd_t fd = fs_open("boot_count", "rw");
    if (fd == INVALID_FILESYSTEM_FD) {
        goto unmount;
    }

    fs_read(fd, boot_count, sizeof(uint8_t));
    fs_seek(fd, 0, FS_SEEK_SET);
    (*boot_count)++;
    fs_write(fd, boot_count, sizeof(uint8_t));
    fs_close(fd);

    fs_unmount();

    snprintf(count_buff, ARRAY_SIZE(count_buff), "boot_count: %d", *boot_count);

    dprintf("%s\n", count_buff);
    qp_drawtext_recolor(lcd, 30, 30, font, count_buff, HSV_WHITE, HSV_BLUE);

    return 0;

unmount:
    fs_unmount();
exit:
    fs_dprintf("%s\n", fail);
    qp_drawtext_recolor(lcd, 30, 30, font, fail, HSV_WHITE, HSV_BLUE);
    return 0;
}

// void __cpu_init(void) {
//     // SCB_DisableICache();
//     SCB_DisableDCache();
// }

void keyboard_post_init_user(void) {
    setPinOutput(LED);
    writePinHigh(LED);

    debug_enable = true;
    defer_exec(3000, deferred_init, NULL);
}
