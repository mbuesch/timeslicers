// -*- coding: utf-8 -*-
//
// Copyright 2023 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

use std::marker::PhantomData;
use std::time::Duration;

pub struct Timer<'a> {
    _x: PhantomData<&'a ()>,
}

impl<'a> Timer<'a> {
    pub fn new<F>(_callback: F, _period: Duration) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        Self { _x: PhantomData }
    }
}

// vim: ts=4 sw=4 expandtab
