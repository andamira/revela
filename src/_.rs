//
//! …
//

// environment
#![cfg_attr(not(feature = "std"), no_std)]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
// nightly
#![cfg_attr(nightly_doc, doc(test(attr(feature(doc_cfg)))))] // enable for all doctests
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
//

/* imports */

extern crate self as revela;

use ::devela::__hidden::*;
use ::devela::all::*;

crate::mods_in! {
    pub mod_ engine;
    pub mod_ model;
    pub mod_ org;
    pub mod_ phys;
    pub mod_ vita;

    // internal
    pub mod_ yard; // Scaffolding, taxonomy, and documentation support.
}

#[rustfmt::skip]
#[doc = crate::_DOC_ALL_!()]
#[doc = crate::_doc!(br+hr)] // gives way to the first root module
#[doc = crate::_DOC_ALL_PLUS_!()]
pub mod all_ {
    macro_rules! _COMMON_DOC { ($mod:literal) => { concat!(" ",
        crate::_doc!(root:$mod), "\n\nAll `", $mod, "` module's items flat re-exported.") }; }


    #[doc = concat![crate::_DOC_ENGINE!(), crate::_DOC_ENGINE_MODULES!(), _COMMON_DOC!("engine")]]
    // #[cfg(any(not(feature = "__select_modules"), feature = "engine"))]
    // #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("engine")))))]
    pub mod _engine { #[allow(unused)] pub use super::super::engine::_all::*; }

    #[doc = concat![crate::_DOC_MODEL!(), crate::_DOC_MODEL_MODULES!(), _COMMON_DOC!("model")]]
    pub mod _model { #[allow(unused)] pub use super::super::model::_all::*; }

    #[doc = concat![crate::_DOC_ORG!(), crate::_DOC_ORG_MODULES!(), _COMMON_DOC!("org")]]
    pub mod _org { #[allow(unused)] pub use super::super::org::_all::*; }

    #[doc = concat![crate::_DOC_PHYS!(), crate::_DOC_PHYS_MODULES!(), _COMMON_DOC!("phys")]]
    pub mod _phys { #[allow(unused)] pub use super::super::phys::_all::*; }

    #[doc = concat![crate::_DOC_VITA!(), crate::_DOC_VITA_MODULES!(), _COMMON_DOC!("vita")]]
    pub mod _vita { #[allow(unused)] pub use super::super::vita::_all::*; }
}
#[doc = crate::_DOC_ALL!()]
pub mod all {
    #[allow(unused_imports)]
    pub use crate::_all::*;
}

// #[doc(hidden)]
// pub use _devela as devela;
// #[doc(hidden)]
// /// Integrated devela vocabulary available through this crate.
// ///
// /// Combines this crate's public items
// /// with the corresponding `devela` vocabulary in one namespace.
// pub mod _devela {
//     #[allow(unused_imports)]
//     pub use crate::all::*;
//     pub use ::devela::all::*;
// }

crate::mods_out! { // _pub_mods, _reexports, _crate_internals
    _pub_mods {
        pub use super::engine::_all::*;
        pub use super::model::_all::*;
        pub use super::org::_all::*;
        pub use super::phys::_all::*;
        pub use super::vita::_all::*;
    }
    _reexports {
    }
    _crate_internals {
        pub use super::engine::_crate_internals::*;
        pub use super::model::_crate_internals::*;
        pub use super::org::_crate_internals::*;
        pub use super::phys::_crate_internals::*;
        pub use super::vita::_crate_internals::*;
        pub use super::yard::_crate_internals::*;
    }
}
