//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_MODEL!()] // public, root
#![doc = crate::_DOC_MODEL_MODULES!()]
#![doc = crate::_doc!(flat:"model")]
#![doc = crate::_doc!(hr)]
//
// docs
crate::CONST! { pub(crate) _DOC_MODEL_MODULES =
    crate::_doc!(modules: crate; model);
}

crate::mods_in! {
}
crate::mods_out! { // _pub_mods, _crate_internals
    _pub_mods {
        // pub use super::{
        // };
    }
    _crate_internals {
        pub(crate) use super::_DOC_MODEL_MODULES;
    }
}
