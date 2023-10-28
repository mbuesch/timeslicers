// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

use esp_idf_hal::cpu::{core, Core};

pub const CORES: usize = esp_idf_hal::cpu::CORES as usize;

pub fn current_core() -> usize {
    match core() {
        Core::Core0 => 0,
        _ => 1,
    }
}

// vim: ts=4 sw=4 expandtab
