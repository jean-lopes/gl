use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

pub use bindings::*;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gl>,
}

impl fmt::Debug for Gl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GL")
    }
}

impl Deref for Gl {
    type Target = bindings::Gl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn load_with<F>(loadfn: F) -> Gl
    where F: FnMut(&'static str) -> *const types::GLvoid {
    Gl {
        inner: Rc::new(bindings::Gl::load_with(loadfn))
    }
}
