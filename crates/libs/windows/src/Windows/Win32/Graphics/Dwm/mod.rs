#[inline]
pub unsafe fn DwmAttachMilContent<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmAttachMilContent(hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DwmAttachMilContent(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DwmDefWindowProc<P0, P1, P2>(hwnd: P0, msg: u32, wparam: P1, lparam: P2, plresult: *mut super::super::Foundation::LRESULT) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmDefWindowProc(hwnd : super::super::Foundation:: HWND, msg : u32, wparam : super::super::Foundation:: WPARAM, lparam : super::super::Foundation:: LPARAM, plresult : *mut super::super::Foundation:: LRESULT) -> super::super::Foundation:: BOOL);
    DwmDefWindowProc(hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi(), plresult)
}
#[inline]
pub unsafe fn DwmDetachMilContent<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmDetachMilContent(hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DwmDetachMilContent(hwnd.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DwmEnableBlurBehindWindow<P0>(hwnd: P0, pblurbehind: *const DWM_BLURBEHIND) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmEnableBlurBehindWindow(hwnd : super::super::Foundation:: HWND, pblurbehind : *const DWM_BLURBEHIND) -> windows_core::HRESULT);
    DwmEnableBlurBehindWindow(hwnd.param().abi(), pblurbehind).ok()
}
#[inline]
pub unsafe fn DwmEnableComposition(ucompositionaction: u32) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmEnableComposition(ucompositionaction : u32) -> windows_core::HRESULT);
    DwmEnableComposition(ucompositionaction).ok()
}
#[inline]
pub unsafe fn DwmEnableMMCSS<P0>(fenablemmcss: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmEnableMMCSS(fenablemmcss : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DwmEnableMMCSS(fenablemmcss.param().abi()).ok()
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn DwmExtendFrameIntoClientArea<P0>(hwnd: P0, pmarinset: *const super::super::UI::Controls::MARGINS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmExtendFrameIntoClientArea(hwnd : super::super::Foundation:: HWND, pmarinset : *const super::super::UI::Controls:: MARGINS) -> windows_core::HRESULT);
    DwmExtendFrameIntoClientArea(hwnd.param().abi(), pmarinset).ok()
}
#[inline]
pub unsafe fn DwmFlush() -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmFlush() -> windows_core::HRESULT);
    DwmFlush().ok()
}
#[inline]
pub unsafe fn DwmGetColorizationColor(pcrcolorization: *mut u32, pfopaqueblend: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetColorizationColor(pcrcolorization : *mut u32, pfopaqueblend : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DwmGetColorizationColor(pcrcolorization, pfopaqueblend).ok()
}
#[inline]
pub unsafe fn DwmGetCompositionTimingInfo<P0>(hwnd: P0, ptiminginfo: *mut DWM_TIMING_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetCompositionTimingInfo(hwnd : super::super::Foundation:: HWND, ptiminginfo : *mut DWM_TIMING_INFO) -> windows_core::HRESULT);
    DwmGetCompositionTimingInfo(hwnd.param().abi(), ptiminginfo).ok()
}
#[inline]
pub unsafe fn DwmGetGraphicsStreamClient(uindex: u32) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamClient(uindex : u32, pclientuuid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = std::mem::zeroed();
    DwmGetGraphicsStreamClient(uindex, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DwmGetGraphicsStreamTransformHint(uindex: u32, ptransform: *mut MilMatrix3x2D) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetGraphicsStreamTransformHint(uindex : u32, ptransform : *mut MilMatrix3x2D) -> windows_core::HRESULT);
    DwmGetGraphicsStreamTransformHint(uindex, ptransform).ok()
}
#[inline]
pub unsafe fn DwmGetTransportAttributes(pfisremoting: *mut super::super::Foundation::BOOL, pfisconnected: *mut super::super::Foundation::BOOL, pdwgeneration: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetTransportAttributes(pfisremoting : *mut super::super::Foundation:: BOOL, pfisconnected : *mut super::super::Foundation:: BOOL, pdwgeneration : *mut u32) -> windows_core::HRESULT);
    DwmGetTransportAttributes(pfisremoting, pfisconnected, pdwgeneration).ok()
}
#[inline]
pub unsafe fn DwmGetUnmetTabRequirements<P0>(appwindow: P0) -> windows_core::Result<DWM_TAB_WINDOW_REQUIREMENTS>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetUnmetTabRequirements(appwindow : super::super::Foundation:: HWND, value : *mut DWM_TAB_WINDOW_REQUIREMENTS) -> windows_core::HRESULT);
    let mut result__ = std::mem::zeroed();
    DwmGetUnmetTabRequirements(appwindow.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DwmGetWindowAttribute<P0>(hwnd: P0, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *mut core::ffi::c_void, cbattribute: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmGetWindowAttribute(hwnd : super::super::Foundation:: HWND, dwattribute : u32, pvattribute : *mut core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    DwmGetWindowAttribute(hwnd.param().abi(), dwattribute.0 as _, pvattribute, cbattribute).ok()
}
#[inline]
pub unsafe fn DwmInvalidateIconicBitmaps<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmInvalidateIconicBitmaps(hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DwmInvalidateIconicBitmaps(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DwmIsCompositionEnabled() -> windows_core::Result<super::super::Foundation::BOOL> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmIsCompositionEnabled(pfenabled : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = std::mem::zeroed();
    DwmIsCompositionEnabled(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DwmModifyPreviousDxFrameDuration<P0, P1>(hwnd: P0, crefreshes: i32, frelative: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmModifyPreviousDxFrameDuration(hwnd : super::super::Foundation:: HWND, crefreshes : i32, frelative : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DwmModifyPreviousDxFrameDuration(hwnd.param().abi(), crefreshes, frelative.param().abi()).ok()
}
#[inline]
pub unsafe fn DwmQueryThumbnailSourceSize(hthumbnail: isize) -> windows_core::Result<super::super::Foundation::SIZE> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmQueryThumbnailSourceSize(hthumbnail : isize, psize : *mut super::super::Foundation:: SIZE) -> windows_core::HRESULT);
    let mut result__ = std::mem::zeroed();
    DwmQueryThumbnailSourceSize(hthumbnail, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DwmRegisterThumbnail<P0, P1>(hwnddestination: P0, hwndsource: P1) -> windows_core::Result<isize>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmRegisterThumbnail(hwnddestination : super::super::Foundation:: HWND, hwndsource : super::super::Foundation:: HWND, phthumbnailid : *mut isize) -> windows_core::HRESULT);
    let mut result__ = std::mem::zeroed();
    DwmRegisterThumbnail(hwnddestination.param().abi(), hwndsource.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DwmRenderGesture(gt: GESTURE_TYPE, ccontacts: u32, pdwpointerid: *const u32, ppoints: *const super::super::Foundation::POINT) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmRenderGesture(gt : GESTURE_TYPE, ccontacts : u32, pdwpointerid : *const u32, ppoints : *const super::super::Foundation:: POINT) -> windows_core::HRESULT);
    DwmRenderGesture(gt, ccontacts, pdwpointerid, ppoints).ok()
}
#[inline]
pub unsafe fn DwmSetDxFrameDuration<P0>(hwnd: P0, crefreshes: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmSetDxFrameDuration(hwnd : super::super::Foundation:: HWND, crefreshes : i32) -> windows_core::HRESULT);
    DwmSetDxFrameDuration(hwnd.param().abi(), crefreshes).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DwmSetIconicLivePreviewBitmap<P0, P1>(hwnd: P0, hbmp: P1, pptclient: Option<*const super::super::Foundation::POINT>, dwsitflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::Gdi::HBITMAP>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmSetIconicLivePreviewBitmap(hwnd : super::super::Foundation:: HWND, hbmp : super::Gdi:: HBITMAP, pptclient : *const super::super::Foundation:: POINT, dwsitflags : u32) -> windows_core::HRESULT);
    DwmSetIconicLivePreviewBitmap(hwnd.param().abi(), hbmp.param().abi(), core::mem::transmute(pptclient.unwrap_or(std::ptr::null())), dwsitflags).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DwmSetIconicThumbnail<P0, P1>(hwnd: P0, hbmp: P1, dwsitflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::Gdi::HBITMAP>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmSetIconicThumbnail(hwnd : super::super::Foundation:: HWND, hbmp : super::Gdi:: HBITMAP, dwsitflags : u32) -> windows_core::HRESULT);
    DwmSetIconicThumbnail(hwnd.param().abi(), hbmp.param().abi(), dwsitflags).ok()
}
#[inline]
pub unsafe fn DwmSetPresentParameters<P0>(hwnd: P0, ppresentparams: *mut DWM_PRESENT_PARAMETERS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmSetPresentParameters(hwnd : super::super::Foundation:: HWND, ppresentparams : *mut DWM_PRESENT_PARAMETERS) -> windows_core::HRESULT);
    DwmSetPresentParameters(hwnd.param().abi(), ppresentparams).ok()
}
#[inline]
pub unsafe fn DwmSetWindowAttribute<P0>(hwnd: P0, dwattribute: DWMWINDOWATTRIBUTE, pvattribute: *const core::ffi::c_void, cbattribute: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmSetWindowAttribute(hwnd : super::super::Foundation:: HWND, dwattribute : u32, pvattribute : *const core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    DwmSetWindowAttribute(hwnd.param().abi(), dwattribute.0 as _, pvattribute, cbattribute).ok()
}
#[inline]
pub unsafe fn DwmShowContact(dwpointerid: u32, eshowcontact: DWM_SHOWCONTACT) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmShowContact(dwpointerid : u32, eshowcontact : DWM_SHOWCONTACT) -> windows_core::HRESULT);
    DwmShowContact(dwpointerid, eshowcontact).ok()
}
#[inline]
pub unsafe fn DwmTetherContact<P0>(dwpointerid: u32, fenable: P0, pttether: super::super::Foundation::POINT) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmTetherContact(dwpointerid : u32, fenable : super::super::Foundation:: BOOL, pttether : super::super::Foundation:: POINT) -> windows_core::HRESULT);
    DwmTetherContact(dwpointerid, fenable.param().abi(), core::mem::transmute(pttether)).ok()
}
#[inline]
pub unsafe fn DwmTransitionOwnedWindow<P0>(hwnd: P0, target: DWMTRANSITION_OWNEDWINDOW_TARGET) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dwmapi.dll" "system" fn DwmTransitionOwnedWindow(hwnd : super::super::Foundation:: HWND, target : DWMTRANSITION_OWNEDWINDOW_TARGET) -> windows_core::HRESULT);
    DwmTransitionOwnedWindow(hwnd.param().abi(), target).ok()
}
#[inline]
pub unsafe fn DwmUnregisterThumbnail(hthumbnailid: isize) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmUnregisterThumbnail(hthumbnailid : isize) -> windows_core::HRESULT);
    DwmUnregisterThumbnail(hthumbnailid).ok()
}
#[inline]
pub unsafe fn DwmUpdateThumbnailProperties(hthumbnailid: isize, ptnproperties: *const DWM_THUMBNAIL_PROPERTIES) -> windows_core::Result<()> {
    windows_targets::link!("dwmapi.dll" "system" fn DwmUpdateThumbnailProperties(hthumbnailid : isize, ptnproperties : *const DWM_THUMBNAIL_PROPERTIES) -> windows_core::HRESULT);
    DwmUpdateThumbnailProperties(hthumbnailid, ptnproperties).ok()
}
pub const DWMFLIP3D_DEFAULT: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(0i32);
pub const DWMFLIP3D_EXCLUDEABOVE: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(2i32);
pub const DWMFLIP3D_EXCLUDEBELOW: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(1i32);
pub const DWMFLIP3D_LAST: DWMFLIP3DWINDOWPOLICY = DWMFLIP3DWINDOWPOLICY(3i32);
pub const DWMNCRP_DISABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(1i32);
pub const DWMNCRP_ENABLED: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(2i32);
pub const DWMNCRP_LAST: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(3i32);
pub const DWMNCRP_USEWINDOWSTYLE: DWMNCRENDERINGPOLICY = DWMNCRENDERINGPOLICY(0i32);
pub const DWMSBT_AUTO: DWM_SYSTEMBACKDROP_TYPE = DWM_SYSTEMBACKDROP_TYPE(0i32);
pub const DWMSBT_MAINWINDOW: DWM_SYSTEMBACKDROP_TYPE = DWM_SYSTEMBACKDROP_TYPE(2i32);
pub const DWMSBT_NONE: DWM_SYSTEMBACKDROP_TYPE = DWM_SYSTEMBACKDROP_TYPE(1i32);
pub const DWMSBT_TABBEDWINDOW: DWM_SYSTEMBACKDROP_TYPE = DWM_SYSTEMBACKDROP_TYPE(4i32);
pub const DWMSBT_TRANSIENTWINDOW: DWM_SYSTEMBACKDROP_TYPE = DWM_SYSTEMBACKDROP_TYPE(3i32);
pub const DWMSC_ALL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4294967295u32);
pub const DWMSC_DOWN: DWM_SHOWCONTACT = DWM_SHOWCONTACT(1u32);
pub const DWMSC_DRAG: DWM_SHOWCONTACT = DWM_SHOWCONTACT(4u32);
pub const DWMSC_HOLD: DWM_SHOWCONTACT = DWM_SHOWCONTACT(8u32);
pub const DWMSC_NONE: DWM_SHOWCONTACT = DWM_SHOWCONTACT(0u32);
pub const DWMSC_PENBARREL: DWM_SHOWCONTACT = DWM_SHOWCONTACT(16u32);
pub const DWMSC_UP: DWM_SHOWCONTACT = DWM_SHOWCONTACT(2u32);
pub const DWMTRANSITION_OWNEDWINDOW_NULL: DWMTRANSITION_OWNEDWINDOW_TARGET = DWMTRANSITION_OWNEDWINDOW_TARGET(-1i32);
pub const DWMTRANSITION_OWNEDWINDOW_REPOSITION: DWMTRANSITION_OWNEDWINDOW_TARGET = DWMTRANSITION_OWNEDWINDOW_TARGET(0i32);
pub const DWMTWR_APP_COMPAT: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(512i32);
pub const DWMTWR_GROUP_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(256i32);
pub const DWMTWR_IMPLEMENTED_BY_SYSTEM: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(1i32);
pub const DWMTWR_NONE: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(0i32);
pub const DWMTWR_TABBING_ENABLED: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(64i32);
pub const DWMTWR_USER_POLICY: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(128i32);
pub const DWMTWR_WINDOW_DWM_ATTRIBUTES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(16i32);
pub const DWMTWR_WINDOW_MARGINS: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(32i32);
pub const DWMTWR_WINDOW_REGION: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(8i32);
pub const DWMTWR_WINDOW_RELATIONSHIP: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(2i32);
pub const DWMTWR_WINDOW_STYLES: DWM_TAB_WINDOW_REQUIREMENTS = DWM_TAB_WINDOW_REQUIREMENTS(4i32);
pub const DWMWA_ALLOW_NCPAINT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(4i32);
pub const DWMWA_BORDER_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(34i32);
pub const DWMWA_CAPTION_BUTTON_BOUNDS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(5i32);
pub const DWMWA_CAPTION_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(35i32);
pub const DWMWA_CLOAK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(13i32);
pub const DWMWA_CLOAKED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(14i32);
pub const DWMWA_COLOR_DEFAULT: u32 = 4294967295u32;
pub const DWMWA_COLOR_NONE: u32 = 4294967294u32;
pub const DWMWA_DISALLOW_PEEK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(11i32);
pub const DWMWA_EXCLUDED_FROM_PEEK: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(12i32);
pub const DWMWA_EXTENDED_FRAME_BOUNDS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(9i32);
pub const DWMWA_FLIP3D_POLICY: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(8i32);
pub const DWMWA_FORCE_ICONIC_REPRESENTATION: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(7i32);
pub const DWMWA_FREEZE_REPRESENTATION: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(15i32);
pub const DWMWA_HAS_ICONIC_BITMAP: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(10i32);
pub const DWMWA_LAST: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(39i32);
pub const DWMWA_NCRENDERING_ENABLED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(1i32);
pub const DWMWA_NCRENDERING_POLICY: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(2i32);
pub const DWMWA_NONCLIENT_RTL_LAYOUT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(6i32);
pub const DWMWA_PASSIVE_UPDATE_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(16i32);
pub const DWMWA_SYSTEMBACKDROP_TYPE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(38i32);
pub const DWMWA_TEXT_COLOR: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(36i32);
pub const DWMWA_TRANSITIONS_FORCEDISABLED: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(3i32);
pub const DWMWA_USE_HOSTBACKDROPBRUSH: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(17i32);
pub const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(20i32);
pub const DWMWA_VISIBLE_FRAME_BORDER_THICKNESS: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(37i32);
pub const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(33i32);
pub const DWMWCP_DEFAULT: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(0i32);
pub const DWMWCP_DONOTROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(1i32);
pub const DWMWCP_ROUND: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(2i32);
pub const DWMWCP_ROUNDSMALL: DWM_WINDOW_CORNER_PREFERENCE = DWM_WINDOW_CORNER_PREFERENCE(3i32);
pub const DWM_BB_BLURREGION: u32 = 2u32;
pub const DWM_BB_ENABLE: u32 = 1u32;
pub const DWM_BB_TRANSITIONONMAXIMIZED: u32 = 4u32;
pub const DWM_CLOAKED_APP: u32 = 1u32;
pub const DWM_CLOAKED_INHERITED: u32 = 4u32;
pub const DWM_CLOAKED_SHELL: u32 = 2u32;
pub const DWM_EC_DISABLECOMPOSITION: u32 = 0u32;
pub const DWM_EC_ENABLECOMPOSITION: u32 = 1u32;
pub const DWM_FRAME_DURATION_DEFAULT: i32 = -1i32;
pub const DWM_SIT_DISPLAYFRAME: u32 = 1u32;
pub const DWM_SOURCE_FRAME_SAMPLING_COVERAGE: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(1i32);
pub const DWM_SOURCE_FRAME_SAMPLING_LAST: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(2i32);
pub const DWM_SOURCE_FRAME_SAMPLING_POINT: DWM_SOURCE_FRAME_SAMPLING = DWM_SOURCE_FRAME_SAMPLING(0i32);
pub const DWM_TNP_OPACITY: u32 = 4u32;
pub const DWM_TNP_RECTDESTINATION: u32 = 1u32;
pub const DWM_TNP_RECTSOURCE: u32 = 2u32;
pub const DWM_TNP_SOURCECLIENTAREAONLY: u32 = 16u32;
pub const DWM_TNP_VISIBLE: u32 = 8u32;
pub const GT_PEN_DOUBLETAP: GESTURE_TYPE = GESTURE_TYPE(1i32);
pub const GT_PEN_PRESSANDHOLD: GESTURE_TYPE = GESTURE_TYPE(3i32);
pub const GT_PEN_PRESSANDHOLDABORT: GESTURE_TYPE = GESTURE_TYPE(4i32);
pub const GT_PEN_RIGHTTAP: GESTURE_TYPE = GESTURE_TYPE(2i32);
pub const GT_PEN_TAP: GESTURE_TYPE = GESTURE_TYPE(0i32);
pub const GT_TOUCH_DOUBLETAP: GESTURE_TYPE = GESTURE_TYPE(6i32);
pub const GT_TOUCH_PRESSANDHOLD: GESTURE_TYPE = GESTURE_TYPE(8i32);
pub const GT_TOUCH_PRESSANDHOLDABORT: GESTURE_TYPE = GESTURE_TYPE(9i32);
pub const GT_TOUCH_PRESSANDTAP: GESTURE_TYPE = GESTURE_TYPE(10i32);
pub const GT_TOUCH_RIGHTTAP: GESTURE_TYPE = GESTURE_TYPE(7i32);
pub const GT_TOUCH_TAP: GESTURE_TYPE = GESTURE_TYPE(5i32);
pub const c_DwmMaxAdapters: u32 = 16u32;
pub const c_DwmMaxMonitors: u32 = 16u32;
pub const c_DwmMaxQueuedBuffers: u32 = 8u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWMFLIP3DWINDOWPOLICY(pub i32);
impl windows_core::TypeKind for DWMFLIP3DWINDOWPOLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWMFLIP3DWINDOWPOLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWMFLIP3DWINDOWPOLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWMNCRENDERINGPOLICY(pub i32);
impl windows_core::TypeKind for DWMNCRENDERINGPOLICY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWMNCRENDERINGPOLICY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWMNCRENDERINGPOLICY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWMTRANSITION_OWNEDWINDOW_TARGET(pub i32);
impl windows_core::TypeKind for DWMTRANSITION_OWNEDWINDOW_TARGET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWMTRANSITION_OWNEDWINDOW_TARGET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWMTRANSITION_OWNEDWINDOW_TARGET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWMWINDOWATTRIBUTE(pub i32);
impl windows_core::TypeKind for DWMWINDOWATTRIBUTE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWMWINDOWATTRIBUTE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWMWINDOWATTRIBUTE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWM_SHOWCONTACT(pub u32);
impl windows_core::TypeKind for DWM_SHOWCONTACT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWM_SHOWCONTACT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWM_SHOWCONTACT").field(&self.0).finish()
    }
}
impl DWM_SHOWCONTACT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWM_SHOWCONTACT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWM_SHOWCONTACT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWM_SHOWCONTACT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWM_SHOWCONTACT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWM_SHOWCONTACT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWM_SOURCE_FRAME_SAMPLING(pub i32);
impl windows_core::TypeKind for DWM_SOURCE_FRAME_SAMPLING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWM_SOURCE_FRAME_SAMPLING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWM_SOURCE_FRAME_SAMPLING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWM_SYSTEMBACKDROP_TYPE(pub i32);
impl windows_core::TypeKind for DWM_SYSTEMBACKDROP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWM_SYSTEMBACKDROP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWM_SYSTEMBACKDROP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWM_TAB_WINDOW_REQUIREMENTS(pub i32);
impl windows_core::TypeKind for DWM_TAB_WINDOW_REQUIREMENTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWM_TAB_WINDOW_REQUIREMENTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWM_TAB_WINDOW_REQUIREMENTS").field(&self.0).finish()
    }
}
impl DWM_TAB_WINDOW_REQUIREMENTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DWM_TAB_WINDOW_REQUIREMENTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DWM_TAB_WINDOW_REQUIREMENTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DWM_TAB_WINDOW_REQUIREMENTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DWM_WINDOW_CORNER_PREFERENCE(pub i32);
impl windows_core::TypeKind for DWM_WINDOW_CORNER_PREFERENCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DWM_WINDOW_CORNER_PREFERENCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DWM_WINDOW_CORNER_PREFERENCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GESTURE_TYPE(pub i32);
impl windows_core::TypeKind for GESTURE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GESTURE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GESTURE_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct DWM_BLURBEHIND {
    pub dwFlags: u32,
    pub fEnable: super::super::Foundation::BOOL,
    pub hRgnBlur: super::Gdi::HRGN,
    pub fTransitionOnMaximized: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Copy for DWM_BLURBEHIND {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Clone for DWM_BLURBEHIND {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for DWM_BLURBEHIND {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for DWM_BLURBEHIND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DWM_PRESENT_PARAMETERS {
    pub cbSize: u32,
    pub fQueue: super::super::Foundation::BOOL,
    pub cRefreshStart: u64,
    pub cBuffer: u32,
    pub fUseSourceRate: super::super::Foundation::BOOL,
    pub rateSource: UNSIGNED_RATIO,
    pub cRefreshesPerFrame: u32,
    pub eSampling: DWM_SOURCE_FRAME_SAMPLING,
}
impl Copy for DWM_PRESENT_PARAMETERS {}
impl Clone for DWM_PRESENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DWM_PRESENT_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWM_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DWM_THUMBNAIL_PROPERTIES {
    pub dwFlags: u32,
    pub rcDestination: super::super::Foundation::RECT,
    pub rcSource: super::super::Foundation::RECT,
    pub opacity: u8,
    pub fVisible: super::super::Foundation::BOOL,
    pub fSourceClientAreaOnly: super::super::Foundation::BOOL,
}
impl Copy for DWM_THUMBNAIL_PROPERTIES {}
impl Clone for DWM_THUMBNAIL_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DWM_THUMBNAIL_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWM_THUMBNAIL_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DWM_TIMING_INFO {
    pub cbSize: u32,
    pub rateRefresh: UNSIGNED_RATIO,
    pub qpcRefreshPeriod: u64,
    pub rateCompose: UNSIGNED_RATIO,
    pub qpcVBlank: u64,
    pub cRefresh: u64,
    pub cDXRefresh: u32,
    pub qpcCompose: u64,
    pub cFrame: u64,
    pub cDXPresent: u32,
    pub cRefreshFrame: u64,
    pub cFrameSubmitted: u64,
    pub cDXPresentSubmitted: u32,
    pub cFrameConfirmed: u64,
    pub cDXPresentConfirmed: u32,
    pub cRefreshConfirmed: u64,
    pub cDXRefreshConfirmed: u32,
    pub cFramesLate: u64,
    pub cFramesOutstanding: u32,
    pub cFrameDisplayed: u64,
    pub qpcFrameDisplayed: u64,
    pub cRefreshFrameDisplayed: u64,
    pub cFrameComplete: u64,
    pub qpcFrameComplete: u64,
    pub cFramePending: u64,
    pub qpcFramePending: u64,
    pub cFramesDisplayed: u64,
    pub cFramesComplete: u64,
    pub cFramesPending: u64,
    pub cFramesAvailable: u64,
    pub cFramesDropped: u64,
    pub cFramesMissed: u64,
    pub cRefreshNextDisplayed: u64,
    pub cRefreshNextPresented: u64,
    pub cRefreshesDisplayed: u64,
    pub cRefreshesPresented: u64,
    pub cRefreshStarted: u64,
    pub cPixelsReceived: u64,
    pub cPixelsDrawn: u64,
    pub cBuffersEmpty: u64,
}
impl Copy for DWM_TIMING_INFO {}
impl Clone for DWM_TIMING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for DWM_TIMING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWM_TIMING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct MilMatrix3x2D {
    pub S_11: f64,
    pub S_12: f64,
    pub S_21: f64,
    pub S_22: f64,
    pub DX: f64,
    pub DY: f64,
}
impl Copy for MilMatrix3x2D {}
impl Clone for MilMatrix3x2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for MilMatrix3x2D {
    type TypeKind = windows_core::CopyType;
}
impl Default for MilMatrix3x2D {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct UNSIGNED_RATIO {
    pub uiNumerator: u32,
    pub uiDenominator: u32,
}
impl Copy for UNSIGNED_RATIO {}
impl Clone for UNSIGNED_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl windows_core::TypeKind for UNSIGNED_RATIO {
    type TypeKind = windows_core::CopyType;
}
impl Default for UNSIGNED_RATIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
