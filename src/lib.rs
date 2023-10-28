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
//! timeslice::define_timeslice_sched! {
//!     name: sched_main,
//!     num_objs: 2,
//!     tasks: {
//!         { name: task_10ms, period: 10 ms, cpu: 0, stack: 16 kiB },
//!         { name: task_50ms, period: 50 ms, cpu: 0, stack: 3 kiB },
//!         { name: task_100ms, period: 100 ms, cpu: 1, stack: 16 kiB },
//!         { name: task_1000ms, period: 1000 ms, cpu: 1, stack: 3 kiB }
//!     }
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
//! impl sched_main::Ops for Box<MyThing1> {
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
//! impl sched_main::Ops for Box<MyThing2> {
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
//!     use std::sync::Arc;
//!     let thing1 = Arc::new(Box::new(MyThing1::new()));
//!     let thing2 = Arc::new(Box::new(MyThing2::new()));
//!
//!     let obj1 = Arc::clone(&thing1);
//!     let obj2 = Arc::clone(&thing2);
//!     sched_main::init([obj1, obj2]);
//!
//!     // ...
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

// vim: ts=4 sw=4 expandtab
