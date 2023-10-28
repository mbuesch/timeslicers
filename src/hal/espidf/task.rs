// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

use esp_idf_hal::task::thread::ThreadSpawnConfiguration;

pub fn task_spawn<F, T>(name: &'static str, core: usize, stack_size: usize, f: F)
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let core = match core {
        0 => esp_idf_hal::cpu::Core::Core0,
        1 => esp_idf_hal::cpu::Core::Core1,
        _ => unreachable!(),
    };

    // Configure a new thread.
    let cname = name.as_bytes();
    assert_eq!(cname[cname.len() - 1], b'\0');
    ThreadSpawnConfiguration::set(&ThreadSpawnConfiguration {
        name: Some(cname),
        inherit: true,
        stack_size,
        priority: 5,
        pin_to_core: Some(core),
    })
    .expect("Failed to set thread configuration.");

    // Spawn the thread.
    std::thread::Builder::new()
        .name(name[..name.len() - 1].to_string())
        .stack_size(stack_size)
        .spawn(f)
        .expect("Failed to spawn timeslice_sched thread.");

    ThreadSpawnConfiguration::set(&Default::default())
        .expect("Failed to set thread configuration.");
}

// vim: ts=4 sw=4 expandtab
