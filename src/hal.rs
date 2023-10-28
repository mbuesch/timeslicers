// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

cfg_if::cfg_if! {
    if #[cfg(feature = "hal-espidf")] {
        mod espidf;
        pub use espidf::interface::*;
    } else if #[cfg(feature = "hal-dummy")] {
        mod dummy;
        pub use dummy::interface::*;
    } else {
        compile_error!("timeslice: Must select one of the hal-* features.");
    }
}

// vim: ts=4 sw=4 expandtab
