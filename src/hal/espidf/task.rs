// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

use core::ffi::CStr;
use esp_idf_hal::task::thread::ThreadSpawnConfiguration;

pub fn task_spawn<F, T>(name: &'static CStr, core: usize, stack_size: usize, f: F)
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    // Configure a new thread.
    let mut conf: ThreadSpawnConfiguration = Default::default();
    conf.name = Some(name);
    conf.inherit = true;
    conf.stack_size = stack_size;
    conf.priority = 5;
    conf.pin_to_core = Some((core as i32).into());
    ThreadSpawnConfiguration::set(&conf).expect("Failed to set thread configuration.");

    // Spawn the thread.
    std::thread::Builder::new()
        .name(name.to_str().unwrap().to_string())
        .stack_size(stack_size)
        .spawn(f)
        .expect("Failed to spawn timeslice_sched thread.");

    ThreadSpawnConfiguration::set(&Default::default())
        .expect("Failed to set thread configuration.");
}

// vim: ts=4 sw=4 expandtab
