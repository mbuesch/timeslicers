// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

pub struct RuntimeMeas {}

impl RuntimeMeas {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }

    #[inline(always)]
    pub fn meas_begin(&self) -> i64 {
        0
    }

    #[inline(always)]
    pub fn meas_end(&self, _task_name: &'static str, _core: usize, _begin: i64) {}

    #[inline(always)]
    pub fn print_cpus(&self) {}

    #[inline(always)]
    pub fn print_task(&self, _task_name: &'static str, _time_base: usize, _core: usize) {}

    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        false
    }

    #[inline(always)]
    pub fn enable(&self, _en: bool) {}
}

// vim: ts=4 sw=4 expandtab
