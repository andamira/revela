//
#![doc = crate::_DOC_PHYS!()] // public
#![doc = crate::_DOC_PHYS_MODULES!()]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//
// docs
crate::CONST! { pub(crate) _DOC_PHYS_MODULES =
    crate::_doc!(modules: crate; phys);
}

crate::mods_in! {
    // pub mod_ astro; // Astronomy-related abstractions
    // pub mod_ bio; // Biology-related abstractions
    // pub mod_ chem; // Chemistry-related abstractions
    // pub mod_ elec; // Electrical state, fields, transport, and energetic coupling
    // pub mod_ geo; // Geophysics-related abstractions
    // pub mod_ heat; // Thermodynamics and heat transfer
    // pub mod_ mech; // Mechanical state, response, interaction, and transmission
    // pub mod_ optic; // Optics and light transport in piecewise media
    // pub mod_ subs; // Physical substrates, bodies, phases, and material response properties
    // pub mod_ time; // Time and calendar types and operations
}
crate::mods_out! { // _pub_mods
    _pub_mods {
        // pub use super::{
        //     // astro::_all::*,
        //     // bio::_all::*,
        //     // chem::_all::*,
        //     // elec::_all::*,
        //     // geo::_all::*,
        //     // heat::_all::*,
        //     // mech::_all::*,
        //     // optic::_all::*,
        //     // subs::_all::*,
        //     // time::_all::*,
        // };
    }
    _crate_internals {
        pub(crate) use super::_DOC_PHYS_MODULES;
    }
}
