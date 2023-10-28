# Simple periodic time slice scheduler

A simple multi-core scheduler that provides a trait with periodically called methods and a scheduler to call these methods as specified.

# Cargo.toml

```toml
[dependencies]
timeslice = { version = "0.1", features = [ "hal-espidf", "meas" ] }
```

# Example code

A simple usage example can look like this:

```rust
// Here we define the scheduler, its tasks and behavior.
timeslice::define_timeslice_sched! {
    name: sched_main,
    num_objs: 1,
    tasks: {
        { name: task_10ms, period: 10 ms, cpu: 0, stack: 16 kiB },
        { name: task_50ms, period: 50 ms, cpu: 0, stack: 3 kiB },
        { name: task_100ms, period: 100 ms, cpu: 1, stack: 16 kiB },
    }
}

// This structure belongs to your application. It contains application state.
struct MyThing {
    // ...
}

impl MyThing {
    fn new() -> Self {
        Self {
            // ...
        }
    }
}

// Implement the scheduler's tasks for your application.
impl sched_main::Ops for Box<MyThing> {
    fn task_10ms(&self) {
        // Called every 10 ms.
        // ... Put your code here ...
    }

    fn task_50ms(&self) {
        // Called every 50 ms.
        // ... Put your code here ...
    }

    fn task_100ms(&self) {
        // Called every 100 ms.
        // ... Put your code here ...
    }
}

fn main() {
    // Initialize the application.
    use std::sync::Arc;
    let thing = Arc::new(Box::new(MyThing::new()));

    // Initialize the scheduler and register your application.
    let obj = Arc::clone(&thing);
    sched_main::init([obj]);

    // ...
}
```

See the documentation for more complex examples.

# Backend selection

One backend has to be selected via `feature` flags. The following backends are available:

- `hal-espidf`: Use `esp-idf-hal` and `esp-idf-svc` hal backend. Select this, if you use an ESP microcontroller.
- `hal-dummy`: Backend for testing only. It does nothing. You should never select it.

Only one of the hal backend `feature` flags can be selected.

# Features

- `meas`: If the `meas` feature is enabled, then functions for run time measurements will be enabled.
  If this feature flag is not given, then the run time measurement functions will be empty dummies.

# Internals

## ESP-IDF implementation details

On `hal-espidf` each task runs as a `std::thread` that is pinned to the specified CPU core. The threads wait for a trigger signal from a periodic ESP timer. On triggering, the trait methods are executed, if the time slice is due.

## Memory safety

This crate does not use `unsafe` code.

# License

Copyright 2023 Michael Büsch <m@bues.ch>

Licensed under the Apache License version 2.0 or the MIT license, at your option.

SPDX-License-Identifier: Apache-2.0 OR MIT
