#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> std::ops::Deref for IIterable<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"TODO")
            .push_slice(b";")
            .push_other(T::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct IIterator<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> std::ops::Deref for IIterator<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterator<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterator<T>
{
}
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn get_Current(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).get_HasCurrent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"TODO")
            .push_slice(b";")
            .push_other(T::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub get_HasCurrent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct IKeyValuePair<K, V>(
    windows_core::IUnknown,
    core::marker::PhantomData<K>,
    core::marker::PhantomData<V>,
)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> std::ops::Deref
    for IKeyValuePair<K, V>
{
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IKeyValuePair<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IKeyValuePair<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IKeyValuePair<K, V>
{
    pub fn get_Key(&self) -> windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Key)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_Value(&self) -> windows_core::Result<V> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Value)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for IKeyValuePair<K, V>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"TODO")
            .push_slice(b";")
            .push_other(K::SIGNATURE)
            .push_slice(b";")
            .push_other(V::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::Interface for IKeyValuePair<K, V>
{
    type Vtable = IKeyValuePair_Vtbl<K, V>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IKeyValuePair_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Key: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<K>,
    ) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<V>,
    ) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct IMapView<K, V>(
    windows_core::IUnknown,
    core::marker::PhantomData<K>,
    core::marker::PhantomData<V>,
)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> std::ops::Deref
    for IMapView<K, V>
{
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IMapView<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IMapView<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMapView<K, V>
{
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IMapView<K, V>
{
    pub fn Lookup<P0>(
        &self,
        key: &mut <K as windows_core::Type<K>>::Default,
    ) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(
                windows_core::Interface::as_raw(this),
                key as *mut _ as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn get_Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).get_Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HasKey<P0>(
        &self,
        key: &mut <K as windows_core::Type<K>>::Default,
    ) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(
                windows_core::Interface::as_raw(this),
                key as *mut _ as _,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for IMapView<K, V>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"TODO")
            .push_slice(b";")
            .push_other(K::SIGNATURE)
            .push_slice(b";")
            .push_other(V::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::Interface for IMapView<K, V>
{
    type Vtable = IMapView_Vtbl<K, V>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IMapView_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<K>,
        *mut windows_core::AbiType<V>,
    ) -> windows_core::HRESULT,
    pub get_Size:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<K>,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
