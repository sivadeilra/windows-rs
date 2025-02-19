#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::*;

#[interface]
unsafe trait IBase {}

#[interface]
unsafe trait IFoo {}

#[implement(IBase)]
struct Base {}

impl IBase_Impl for Base {}

#[implement(IFoo)]
struct Foo {
    #[base]
    base: Base,
}

impl IFoo_Impl for Foo {}

#[test]
fn make_base() -> Result<()> {
    let base = ComObject::new(Base {});

    Ok(())
}
