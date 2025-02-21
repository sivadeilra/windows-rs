#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use windows::core::*;
use windows::Foundation::*;

#[interface("3fbaf313-5764-4a60-8766-e78aa064f12d")]
unsafe trait IBase: IUnknown {}

#[implement(IBase)]
struct Base {}

impl IBase_Impl for Base_Impl {}


#[interface("e997896c-111a-4baa-b53a-bf155fb1a1ef")]
unsafe trait IDerived: IUnknown {}

#[implement(IDerived)]
struct Derived {
    #[base]
    base: Base_Impl,

    #[allow(dead_code)]
    more_stuff: u32,
}

impl IDerived_Impl for Derived_Impl {}

#[test]
fn make_base() -> Result<()> {
    let _base = ComObject::new(Base {});

    Ok(())
}

#[test]
fn make_derived() -> Result<()> {
    let base = ComObject::new(Base {});
    let derived = ComObject::new_aggregated(Derived { more_stuff: 42 }, base);
    drop(derived);
    Ok(())
}