mod call;
mod cut;
mod exit;
mod ifc;
pub use call::{fs_call, unfocused_call};
pub use cut::{fs_cut, unfocused_cut};
<<<<<<< HEAD
pub use exit::{exit, fs_exit};
=======
pub use exit::exit;
>>>>>>> 47eb428 (updated ifc macros)
pub use ifc::{
    fs_ife, fs_ifg, fs_ifge, fs_ifl, fs_ifle, fs_ifne, unfocused_ife, unfocused_ifg,
    unfocused_ifge, unfocused_ifl, unfocused_ifle, unfocused_ifne,
};
<<<<<<< HEAD
=======
pub use op::{fs_op, unfocused_op};
>>>>>>> 47eb428 (updated ifc macros)
