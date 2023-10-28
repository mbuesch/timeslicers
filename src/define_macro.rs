// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

#[macro_export]
macro_rules! define_timeslice_sched {
    (
        name: $name:ident,
        num_objs: $num_objs:literal,
        tasks: {
            $(
                {
                    name: $taskname:ident,
                    period: $timebase:literal ms,
                    cpu: $core:literal,
                    stack: $stack_kib:literal kiB
                }
            ),*
        }
    ) => {
        paste::paste! {
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
                    },
                    thread,
                    time::Duration,
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

                lazy_static::lazy_static! {
                    /// Time slice scheduler instance.
                    #[doc(hidden)]
                    static ref TIMESLICESCHED: TimeSliceSched = TimeSliceSched {
                        initialized: AtomicBool::new(false),
                        baseperiod: AtomicU32::new(0),
                        count: AtomicU32::new(0),
                        count_mod: AtomicU32::new(0),
                        $(
                            [<trigflag_ $taskname>]: Arc::new((Mutex::new(false),
                                                               Condvar::new())),
                        )*
                        rt: RuntimeMeas::new(),
                    };

                    /// Time slice scheduler instance.
                    #[doc(hidden)]
                    static ref TIMESLICESCHED_OS: Mutex<Option<$crate::hal::Timer<'static>>> = Mutex::new(None);
                }

                /// Time slice scheduler initialization.
                #[inline]
                pub fn init(objs: [OpsObject; $num_objs]) {
                    TimeSliceSched::init(objs);
                }

                /// Print the task and CPU runtime load.
                pub fn rt_print() {
                    TIMESLICESCHED.rt.print_cpus();
                    $(
                        TIMESLICESCHED.rt.print_task(
                            std::stringify!($taskname),
                            $timebase,
                            $core
                        );
                    )*
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
                            let stack: usize = ($stack_kib) * 1024;
                            $crate::hal::task_spawn(
                                std::concat!(std::stringify!($name), "_cpu", $core, "\0"),
                                core,
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

                                        TIMESLICESCHED.rt.meas_end(std::stringify!($taskname), $core, begin);
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
