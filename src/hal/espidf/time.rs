// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

#[inline]
pub fn now_us() -> u32 {
    let systime = esp_idf_svc::systime::EspSystemTime;
    systime.now().as_micros() as u32
}

// vim: ts=4 sw=4 expandtab
