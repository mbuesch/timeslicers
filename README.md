# Periodic time slice scheduler - For embedded systems

[Github repository](https://github.com/mbuesch/timeslicers)

A simple multi-core fixed interval task scheduler.

In embedded applications it is often needed to do things in fixed intervals.
This scheduler provides a flexible interface to define fixed interval tasks.

A macro defines at compile time:

- The task name
- The task period/interval
- The CPU core statically assigned to the task
- The task priority
- The stack size

The macro generates a trait, which must be implemented for one or more application objects.
This trait defines the functions being called by the scheduler at the specified intervals.

Task trait methods are optional to implement.
The default implementation is to do nothing.

## Behavior and restrictions

To keep things simple, the scheduler has a couple of restrictions:

- All task periods must be multiples of the smallest task period
- The task priorities must be in the range `0..=9`
- The number of application objects that can be registered to the scheduler is compile time constant

Scheduling behavior:

- The tasks are triggered in the order they are defined in the macro.
  If multiple tasks are triggered at the same time, the ones with the higher priority will be executed first.
- The actual execution order of tasks with the same priority triggered at the same time is not defined.

## Supported platforms

- esp-idf-hal: ESP32 with IDF.

# Cargo.toml

```toml
[dependencies]
timeslice = { version = "0.7", features = [ "hal-espidf", "meas" ] }
```

# Example code

A simple usage example can look like this:

```rust
// Here we define the scheduler, its tasks and behavior.
timeslice::define_sched! {
    name: sched_main,
    num_objs: 1,
    tasks: {
        { name: task_10ms,  period: 10 ms,  cpu: 0, prio: 9, stack: 16 kiB },
        { name: task_50ms,  period: 50 ms,  cpu: 0, prio: 8, stack: 3 kiB },
        { name: task_100ms, period: 100 ms, cpu: 1, prio: 7, stack: 16 kiB },
    },
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
impl sched_main::Ops for MyThing {
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
    let thing = std::sync::Arc::new(MyThing::new());

    // Initialize the scheduler and register your application.
    sched_main::init([thing]);
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
esp-idf-hal = "0.46"
esp-idf-svc = "0.52"
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

Copyright 2023-2026 Michael Büsch <m@bues.ch>

Licensed under the Apache License version 2.0 or the MIT license, at your option.

SPDX-License-Identifier: Apache-2.0 OR MIT
