// Copyright 2022 Jose Pablo Ramirez (@jpe230)
// SPDX-License-Identifier: GPL-2.0-or-later

#include QMK_KEYBOARD_H
#include "color.h"
#include "qp.h"
#include "qp_st77xx_opcodes.h"
#include "qp_st7735_opcodes.h"
#include "qp_comms.h"

const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
    LAYOUT_ortho_1x1(KC_A)
};

painter_device_t lcd;

uint32_t deferred_init(uint32_t trigger_time, void *cb_arg) {
    setPinOutput(LCD_BL_PIN);
    writePinLow(LCD_BL_PIN);

    setPinInput(SD_DETECTION);

    lcd = qp_st7735_make_spi_device(80, 160, LCD_CS_PIN, LCD_DC_PIN, LCD_RST_PIN, 128, 0);
    qp_init(lcd, QP_ROTATION_0);
    qp_rect(lcd, 0, 0, 79, 159, HSV_RED, true);

#if defined(FILESYSTEM_ENABLE)
    uint32_t boot_count = 0;

    dprintf("Initing\n");
    wait_ms(1000);

    if (!fs_init()) {
        dprintf("fs error: init\n");
        return 0;
    }

    if (!fs_mount()) {
        dprintf("fs error: mounting\n");
        return 0;
    }

    fs_fd_t fd = fs_open("boot_count", "rw");
    if (fd == INVALID_FILESYSTEM_FD) {
        dprintf("fs error: open\n");
        fs_unmount();
        return 0;
    }

    fs_read(fd, &boot_count, sizeof(boot_count));
    fs_seek(fd, 0, FS_SEEK_SET);
    ++boot_count;
    fs_write(fd, &boot_count, sizeof(boot_count));
    fs_close(fd);

    fs_unmount();

    dprintf("boot_count: %d\n", (int)boot_count);

    // char buffer[20] = {0};
    // snprintf(buffer, ARRAY_SIZE(buffer), "boot_count: %d", boot_count);
    // qp_drawtext(lcd, 0, 0, )
#endif // defined(FILESYSTEM_ENABLE)

    return 0;
}

void keyboard_post_init_user(void) {
    debug_enable = true;

    setPinOutput(LED);
    writePinLow(LED);

    defer_exec(3000, deferred_init, NULL);
}