// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

#include "filesystem.h"
#include "lfs.h"

#if !defined(PROTOCOL_CHIBIOS)
#    error SD/MMC is only supported on ChibiOS at the moment
#endif

#include <hal.h>

bool fs_device_init(void) {
    // sdcInit done by hal.c

    // NULL -> default config
    fs_dprintf("init\n");

    return sdcStart(&SD_DRIVER, NULL) >= 0;
}

int fs_device_read(const struct lfs_config *c, lfs_block_t block, lfs_off_t off, void *buffer, lfs_size_t size) {
    fs_dprintf("read\n");

    bool ret = sdcConnect(&SD_DRIVER);
    ret &= sdcRead(&SD_DRIVER, block * c->block_size + off, buffer, size);
    ret &= sdcDisconnect(&SD_DRIVER);

    return ret ? -1 : 0;
}

int fs_device_prog(const struct lfs_config *c, lfs_block_t block, lfs_off_t off, const void *buffer, lfs_size_t size) {
    fs_dprintf("prog\n");

    bool ret = sdcConnect(&SD_DRIVER);
    ret &= sdcWrite(&SD_DRIVER, block * c->block_size + off, buffer, size);
    ret &= sdcDisconnect(&SD_DRIVER);

    return ret ? -1 : 0;
}

int fs_device_erase(const struct lfs_config *c, lfs_block_t block) {
    fs_dprintf("erase\n");

    bool ret = sdcConnect(&SD_DRIVER);
    ret &= sdcErase(&SD_DRIVER, block * c->block_size, (block + 1) * c->block_size);
    ret &= sdcDisconnect(&SD_DRIVER);

    return ret ? -1 : 0;
}

int fs_device_sync(const struct lfs_config *c) {
    fs_dprintf("sync\n");
    return 0;
}

uint8_t lfs_fs_device_read_buf[MMCSD_BLOCK_SIZE];
uint8_t lfs_fs_device_prog_buf[MMCSD_BLOCK_SIZE];
uint8_t lfs_fs_device_lookahead_buf[MMCSD_BLOCK_SIZE];

// configuration of the filesystem is provided by this struct
const struct lfs_config lfs_cfg = {
    // block device operations
    .read  = fs_device_read,
    .prog  = fs_device_prog,
    .erase = fs_device_erase,
    .sync  = fs_device_sync,

    // block device configuration
    .read_size      = MMCSD_BLOCK_SIZE,
    .prog_size      = MMCSD_BLOCK_SIZE,
    .block_size     = MMCSD_BLOCK_SIZE,
    .block_count    = 1,
    .cache_size     = MMCSD_BLOCK_SIZE,
    .lookahead_size = MMCSD_BLOCK_SIZE,

    .read_buffer      = lfs_fs_device_read_buf,
    .prog_buffer      = lfs_fs_device_prog_buf,
    .lookahead_buffer = lfs_fs_device_lookahead_buf
};
