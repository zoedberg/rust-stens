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

#[macro_export]
macro_rules! tn {
    ($name:literal) => {
        $crate::Ident::from($name)
    };
}

#[macro_export]
macro_rules! fields {
    { $($value:expr),+ $(,)? } => {
        {
            let mut c = 0u8;
            let mut m = ::std::collections::BTreeMap::new();
            $(
                assert!(m.insert($crate::ast::Field::unnamed(c), $value.into()).is_none(), "repeated field");
                #[allow(unused_assignments)] {
                    c += 1;
                }
            )+
            amplify::confinement::Confined::try_from(m).expect("too many fields").into()
        }
    };
    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut c = 0u8;
            let mut m = ::std::collections::BTreeMap::new();
            $(
                assert!(m.insert($crate::ast::Field::named(tn!($key), c), $value.into()).is_none(), "repeated field");
                #[allow(unused_assignments)] {
                    c += 1;
                }
            )+
            amplify::confinement::Confined::try_from(m).expect("too many fields").into()
        }
    }
}

#[macro_export]
macro_rules! variants {
    { $($key:expr => $ord:literal => $value:expr),+ $(,)? } => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                assert!(m.insert($crate::ast::Field::named(tn!($key), $value), $ord).is_none(), "repeated enum variant");
            )+
            amplify::confinement::Confined::try_from(m).expect("too many enum variants").into()
        }
    };
    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut c = 0u8;
            let mut m = ::std::collections::BTreeMap::new();
            $(
                assert!(m.insert($crate::ast::Field::named(tn!($key), $value), c).is_none(), "repeated enum variant");
                #[allow(unused_assignments)] {
                    c += 1;
                }
            )+
            amplify::confinement::Confined::try_from(m).expect("too many enum variants").into()
        }
    }
}
