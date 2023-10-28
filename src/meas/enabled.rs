// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

#![allow(clippy::new_without_default)]

use std::{
    collections::HashMap,
    io::Write,
    sync::{
        atomic::{
            AtomicBool, AtomicI64, AtomicU32,
            Ordering::{Acquire, Relaxed, SeqCst},
        },
        RwLock,
    },
};

fn usfmt(us: u32) -> String {
    if us >= 1000 {
        format!("{} ms", us.div_ceil(1000))
    } else {
        format!("{} us", us)
    }
}

struct RtCpuData {
    cum: AtomicU32,
    min: AtomicU32,
    max: AtomicU32,
}

impl RtCpuData {
    const fn new() -> Self {
        Self {
            cum: AtomicU32::new(0),
            min: AtomicU32::new(u32::MAX),
            max: AtomicU32::new(0),
        }
    }

    fn reset(&self) {
        self.cum.store(0, Relaxed);
        self.min.store(u32::MAX, Relaxed);
        self.max.store(0, Relaxed);
    }

    #[inline]
    fn update(&self, rt_us: u32) {
        self.cum.fetch_add(rt_us, Relaxed);
    }

    #[inline]
    fn cum(&self) -> &AtomicU32 {
        &self.cum
    }

    #[inline]
    fn min(&self) -> &AtomicU32 {
        &self.min
    }

    #[inline]
    fn max(&self) -> &AtomicU32 {
        &self.max
    }
}

struct RtTaskData {
    count: AtomicU32,
    cum: AtomicU32,
    min: AtomicU32,
    max: AtomicU32,
}

impl RtTaskData {
    const fn new() -> Self {
        Self {
            count: AtomicU32::new(0),
            cum: AtomicU32::new(0),
            min: AtomicU32::new(u32::MAX),
            max: AtomicU32::new(0),
        }
    }

    fn reset(&self) {
        self.count.store(0, Relaxed);
        self.cum.store(0, Relaxed);
        self.min.store(u32::MAX, Relaxed);
        self.max.store(0, Relaxed);
    }

    #[inline]
    fn update(&self, rt_us: u32) {
        self.count.fetch_add(1, Relaxed);
        self.cum.fetch_add(rt_us, Relaxed);
        self.min.store(self.min.load(Relaxed).min(rt_us), Relaxed);
        self.max.store(self.max.load(Relaxed).max(rt_us), Relaxed);
    }

    #[inline]
    fn count(&self) -> &AtomicU32 {
        &self.count
    }

    #[inline]
    fn cum(&self) -> &AtomicU32 {
        &self.cum
    }

    #[inline]
    fn min(&self) -> &AtomicU32 {
        &self.min
    }

    #[inline]
    fn max(&self) -> &AtomicU32 {
        &self.max
    }
}

pub struct RuntimeMeas {
    initialized: AtomicBool,
    enabled: AtomicBool,
    print_stamp: AtomicI64,
    cpus: [RtCpuData; crate::hal::CORES],
    tasks: RwLock<HashMap<&'static str, RtTaskData>>,
}

impl RuntimeMeas {
    pub fn new() -> Self {
        #[allow(clippy::declare_interior_mutable_const)]
        const RTCPUDATA_INIT: RtCpuData = RtCpuData::new();
        Self {
            initialized: AtomicBool::new(false),
            enabled: AtomicBool::new(false),
            print_stamp: AtomicI64::new(0),
            cpus: [RTCPUDATA_INIT; crate::hal::CORES],
            tasks: RwLock::new(HashMap::new()),
        }
    }

    #[inline]
    pub fn meas_begin(&self) -> i64 {
        if self.is_enabled() {
            crate::hal::now_us()
        } else {
            -1
        }
    }

    #[inline]
    fn do_meas_end(&self, task: &RtTaskData, core: usize, rt: u32) {
        self.cpus[core].update(rt);
        task.update(rt);
    }

    #[inline]
    pub fn meas_end(&self, task_name: &'static str, core: usize, begin: i64) {
        if !self.is_enabled() || begin < 0 {
            return;
        }
        let rt = crate::hal::now_us().wrapping_sub(begin) as u32;
        if !(0..10_000_000).contains(&rt) {
            return;
        }
        let tasks = self.tasks.read().unwrap();
        if let Some(task) = tasks.get(task_name) {
            self.do_meas_end(task, core, rt);
        } else {
            drop(tasks);
            {
                let mut tasks = self.tasks.write().unwrap();
                tasks.insert(task_name, RtTaskData::new());
            }
            let tasks = self.tasks.read().unwrap();
            if let Some(task) = tasks.get(task_name) {
                self.do_meas_end(task, core, rt);
            }
        }
    }

    pub fn print_cpus(&self) {
        if self.is_enabled() {
            let now = crate::hal::now_us();
            let initialized = self.initialized.swap(true, Relaxed);
            let prev_time = if initialized {
                self.print_stamp.load(Relaxed)
            } else {
                self.print_stamp.store(now, Relaxed);
                now
            };
            let period = now.wrapping_sub(prev_time);
            if period >= 10_000_000 {
                self.print_stamp.store(now, Relaxed);
            } else if period >= 100_000 {
                self.print_stamp.store(now, Relaxed);
                let period = period as u32;
                let mut stdout = std::io::stdout().lock();
                let _ = writeln!(stdout);
                for cpu in 0..crate::hal::CORES {
                    let rt_cpu = &self.cpus[cpu];
                    let cur = rt_cpu.cum().swap(0, Relaxed);
                    let cur = ((cur * 100) + (period / 2)) / period;
                    let min = rt_cpu.min().load(Relaxed).min(cur);
                    let max = rt_cpu.max().load(Relaxed).max(cur);
                    rt_cpu.min().store(min, Relaxed);
                    rt_cpu.max().store(max, Relaxed);
                    if min != u32::MAX {
                        let _ = writeln!(
                            stdout,
                            "CPU {}: {} %; min {} %; max {} %",
                            cpu, cur, min, max
                        );
                    }
                }
            }
        }
    }

    pub fn print_task(&self, task_name: &'static str, time_base: usize, core: usize) {
        if self.is_enabled() {
            let mut stdout = std::io::stdout().lock();
            let tasks = self.tasks.read().unwrap();
            if let Some(rt_task) = tasks.get(task_name) {
                let count = rt_task.count().swap(0, Relaxed);
                let cum = rt_task.cum().swap(0, Relaxed);
                let min = rt_task.min().load(Relaxed);
                let max = rt_task.max().load(Relaxed);
                let avg = if count > 0 { cum / count } else { u32::MAX };
                if min != u32::MAX {
                    let _ = writeln!(
                        stdout,
                        "{}, {} ms @ CPU {}: {}; min {}; max {}",
                        task_name,
                        time_base,
                        core,
                        usfmt(avg),
                        usfmt(min),
                        usfmt(max)
                    );
                }
            }
        }
    }

    fn reset(&self) {
        for cpu in 0..crate::hal::CORES {
            self.cpus[cpu].reset();
        }
        for task in self.tasks.read().unwrap().values() {
            task.reset();
        }
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Relaxed)
    }

    pub fn enable(&self, en: bool) {
        if en {
            if !self.enabled.load(Acquire) {
                self.reset();
                self.initialized.store(false, Relaxed);
                self.enabled.store(true, SeqCst);
            }
        } else {
            self.enabled.store(false, SeqCst);
        }
    }
}

// vim: ts=4 sw=4 expandtab
