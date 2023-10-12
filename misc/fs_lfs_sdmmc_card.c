// Copyright 2023 Pablo Martinez (@elpekenin) <elpekenin@elpekenin.dev>
// SPDX-License-Identifier: GPL-2.0-or-later

// TODO: offset support

#include "filesystem.h"
#include "lfs.h"

#if !defined(PROTOCOL_CHIBIOS)
#    error SD/MMC is only supported on ChibiOS at the moment
#endif

#include <hal.h>

#define BYTES_TO_BLOCKS(__bytes) ((__bytes + (MMCSD_BLOCK_SIZE) - 1) / (MMCSD_BLOCK_SIZE))

bool fs_device_init(void) {
    fs_dprintf("init\n");
    // NULL -> default config
    return sdcStart(&SD_DRIVER, NULL) >= 0;
}

int fs_device_read(const struct lfs_config *c, lfs_block_t block, lfs_off_t off, void *buffer, lfs_size_t size) {
    fs_dprintf("read\n");

    if (off != 0) {
        fs_dprintf("error: offset != 0 is unsupported for SD cards\n");
        return LFS_ERR_INVAL;
    }

    if (sdcConnect(&SD_DRIVER) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    if (sdcRead(&SD_DRIVER, block, buffer, BYTES_TO_BLOCKS(size)) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    if (sdcDisconnect(&SD_DRIVER) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    return 0;
}

int fs_device_prog(const struct lfs_config *c, lfs_block_t block, lfs_off_t off, const void *buffer, lfs_size_t size) {
    fs_dprintf("prog\n");

    if (off != 0) {
        fs_dprintf("error: offset != 0 is unsupported for SD cards\n");
        return LFS_ERR_INVAL;
    }

    if (sdcConnect(&SD_DRIVER) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    if (sdcWrite(&SD_DRIVER, block, buffer, BYTES_TO_BLOCKS(size)) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    if (sdcDisconnect(&SD_DRIVER) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    return 0;
}

int fs_device_erase(const struct lfs_config *c, lfs_block_t block) {
    fs_dprintf("erase\n");

    if (sdcConnect(&SD_DRIVER) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    if (sdcErase(&SD_DRIVER, block, (block + 1)) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    if (sdcDisconnect(&SD_DRIVER) != HAL_SUCCESS) {
        return LFS_ERR_IO;
    }

    return 0;
}

int fs_device_sync(const struct lfs_config *c) {
    fs_dprintf("sync\n");
    return 0;
}

static uint8_t lfs_fs_device_read_buf[MMCSD_BLOCK_SIZE];
static uint8_t lfs_fs_device_prog_buf[MMCSD_BLOCK_SIZE];
static uint8_t lfs_fs_device_lookahead_buf[MMCSD_BLOCK_SIZE];

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
    .block_count    = (2 * 32 * 1024), // 2 * 32 * 1024 (* 512 bytes, aka MMCSD_BLOCK_SIZE) == 32MB
    .block_cycles   = 100,
    .cache_size     = MMCSD_BLOCK_SIZE,
    .lookahead_size = MMCSD_BLOCK_SIZE,

    .read_buffer      = lfs_fs_device_read_buf,
    .prog_buffer      = lfs_fs_device_prog_buf,
    .lookahead_buffer = lfs_fs_device_lookahead_buf
};
