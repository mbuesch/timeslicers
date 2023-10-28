// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

cfg_if::cfg_if! {
    if #[cfg(feature = "meas")] {
        mod enabled;
        pub use enabled::RuntimeMeas;
    } else {
        mod disabled;
        pub use disabled::RuntimeMeas;
    }
}

// vim: ts=4 sw=4 expandtab
