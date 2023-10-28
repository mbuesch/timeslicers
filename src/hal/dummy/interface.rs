// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

pub use crate::hal::dummy::{
    cpu::{current_core, CORES},
    task::task_spawn,
    time::now_us,
    timer::Timer,
};

// vim: ts=4 sw=4 expandtab
