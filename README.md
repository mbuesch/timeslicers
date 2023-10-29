# Simple periodic time slice scheduler

A simple multi-core scheduler that provides a trait to the application.
This trait, if implemented for an application specific object, can be used to get periodic calls from the scheduler.
The application trait object has to be registered to the scheduler to get the periodic calls.

Task methods of the scheduler trait are optional to implement, if one or more methods is not needed for a particular application object.

## Restrictions

To keep things simple, the scheduler has a couple of restrictions:

- All task periods must be multiples of the smallest task period.
- All tasks run with the same OS priority. Therefore, the tasks won't interrupt each other.
- The order of execution of the tasks us undefined.
- The number of application objects that can be registered to the scheduler is compile time constant.

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

## esp-idf-hal and esp-idf-svc versions

The `hal-espidf` backend depends on the following crates:

```toml
esp-idf-hal = "0.42"
esp-idf-svc = "0.47"
```

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
