// Strict encoding schema library, implementing validation and parsing
// strict encoded data against a schema.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2022-2023 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2023 UBIDECO Institute
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    //dead_code,
    //missing_docs
)]
#![allow(unused_braces)] // Due to rust compiler bug not understanding proc macro expressions
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
extern crate amplify;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;
#[macro_use]
pub extern crate strict_encoding as encoding;
extern crate core;

pub use encoding::derive;
pub use strict_encoding::{
    fname, libname, strict_dumb, tn, vname, StrictDecode, StrictDumb, StrictEncode, StrictType,
};

#[macro_use]
mod macros;
mod util;
pub mod ast;
pub mod typelib;
pub mod typesys;
//pub mod value;

pub use ast::{Cls, KeyTy, SemId, Ty, TypeRef};
pub use typelib::{Dependency, LibAlias, LibRef, TypeLib, TypeLibId};
pub use typesys::TypeSystem;
pub use util::{BuildFragment, PreFragment, SemVer, Urn};
