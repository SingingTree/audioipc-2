// Copyright © 2017 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

extern crate audioipc;
extern crate cubeb_core;
#[macro_use]
extern crate cubeb_backend;
mod context;
mod stream;

use cubeb_backend::capi;
use cubeb_core::ffi;
use context::ClientContext;
use std::os::raw::{c_char, c_int};
use stream::ClientStream;

#[no_mangle]
/// Entry point from C code.
pub unsafe extern "C" fn cubeb_remote_init(c: *mut *mut ffi::cubeb, context_name: *const c_char) -> c_int {
    capi::capi_init::<ClientContext>(c, context_name)
}