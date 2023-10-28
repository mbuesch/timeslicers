// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

pub fn task_spawn<F, T>(_name: &'static str, _core: usize, _stack_size: usize, _f: F)
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
}

// vim: ts=4 sw=4 expandtab
