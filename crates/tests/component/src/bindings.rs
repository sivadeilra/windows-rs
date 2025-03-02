#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Nested {
    windows_core::imp::define_interface!(
        IThing,
        IThing_Vtbl,
        0x5448be22_9873_5ae6_9106_f6e8455d2fdd
    );
    impl std::ops::Deref for IThing {
        type Target = windows_core::IInspectable;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute(self) }
        }
    }
    windows_core::imp::interface_hierarchy!(
        IThing,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IThing {
        pub fn Method(&self) -> windows_core::Result<()> {
            let this = self;
            unsafe {
                (windows_core::Interface::vtable(this).Method)(windows_core::Interface::as_raw(
                    this,
                ))
                .ok()
            }
        }
    }
    impl windows_core::RuntimeType for IThing {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    #[repr(C)]
    pub struct IThing_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Method: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    }
    pub trait IThing_Impl: Sized {
        fn Method(&self) -> windows_core::Result<()>;
    }
    impl windows_core::RuntimeName for IThing {
        const NAME: &'static str = "test_component.Nested.IThing";
    }
    impl IThing_Vtbl {
        pub const fn new<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IThing_Impl,
            const OFFSET: isize,
        >() -> IThing_Vtbl {
            unsafe extern "system" fn Method<
                Identity: windows_core::IUnknownImpl<Impl = Impl>,
                Impl: IThing_Impl,
                const OFFSET: isize,
            >(
                this: *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
                let this = (*this).get_impl();
                IThing_Impl::Method(this).into()
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IThing, OFFSET>(),
                Method: Method::<Identity, Impl, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IThing as windows_core::Interface>::IID
        }
    }
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x97540591_1323_59c0_9ae0_f510cae62e54);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Property:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetProperty:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut Flags) -> windows_core::HRESULT,
    pub Int32Array: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const i32,
        u32,
        *mut i32,
        *mut u32,
        *mut *mut i32,
        *mut u32,
        *mut *mut i32,
    ) -> windows_core::HRESULT,
    pub StringArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const std::mem::MaybeUninit<windows_core::HSTRING>,
        u32,
        *mut std::mem::MaybeUninit<windows_core::HSTRING>,
        *mut u32,
        *mut *mut std::mem::MaybeUninit<windows_core::HSTRING>,
        *mut u32,
        *mut *mut std::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Input: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Property(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Property)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetProperty(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetProperty)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Flags(&self) -> windows_core::Result<Flags> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Flags)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Int32Array(
        &self,
        a: &[i32],
        b: &mut [i32],
        c: &mut windows_core::Array<i32>,
    ) -> windows_core::Result<windows_core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Int32Array)(
                windows_core::Interface::as_raw(this),
                a.len().try_into().unwrap(),
                a.as_ptr(),
                b.len().try_into().unwrap(),
                b.as_mut_ptr(),
                c.set_abi_len(),
                c as *mut _ as _,
                windows_core::Array::<i32>::set_abi_len(std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn StringArray(
        &self,
        a: &[windows_core::HSTRING],
        b: &mut [windows_core::HSTRING],
        c: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).StringArray)(
                windows_core::Interface::as_raw(this),
                a.len().try_into().unwrap(),
                core::mem::transmute(a.as_ptr()),
                b.len().try_into().unwrap(),
                core::mem::transmute_copy(&b),
                c.set_abi_len(),
                c as *mut _ as _,
                windows_core::Array::<windows_core::HSTRING>::set_abi_len(std::mem::transmute(
                    &mut result__,
                )),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .map(|| result__.assume_init())
        }
    }
    pub fn Input<P0, P1, P2, P3>(&self, a: P0, b: P1, c: P2, d: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<Class>,
        P2: windows_core::Param<windows::Foundation::IStringable>,
        P3: windows_core::Param<Callback>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Input)(
                windows_core::Interface::as_raw(this),
                a.param().abi(),
                b.param().abi(),
                c.param().abi(),
                d.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = IClass_Vtbl;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "test_component.Class";
}
unsafe impl Send for Class {}
unsafe impl Sync for Class {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Flags(pub u32);
impl Flags {
    pub const Ok: Self = Self(0u32);
}
impl windows_core::TypeKind for Flags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Flags").field(&self.0).finish()
    }
}
impl Flags {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for Flags {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for Flags {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(test_component.Flags;u4)");
}
windows_core::imp::define_interface!(
    Callback,
    Callback_Vtbl,
    0xe39afc7e_93f1_5a1d_92ef_bd5f71c62cb8
);
impl Callback {
    pub fn new<F: FnMut(i32) -> windows_core::Result<i32> + Send + 'static>(invoke: F) -> Self {
        let com = CallbackBox::<F> {
            vtable: &CallbackBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke(&self, a: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                a,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
struct CallbackBox<F: FnMut(i32) -> windows_core::Result<i32> + Send + 'static> {
    vtable: *const Callback_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(i32) -> windows_core::Result<i32> + Send + 'static> CallbackBox<F> {
    const VTABLE: Callback_Vtbl = Callback_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <Callback as windows_core::Interface>::IID
            || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
            || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            core::ptr::null_mut()
        };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        a: i32,
        result__: *mut i32,
    ) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        match (this.invoke)(a) {
            Ok(ok__) => {
                core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
            Err(err) => err.into(),
        }
    }
}
impl windows_core::RuntimeType for Callback {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct Callback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IClass_Impl: Sized {
    fn Property(&self) -> windows_core::Result<i32>;
    fn SetProperty(&self, value: i32) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<Flags>;
    fn Int32Array(
        &self,
        a: &[i32],
        b: &mut [i32],
        c: &mut windows_core::Array<i32>,
    ) -> windows_core::Result<windows_core::Array<i32>>;
    fn StringArray(
        &self,
        a: &[windows_core::HSTRING],
        b: &mut [windows_core::HSTRING],
        c: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>>;
    fn Input(
        &self,
        a: Option<&windows_core::IInspectable>,
        b: Option<&Class>,
        c: Option<&windows::Foundation::IStringable>,
        d: Option<&Callback>,
    ) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClass {
    const NAME: &'static str = "test_component.IClass";
}
impl IClass_Vtbl {
    pub const fn new<
        Identity: windows_core::IUnknownImpl<Impl = Impl>,
        Impl: IClass_Impl,
        const OFFSET: isize,
    >() -> IClass_Vtbl {
        unsafe extern "system" fn Property<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IClass_Impl::Property(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IClass_Impl::SetProperty(this, value).into()
        }
        unsafe extern "system" fn Flags<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut Flags,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IClass_Impl::Flags(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Int32Array<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            a_array_size: u32,
            a: *const i32,
            b_array_size: u32,
            b: *mut i32,
            c_array_size: *mut u32,
            c: *mut *mut i32,
            result_size__: *mut u32,
            result__: *mut *mut i32,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IClass_Impl::Int32Array(
                this,
                core::slice::from_raw_parts(core::mem::transmute_copy(&a), a_array_size as usize),
                core::slice::from_raw_parts_mut(
                    core::mem::transmute_copy(&b),
                    b_array_size as usize,
                ),
                windows_core::ArrayProxy::from_raw_parts(
                    core::mem::transmute_copy(&c),
                    c_array_size,
                )
                .as_array(),
            ) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StringArray<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            a_array_size: u32,
            a: *const std::mem::MaybeUninit<windows_core::HSTRING>,
            b_array_size: u32,
            b: *mut std::mem::MaybeUninit<windows_core::HSTRING>,
            c_array_size: *mut u32,
            c: *mut *mut std::mem::MaybeUninit<windows_core::HSTRING>,
            result_size__: *mut u32,
            result__: *mut *mut std::mem::MaybeUninit<windows_core::HSTRING>,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IClass_Impl::StringArray(
                this,
                core::slice::from_raw_parts(core::mem::transmute_copy(&a), a_array_size as usize),
                core::slice::from_raw_parts_mut(
                    core::mem::transmute_copy(&b),
                    b_array_size as usize,
                ),
                windows_core::ArrayProxy::from_raw_parts(
                    core::mem::transmute_copy(&c),
                    c_array_size,
                )
                .as_array(),
            ) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    core::ptr::write(result__, ok_data__);
                    core::ptr::write(result_size__, ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Input<
            Identity: windows_core::IUnknownImpl<Impl = Impl>,
            Impl: IClass_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            a: *mut core::ffi::c_void,
            b: *mut core::ffi::c_void,
            c: *mut core::ffi::c_void,
            d: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IClass_Impl::Input(
                this,
                windows_core::from_raw_borrowed(&a),
                windows_core::from_raw_borrowed(&b),
                windows_core::from_raw_borrowed(&c),
                windows_core::from_raw_borrowed(&d),
            )
            .into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClass, OFFSET>(),
            Property: Property::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            Int32Array: Int32Array::<Identity, Impl, OFFSET>,
            StringArray: StringArray::<Identity, Impl, OFFSET>,
            Input: Input::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClass as windows_core::Interface>::IID
    }
}
