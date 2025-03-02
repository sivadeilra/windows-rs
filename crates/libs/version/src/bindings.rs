#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_targets::link!("ntdll.dll" "system" fn RtlGetVersion(lpversioninformation : *mut OSVERSIONINFOW) -> NTSTATUS);
pub type NTSTATUS = i32;
#[repr(C)]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl Copy for OSVERSIONINFOEXW {}
impl Clone for OSVERSIONINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl Copy for OSVERSIONINFOW {}
impl Clone for OSVERSIONINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VER_NT_WORKSTATION: u32 = 1u32;
