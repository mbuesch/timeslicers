// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

//! # Simple time slice scheduler
//!
//! ## Example code
//!
//! ```
//! // Define the scheduler, its tasks and behavior.
//! timeslice::define_sched! {
//!     name: sched_main,
//!     num_objs: 2,
//!     tasks: {
//!         { name: task_10ms, period: 10 ms, cpu: 0, stack: 16 kiB },
//!         { name: task_50ms, period: 50 ms, cpu: 0, stack: 3 kiB },
//!         { name: task_100ms, period: 100 ms, cpu: 1, stack: 16 kiB },
//!         { name: task_1000ms, period: 1000 ms, cpu: 1, stack: 3 kiB },
//!     },
//! }
//!
//! struct MyThing1 {
//!     // ...
//! }
//!
//! impl MyThing1 {
//!     fn new() -> Self {
//!         Self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl sched_main::Ops for MyThing1 {
//!     fn task_10ms(&self) {
//!         // Called every 10 ms.
//!         // ... Put your code here ...
//!     }
//!
//!     fn task_50ms(&self) {
//!         // Called every 50 ms.
//!         // ... Put your code here ...
//!     }
//!
//!     fn task_100ms(&self) {
//!         // Called every 100 ms.
//!         // ... Put your code here ...
//!     }
//!
//!     fn task_1000ms(&self) {
//!         // Called every 1000 ms.
//!         // ... Put your code here ...
//!     }
//! }
//!
//! struct MyThing2 {
//!     // ...
//! }
//!
//! impl MyThing2 {
//!     fn new() -> Self {
//!         Self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl sched_main::Ops for MyThing2 {
//!     fn task_10ms(&self) {
//!         // Called every 10 ms.
//!         // ... Put your code here ...
//!     }
//!
//!     fn task_100ms(&self) {
//!         // Called every 100 ms.
//!         // ... Put your code here ...
//!     }
//! }
//!
//! fn main() {
//!     let thing1 = std::sync::Arc::new(MyThing1::new());
//!     let thing2 = std::sync::Arc::new(MyThing2::new());
//!
//!     sched_main::init([thing1, thing2]);
//! }
//! ```
//!
//! # Runtime stats
//!
//! The scheduler can capture and calculate runtime statistics
//! so that you can get an idea about how loaded the CPUs are
//! and how filled the time slices are.
//!
//! To enable measurement the crate feature `meas` has to be enabled in your
//! `Cargo.toml` and at runtime the measurement has to be switched on with a call
//! to `rt_enable(true)`.
//! Then the statistics can then be printed to stdout with a call to `rt_print()`.
//!
//! Example:
//!
//! ```
//! // Define the scheduler, its tasks and behavior.
//! timeslice::define_sched! {
//!     name: sched_main,
//!     num_objs: 1,
//!     tasks: {
//!         { name: task_10ms, period: 10 ms, cpu: 0, stack: 8 kiB },
//!         { name: task_1000ms, period: 1000 ms, cpu: 1, stack: 8 kiB },
//!     },
//! }
//!
//! struct MyThing1 { /* ... */ }
//!
//! impl sched_main::Ops for MyThing1 {
//!     fn task_10ms(&self) {
//!         // ...
//!     }
//!
//!     fn task_1000ms(&self) {
//!         // Print the scheduler statistics to stdout:
//!         sched_main::rt_print();
//!     }
//! }
//!
//! fn main() {
//!     let thing1 = std::sync::Arc::new(MyThing1 {});
//!
//!     sched_main::init([thing1]);
//!
//!     // Enable scheduler runtime measurement and statistics:
//!     sched_main::rt_enable(true);
//! }

//! ```

/// Do not access this module directly from other crates.
#[doc(hidden)]
pub mod hal;

/// Do not access this module directly from other crates.
#[doc(hidden)]
pub mod meas;

/// This module contains the main API macros.
mod define_macro;

/// Re-exported for define_sched macro.
#[doc(hidden)]
pub use pastey::paste;

// vim: ts=4 sw=4 expandtab
