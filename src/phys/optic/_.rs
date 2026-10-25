//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_PHYS_OPTIC!()] // public
#![doc = crate::_doc!(modules: crate::phys; optic)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(hr)]
//

// TODO
crate::mods_in! {
    // matter-light relation
    mod material; // Medium, Response, Transition
    // medium → bulk material
    // response → physical law at transition
    // transition → locus of change

    // spectral & energy description
    mod radiation;
    // spectrum
    // radiometry (later)

    // geometry of light transport
    mod transport; // Ray, Beam, Direction
    // ray
    // path
    // beam (later)
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        // };
    }
}
