extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, DebugStructGenerator, Fallbacks, Profile, Registry, StructGenerator};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();
   
    let registry = Registry::new(Api::Gl, version(), profile(), fallbacks(), []);

    if cfg!(feature = "debug") { 
        registry.write_bindings(DebugStructGenerator, &mut file_gl).unwrap();
    } else { 
        registry.write_bindings(StructGenerator, &mut file_gl).unwrap();
    }
}

fn version() -> (u8, u8) {
    if      cfg!(feature = "gl10") { (1,0) } 
    else if cfg!(feature = "gl11") { (1,1) }
    else if cfg!(feature = "gl12") { (1,2) }
    else if cfg!(feature = "gl13") { (1,3) }
    else if cfg!(feature = "gl14") { (1,4) }
    else if cfg!(feature = "gl15") { (1,5) }
    else if cfg!(feature = "gl20") { (2,0) }
    else if cfg!(feature = "gl21") { (2,1) }
    else if cfg!(feature = "gl30") { (3,0) }
    else if cfg!(feature = "gl31") { (3,1) }
    else if cfg!(feature = "gl32") { (3,2) }
    else if cfg!(feature = "gl33") { (3,3) }
    else if cfg!(feature = "gl40") { (4,0) }
    else if cfg!(feature = "gl41") { (4,1) }
    else if cfg!(feature = "gl42") { (4,2) }
    else if cfg!(feature = "gl43") { (4,3) }
    else if cfg!(feature = "gl44") { (4,4) }
    else if cfg!(feature = "gl45") { (4,5) }
    else if cfg!(feature = "gl46") { (4,6) }
    else { (4,6) }
}

fn profile() -> Profile {
    if cfg!(feature = "compatibility") { 
        Profile::Compatibility 
    } else { 
        Profile::Core 
    }
}

fn fallbacks() -> Fallbacks {
    if cfg!(feature = "fallbacks") { 
        Fallbacks::All
    } else { 
        Fallbacks::None
    }
}
