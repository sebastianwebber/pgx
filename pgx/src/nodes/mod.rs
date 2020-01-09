#[cfg(feature = "pg10")]
mod pg10;

#[cfg(feature = "pg11")]
mod pg11;

#[cfg(feature = "pg12")]
mod pg12;

#[cfg(feature = "pg10")]
pub use pg10::*;

#[cfg(feature = "pg11")]
pub use pg11::*;

#[cfg(feature = "pg12")]
pub use pg12::*;

use crate::{pg_sys, DatumCompatible, PgBox};

/// #define IsA(nodeptr,_type_)		(nodeTag(nodeptr) == T_##_type_)
#[allow(clippy::not_unsafe_ptr_arg_deref)] // ok b/c we check that nodeptr isn't null
#[inline]
pub fn is_a(nodeptr: *mut pg_sys::Node, tag: pg_sys::NodeTag) -> bool {
    !nodeptr.is_null() && (unsafe { *nodeptr }).type_ == tag
}

impl PgNode {
    pub fn is<T>(self, boxed: PgBox<T>) -> bool
    where
        T: DatumCompatible<T>,
    {
        let node = boxed.to_pg() as *mut pg_sys::Node;
        let me = self as u32;
        !node.is_null() && unsafe { node.as_ref() }.unwrap().type_ == me
    }
}