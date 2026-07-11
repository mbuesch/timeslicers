// -*- coding: utf-8 -*-
//
// Copyright 2023-2024 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

/// Define a scheduler
#[macro_export]
macro_rules! define_sched {
    (
        name: $name:ident,
        num_objs: $num_objs:literal,
        tasks: {
            $(
                {
                    name: $taskname:ident,
                    period: $timebase:literal ms,
                    cpu: $core:literal,
                    prio: $prio:literal,
                    stack: $stack_kib:literal kiB
                }
            ),* $(,)?
        } $(,)?
    ) => {
        $crate::paste! {
            pub mod $name {
                use std::{
                    sync::{
                        atomic::{
                            AtomicBool,
                            AtomicU32,
                            Ordering::{
                                Relaxed,
                                SeqCst,
                            },
                            fence,
                        },
                        Arc,
                        Condvar,
                        Mutex,
                        LazyLock,
                    },
                    thread,
                    time::Duration,
                    ffi::CStr,
                };
                use $crate::meas::RuntimeMeas;

                /// Time slice scheduler tasks.
                pub trait Ops {
                    $(
                        /// Run the user code for this time base.
                        fn $taskname(&self) {
                        }
                    )*
                }

                /// Time slice scheduler handler trait object.
                pub type OpsObject = Arc<dyn Ops + Send + Sync + 'static>;

                /// Time slice scheduler.
                #[doc(hidden)]
                pub struct TimeSliceSched {
                    initialized: AtomicBool,
                    baseperiod: AtomicU32,
                    count: AtomicU32,
                    count_mod: AtomicU32,
                    $(
                        [<trigflag_ $taskname>]: Arc<(Mutex<bool>, Condvar)>,
                    )*
                    rt: RuntimeMeas,
                }

                /// Time slice scheduler instance.
                #[doc(hidden)]
                static TIMESLICESCHED: LazyLock<TimeSliceSched> = LazyLock::new(|| {
                    TimeSliceSched {
                        initialized: AtomicBool::new(false),
                        baseperiod: AtomicU32::new(0),
                        count: AtomicU32::new(0),
                        count_mod: AtomicU32::new(0),
                        $(
                            [<trigflag_ $taskname>]: Arc::new((Mutex::new(false),
                                                               Condvar::new())),
                        )*
                        rt: RuntimeMeas::new(),
                    }
                });

                /// Time slice scheduler instance.
                #[doc(hidden)]
                static TIMESLICESCHED_OS: LazyLock<Mutex<Option<$crate::hal::Timer<'static>>>>
                    = LazyLock::new(|| Mutex::new(None));

                /// Time slice scheduler initialization.
                #[inline]
                pub fn init(objs: [OpsObject; $num_objs]) {
                    TimeSliceSched::init(objs);
                }

                /// Print the task and CPU runtime load.
                pub fn rt_print() {
                    if TIMESLICESCHED.rt.is_enabled() {
                        TIMESLICESCHED.rt.print_cpus();
                        $(
                            TIMESLICESCHED.rt.print_task(
                                core::stringify!($taskname),
                                $timebase,
                                $core
                            );
                        )*
                    }
                }

                #[inline]
                pub fn rt_is_enabled() -> bool {
                    TIMESLICESCHED.rt.is_enabled()
                }

                #[inline]
                pub fn rt_enable(enable: bool) {
                    TIMESLICESCHED.rt.enable(enable);
                }

                impl TimeSliceSched {
                    /// Initialize the time slice scheduler, once.
                    fn init(objs: [OpsObject; $num_objs]) {
                        assert!(!TIMESLICESCHED.initialized.swap(true, Relaxed));
                        let objs = Arc::new(objs);

                        // Calculate base period and counter modulo.
                        let mut min_timebase = u32::MAX;
                        let mut max_timebase = u32::MIN;
                        $(
                            min_timebase = min_timebase.min($timebase);
                            max_timebase = max_timebase.max($timebase);
                        )*
                        let baseperiod = min_timebase;
                        let count_mod = max_timebase / baseperiod;

                        // Spawn all handler threads.
                        $(
                            // Clone shared variable refs.
                            let [<thread_trigflag_ $taskname>] = Arc::clone(&TIMESLICESCHED.[<trigflag_ $taskname>]);
                            let [<thread_objs_ $taskname>] = Arc::clone(&objs);

                            let core: usize = $core;
                            let prio: u8 = $prio;
                            assert!(prio < 10, "prio must be a number in the range 0..=9");
                            let stack: usize = ($stack_kib) * 1024;
                            let name: &'static str = core::concat!(core::stringify!($name), "_cpu", $core, "\0");
                            let name_cstr = CStr::from_bytes_with_nul(name.as_bytes()).unwrap();
                            $crate::hal::task_spawn(
                                name_cstr,
                                core,
                                prio,
                                stack,
                                move || {
                                    assert_eq!($crate::hal::current_core(), core);
                                    let (flag_mutex, trig_condvar) = &*[<thread_trigflag_ $taskname>];

                                    loop {
                                        // Wait for the thread flag to be set.
                                        {
                                            let mut flag = flag_mutex.lock().unwrap();
                                            while !*flag {
                                                flag = trig_condvar.wait(flag).unwrap();
                                            }
                                            *flag = false;
                                        }

                                        let begin = TIMESLICESCHED.rt.meas_begin();

                                        // Execute all handlers for this task.
                                        for obj in &*[<thread_objs_ $taskname>] {
                                            obj.$taskname();
                                        }

                                        TIMESLICESCHED.rt.meas_end(core::stringify!($taskname), $core, begin);
                                    }
                                }
                            );
                        )*

                        TIMESLICESCHED.baseperiod.store(baseperiod, Relaxed);
                        TIMESLICESCHED.count.store(0, Relaxed);
                        TIMESLICESCHED.count_mod.store(count_mod, Relaxed);
                        fence(SeqCst);

                        *TIMESLICESCHED_OS.lock().unwrap() = Some($crate::hal::Timer::new(
                            || TIMESLICESCHED.base_tick_handler(),
                            Duration::from_millis(baseperiod as u64)
                        ));
                    }

                    /// Base timer tick handler.
                    fn base_tick_handler(&self) {
                        let baseperiod = self.baseperiod.load(Relaxed);
                        let count = self.count.load(Relaxed);
                        $(
                            if count % ($timebase / baseperiod) == 0 {
                                let ([<flag_ $taskname>], [<trig_ $taskname>]) = &*self.[<trigflag_ $taskname>];
                                *[<flag_ $taskname>].lock().unwrap() = true;
                                [<trig_ $taskname>].notify_one();
                            }
                        )*
                        let count_mod = self.count_mod.load(Relaxed);
                        let count = (count + 1) % count_mod;
                        self.count.store(count, Relaxed);
                    }
                }
            }
        }
    }
}

// vim: ts=4 sw=4 expandtab
