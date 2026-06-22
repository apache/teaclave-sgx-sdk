// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! The first version of the prelude of The Rust Standard Library.
//!
//! See the [module-level documentation](super) for more.

// Re-exported core operators
#[doc(no_inline)]
pub use crate::marker::{Send, Sized, Sync, Unpin};
#[doc(no_inline)]
pub use crate::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[doc(no_inline)]
pub use crate::mem::drop;

// Re-exported types and traits
#[doc(no_inline)]
pub use crate::convert::{AsMut, AsRef, From, Into};
#[doc(no_inline)]
pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
#[doc(no_inline)]
pub use crate::iter::{Extend, IntoIterator, Iterator};
#[doc(no_inline)]
pub use crate::option::Option::{self, None, Some};
#[doc(no_inline)]
pub use crate::result::Result::{self, Err, Ok};

// Re-exported built-in macros
#[allow(deprecated)]
#[doc(no_inline)]
pub use core::prelude::v1::{
    assert, assert_eq, assert_ne, cfg, column, compile_error, concat, debug_assert,
    debug_assert_eq, debug_assert_ne, env, file, format_args, include, include_bytes,
    include_str, line, log_syntax, matches, module_path, option_env, stringify, todo,
    r#try, trace_macros, unimplemented, unreachable, write, writeln,
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
};

#[doc(no_inline)]
pub use core::prelude::v1::concat_bytes;

// Do not `doc(no_inline)` so that they become doc items on their own
// (no public module for them to be re-exported from).
pub use core::prelude::v1::{
    alloc_error_handler, derive, global_allocator, test, test_case,
};

// Do not `doc(no_inline)` either.
pub use core::prelude::v1::cfg_accessible;

pub use core::prelude::v1::cfg_eval;

pub use core::prelude::v1::type_ascribe;

// The file so far is equivalent to core/src/prelude/v1.rs. It is duplicated
// rather than glob imported because we want docs to show these re-exports as
// pointing to within `std`.
// Below are the items from the alloc crate.

#[doc(no_inline)]
pub use crate::borrow::ToOwned;
#[doc(no_inline)]
pub use crate::boxed::Box;
#[doc(no_inline)]
pub use crate::string::{String, ToString};
#[doc(no_inline)]
pub use crate::vec::Vec;

// Std-defined macros that belong in the prelude (matching std::prelude::v1).
// Without these, downstream crates built with the `std` feature cannot resolve
// `panic!` (e.g. via `debug_assert!`), `vec!`, `format!`, etc. unqualified.
#[doc(no_inline)]
pub use crate::{dbg, eprint, eprintln, format, print, println, thread_local};

// `vec` and `panic` would be ambiguous with the modules of the same name, so
// shadow those modules with private empty modules and glob-export only the
// macros (this mirrors std's own prelude mechanism).
mod ambiguous_macros_only {
    #[allow(hidden_glob_reexports)]
    mod vec {}
    #[allow(hidden_glob_reexports)]
    mod panic {}
    pub use crate::*;
}
#[doc(no_inline)]
pub use self::ambiguous_macros_only::{panic, vec};
