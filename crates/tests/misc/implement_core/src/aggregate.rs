//! Unit tests for `windows_core::ComObject`

#![allow(unused_imports)]

use core::ffi::c_void;
use std::borrow::Borrow;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::Arc;
use windows_core::{
    implement, interface, ComObject, IUnknown, IUnknownImpl, IUnknown_Vtbl, Interface, InterfaceRef,
};

#[interface("818f2fd1-d479-4398-b286-a93c4c7904d1")]
unsafe trait IFoo: IUnknown {
    fn get_x(&self) -> u32;
}

#[interface("687eb4b2-6df6-41a3-86c7-4b04b94ad2d8")]
unsafe trait IBar: IUnknown {
    fn say_hello(&self);
}

#[implement(IFoo)]
struct Base {
    n: u32,
}

impl IFoo_Impl for Base_Impl {
    unsafe fn get_x(&self) -> u32 {
        self.n
    }
}

#[implement(IBar)]
struct Derived {
    #[base]
    base: Base_Impl,

    q: String,
}

impl IBar_Impl for Derived_Impl {
    unsafe fn say_hello(&self) {
        println!("Derived::say_hello: Hello!");
        println!("base.n = {}", self.base.n);
        println!("q = {:?}", self.q);
    }
}

#[test]
fn make_base() {
    let base = ComObject::new(Base { n: 100 });
    let base_foo: IFoo = base.to_interface();

    assert_eq!(unsafe { base_foo.get_x() }, 100);
}

#[test]
fn make_derived() {
    let derived = ComObject::new_aggregated(
        Derived {
            q: "hello!".to_string(),
        },
        Base { n: 50 }.into_outer(),
    );
    // let derived_ifoo: IFoo = derived.to_interface();
    let derived_ibar: IBar = derived.to_interface();
    unsafe {
        derived_ibar.say_hello();
    }
}
