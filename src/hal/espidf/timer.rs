// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

use esp_idf_svc::timer::{EspTimer, EspTimerService, Task};
use std::time::Duration;

pub struct Timer<'a> {
    _timsvc: EspTimerService<Task>,
    _tim: EspTimer<'a>,
}

impl<'a> Timer<'a> {
    pub fn new<F>(callback: F, period: Duration) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        let timsvc = EspTimerService::new().expect("Failed to create system timer service.");
        let tim = timsvc
            .timer(callback)
            .expect("Failed to create system timer.");
        tim.every(period).expect("Failed to start system timer.");
        Self {
            _timsvc: timsvc,
            _tim: tim,
        }
    }
}

// vim: ts=4 sw=4 expandtab
