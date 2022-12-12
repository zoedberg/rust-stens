// Strict encoding schema library, implementing validation and parsing of strict
// encoded data against the schema.
//
// Written in 2022-2023 by
//     Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2022-2023 by Ubideco Project.
//
// You should have received a copy of the Apache 2.0 License along with this
// software. If not, see <https://opensource.org/licenses/Apache-2.0>.

//! DTL stands for "Data type library".

mod gravel;
mod monolith;
mod translate;
mod serialize;

pub use gravel::{Gravel, GravelAlias, GravelId, GravelName, GravelTy};
pub use monolith::{Monolith, MonolithTy};

pub type TypeIndex = std::collections::BTreeMap<crate::TyId, crate::TypeName>;
