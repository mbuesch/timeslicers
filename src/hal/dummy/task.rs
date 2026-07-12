// -*- coding: utf-8 -*-
//
// Copyright 2023-2026 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

use core::ffi::CStr;

pub fn task_spawn<F, T>(
    _name: &'static CStr,
    _core: usize,
    _priority: u8,
    _stack_size: usize,
    _f: F,
) where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
}

// vim: ts=4 sw=4 expandtab
