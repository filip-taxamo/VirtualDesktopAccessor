#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use com::com_interface;
use com::{
    interfaces::IUnknown,
    sys::{CLSID, GUID, HRESULT},
    ComInterface, ComPtr, ComRc, IID,
};
use std::ffi::c_void;

use winapi::shared::minwindef::{BOOL, DWORD, INT, LPVOID, UINT, ULONG};
use winapi::shared::ntdef::{PCWSTR, PWSTR, ULONGLONG};
use winapi::shared::windef::{HWND, RECT, SIZE};
use winapi::winrt::hstring::HSTRING;

pub const CLSID_ImmersiveShell: CLSID = CLSID {
    data1: 0xC2F03A33,
    data2: 0x21F5,
    data3: 0x47FA,
    data4: [0xB4, 0xBB, 0x15, 0x63, 0x62, 0xA2, 0xF2, 0x39],
};

pub const CLSID_IVirtualNotificationService: CLSID = CLSID {
    data1: 0xA501FDEC,
    data2: 0x4A09,
    data3: 0x464C,
    data4: [0xAE, 0x4E, 0x1B, 0x9C, 0x21, 0xB8, 0x49, 0x18],
};

pub const IID_IVirtualDesktopNotification: IID = IID {
    data1: 0xC179334C,
    data2: 0x4295,
    data3: 0x40D3,
    data4: [0xBE, 0xA1, 0xC6, 0x54, 0xD9, 0x65, 0x60, 0x5A],
};

pub const CLSID_VirtualDesktopManagerInternal: IID = IID {
    data1: 0xC5E0CDCA,
    data2: 0x7B6E,
    data3: 0x41B2,
    data4: [0x9F, 0xC4, 0xD9, 0x39, 0x75, 0xCC, 0x46, 0x7B],
};

// Ignore following API's:
type IAsyncCallback = UINT;
type IImmersiveMonitor = UINT;
type APPLICATION_VIEW_COMPATIBILITY_POLICY = UINT;
type IShellPositionerPriority = UINT;
type IApplicationViewOperation = UINT;
type APPLICATION_VIEW_CLOAK_TYPE = UINT;
type IApplicationViewPosition = UINT;
type IImmersiveApplication = UINT;
type IApplicationViewChangeListener = UINT;

/*
Notepad++ replaces for fn PascalCase -> fn pascal_case
fn ([A-Z])
fn \L$1

fn ([a-z_]+)([A-Z]+)
fn $1_\L$2

Other:
STDMETHOD\((\S+)\)
fn $1

) PURE;
) -> HRESULT;
*/

#[com_interface("6d5140c1-7436-11ce-8034-00aa006009fa")]
pub trait IServiceProvider: IUnknown {
    unsafe fn query_service(
        &self,
        guidService: *const GUID,
        riid: *const IID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT;
    unsafe fn remote_query_service(
        &self,
        guidService: *const GUID,
        riid: *const IID,
        ppvObject: *mut *mut IUnknown,
    ) -> HRESULT;
}

#[com_interface("a5cd92ff-29be-454c-8d04-d82879fb3f1b")]
pub trait IVirtualDesktopManager: IUnknown {
    unsafe fn is_window_on_current_virtual_desktop(
        &self,
        topLevelWindow: HWND,
        outOnCurrentDesktop: *mut bool,
    ) -> HRESULT;
    unsafe fn get_window_desktop_id(
        &self,
        topLevelWindow: HWND,
        outDesktopId: *mut GUID,
    ) -> HRESULT;
    unsafe fn move_window_to_desktop(
        &self,
        topLevelWindow: HWND,
        desktopId: *const GUID,
    ) -> HRESULT;
}

#[com_interface("372E1D3B-38D3-42E4-A15B-8AB2B178F513")]
pub trait IApplicationView: IUnknown {
    /* IInspecateble */
    unsafe fn get_iids(&self, outIidCount: *mut ULONG, outOptIidArrayPtr: *mut *mut IID)
        -> HRESULT;
    unsafe fn get_runtime_class_name(&self, outOptClassName: *mut HSTRING) -> HRESULT;
    unsafe fn get_trust_level(&self, ptrTrustLevel: LPVOID) -> HRESULT;

    /* IApplicationView methods */
    unsafe fn set_focus(&self) -> HRESULT;
    unsafe fn switch_to(&self) -> HRESULT;

    unsafe fn try_invoke_back(&self, ptrAsyncCallback: IAsyncCallback) -> HRESULT;
    unsafe fn get_thumbnail_window(&self, outHwnd: *mut HWND) -> HRESULT;
    unsafe fn get_monitor(&self, outMonitors: *mut *mut IImmersiveMonitor) -> HRESULT;
    unsafe fn get_visibility(&self, outInt: LPVOID) -> HRESULT;
    unsafe fn set_cloak(
        &self,
        applicationViewCloakType: APPLICATION_VIEW_CLOAK_TYPE,
        unknown: INT,
    ) -> HRESULT;
    unsafe fn get_position(&self, unknownIid: *const IID, unknownArrayPtr: LPVOID) -> HRESULT;
    unsafe fn set_position(&self, viewPosition: *mut IApplicationViewPosition) -> HRESULT;
    unsafe fn insert_after_window(&self, window: HWND) -> HRESULT;
    unsafe fn get_extended_frame_position(&self, rect: *mut RECT) -> HRESULT;
    unsafe fn get_app_user_model_id(&self, id: *mut PWSTR) -> HRESULT; // Proc17
    unsafe fn set_app_user_model_id(&self, id: PCWSTR) -> HRESULT;
    unsafe fn is_equal_by_app_user_model_id(&self, id: PCWSTR, outResult: *mut INT) -> HRESULT;
    /*** IApplicationView methods ***/
    unsafe fn get_view_state(&self, outState: *mut UINT) -> HRESULT; // Proc20
    unsafe fn set_view_state(&self, state: UINT) -> HRESULT; // Proc21
    unsafe fn get_neediness(&self, outNeediness: *mut INT) -> HRESULT; // Proc22
    unsafe fn get_last_activation_timestamp(&self, outTimestamp: *mut ULONGLONG) -> HRESULT;
    unsafe fn set_last_activation_timestamp(&self, timestamp: ULONGLONG) -> HRESULT;
    unsafe fn get_virtual_desktop_id(&self, outDesktopGuid: *mut GUID) -> HRESULT;
    unsafe fn set_virtual_desktop_id(&self, desktopGuid: *const GUID) -> HRESULT;
    unsafe fn get_show_in_switchers(&self, outShow: *mut INT) -> HRESULT;
    unsafe fn set_show_in_switchers(&self, show: INT) -> HRESULT;
    unsafe fn get_scale_factor(&self, outScaleFactor: *mut INT) -> HRESULT;
    unsafe fn can_receive_input(&self, outCan: *mut BOOL) -> HRESULT;
    unsafe fn get_compatibility_policy_type(
        &self,
        outPolicyType: *mut APPLICATION_VIEW_COMPATIBILITY_POLICY,
    ) -> HRESULT;
    unsafe fn set_compatibility_policy_type(
        &self,
        policyType: APPLICATION_VIEW_COMPATIBILITY_POLICY,
    ) -> HRESULT;

    //unsafe fn get_position_priority(&self, THIS_ IShellPositionerPriority**) -> HRESULT; // removed in 1803
    //unsafe fn set_position_priority(&self, THIS_ IShellPositionerPriority*) -> HRESULT; // removed in 1803

    unsafe fn get_size_constraints(
        &self,
        monitor: *mut IImmersiveMonitor,
        outSize1: *mut SIZE,
        outSize2: *mut SIZE,
    ) -> HRESULT;
    unsafe fn get_size_constraints_for_dpi(
        &self,
        dpi: UINT,
        outSize1: *mut SIZE,
        outSize2: *mut SIZE,
    ) -> HRESULT;
    unsafe fn set_size_constraints_for_dpi(
        &self,
        dpi: *const UINT,
        size1: *const SIZE,
        size2: *const SIZE,
    ) -> HRESULT;

    //unsafe fn query_size_constraints_from_app)(&self, THIS PURE; // removed in 1803

    unsafe fn on_min_size_preferences_updated(&self, window: HWND) -> HRESULT;
    unsafe fn apply_operation(&self, operation: *mut IApplicationViewOperation) -> HRESULT;
    unsafe fn is_tray(&self, outIs: *mut BOOL) -> HRESULT;
    unsafe fn is_in_high_zorder_band(&self, outIs: *mut BOOL) -> HRESULT;
    unsafe fn is_splash_screen_presented(&self, outIs: *mut BOOL) -> HRESULT;
    unsafe fn flash(&self) -> HRESULT;
    unsafe fn get_root_switchable_owner(
        &self,
        appView: *const *mut IApplicationViewVTable,
    ) -> HRESULT; // proc45
    unsafe fn enumerate_ownership_tree(&self, objects: *const *mut IObjectArrayVTable) -> HRESULT; // proc46

    unsafe fn get_enterprise_id(&self, outId: *mut PWSTR) -> HRESULT; // proc47
    unsafe fn is_mirrored(&self, outIs: *mut BOOL) -> HRESULT; //

    unsafe fn unknown1(&self, arg: *mut INT) -> HRESULT;
    unsafe fn unknown2(&self, arg: *mut INT) -> HRESULT;
    unsafe fn unknown3(&self, arg: *mut INT) -> HRESULT;
    unsafe fn unknown4(&self, arg: INT) -> HRESULT;
    unsafe fn unknown5(&self, arg: *mut INT) -> HRESULT;
    unsafe fn unknown6(&self, arg: INT) -> HRESULT;
    unsafe fn unknown7(&self) -> HRESULT;
    unsafe fn unknown8(&self, arg: *mut INT) -> HRESULT;
    unsafe fn unknown9(&self, arg: INT) -> HRESULT;
    unsafe fn unknown10(&self, arg: INT, arg2: INT) -> HRESULT;
    unsafe fn unknown11(&self, arg: INT) -> HRESULT;
    unsafe fn unknown12(&self, arg: *mut SIZE) -> HRESULT;
}

#[com_interface("92ca9dcd-5622-4bba-a805-5e9f541bd8c9")]
pub trait IObjectArray: IUnknown {
    unsafe fn get_count(&self, outPcObjects: *mut UINT) -> HRESULT;
    unsafe fn get_at(&self, uiIndex: UINT, riid: *const IID, outArray: *mut LPVOID) -> HRESULT;
}

#[com_interface("ff72ffdd-be7e-43fc-9c03-ad81681e88e4")]
pub trait IVirtualDesktop: IUnknown {
    unsafe fn is_view_visible(
        &self,
        pView: ComPtr<dyn IApplicationView>,
        outBool: *mut u32,
    ) -> HRESULT;
    unsafe fn get_id(&self, outGuid: *mut GUID) -> HRESULT;
}

#[com_interface("1841c6d7-4f9d-42c0-af41-8747538f10e5")]
pub trait IApplicationViewCollection: IUnknown {
    unsafe fn get_views(&self, outViews: *const *mut IObjectArrayVTable) -> HRESULT;

    unsafe fn get_views_by_zorder(&self, outViews: *const *mut IObjectArrayVTable) -> HRESULT;

    unsafe fn get_views_by_app_user_model_id(
        &self,
        id: PCWSTR,
        outViews: *const *mut IObjectArrayVTable,
    ) -> HRESULT;

    unsafe fn get_view_for_hwnd(
        &self,
        window: HWND,
        outViews: *const *mut IApplicationViewVTable,
    ) -> HRESULT;

    unsafe fn get_view_for_application(
        &self,
        app: *const IImmersiveApplication,
        outViews: *const *mut IApplicationViewVTable,
    ) -> HRESULT;

    unsafe fn get_view_for_app_user_model_id(
        &self,
        id: PCWSTR,
        outViews: *const *mut IApplicationViewVTable,
    ) -> HRESULT;

    unsafe fn get_view_in_focus(&self, outViews: *const *mut IApplicationViewVTable) -> HRESULT;

    unsafe fn unknown1(&self, outViews: *const *mut IApplicationViewVTable) -> HRESULT;

    unsafe fn refresh_collection(&self) -> HRESULT;

    unsafe fn register_for_application_view_changes(
        &self,
        listener: *const IApplicationViewChangeListener,
        outId: *mut DWORD,
    ) -> HRESULT;

    unsafe fn unregister_for_application_view_changes(&self, id: DWORD) -> HRESULT;
}

#[com_interface("C179334C-4295-40D3-BEA1-C654D965605A")]
pub trait IVirtualDesktopNotification: IUnknown {
    unsafe fn virtual_desktop_created(&self, desktop: ComPtr<dyn IVirtualDesktop>) -> HRESULT;

    unsafe fn virtual_desktop_destroy_begin(
        &self,
        desktopDestroyed: ComPtr<dyn IVirtualDesktop>,
        desktopFallback: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT;

    unsafe fn virtual_desktop_destroy_failed(
        &self,
        desktopDestroyed: ComPtr<dyn IVirtualDesktop>,
        desktopFallback: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT;

    unsafe fn virtual_desktop_destroyed(
        &self,
        desktopDestroyed: ComPtr<dyn IVirtualDesktop>,
        desktopFallback: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT;

    unsafe fn view_virtual_desktop_changed(&self, view: ComPtr<dyn IApplicationView>) -> HRESULT;

    unsafe fn current_virtual_desktop_changed(
        &self,
        desktopOld: ComPtr<dyn IVirtualDesktop>,
        desktopNew: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT;
}

#[com_interface("0cd45e71-d927-4f15-8b0a-8fef525337bf")]
pub trait IVirtualDesktopNotificationService: IUnknown {
    unsafe fn register(
        &self,
        notification: ComPtr<dyn IVirtualDesktopNotification>,
        outCookie: *mut DWORD,
    ) -> HRESULT;

    unsafe fn unregister(&self, cookie: DWORD) -> HRESULT;
}

#[com_interface("f31574d6-b682-4cdc-bd56-1827860abec6")]
pub trait IVirtualDesktopManagerInternal: IUnknown {
    unsafe fn get_count(&self, outCount: *mut UINT) -> HRESULT;
    unsafe fn move_view_to_desktop(
        &self,
        view: ComPtr<dyn IApplicationView>,
        desktop: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT;
    unsafe fn can_move_view_between_desktops(
        &self,
        view: ComPtr<dyn IApplicationView>,
        canMove: *mut i32,
    ) -> HRESULT;
    // unsafe fn get_current_desktop(&self, outDesktop: *mut *mut IVirtualDesktopVTable) -> HRESULT;
    unsafe fn get_current_desktop(&self, outDesktop: *const *mut IVirtualDesktopVTable) -> HRESULT;
}

/*

// HKEY_LOCAL_MACHINE\SOFTWARE\Classes\Interface\{F31574D6-B682-4CDC-BD56-1827860ABEC6}
MIDL_INTERFACE("f31574d6-b682-4cdc-bd56-1827860abec6")
IVirtualDesktopManagerInternal : public IUnknown
{
public:


    virtual HRESULT STDMETHODCALLTYPE GetCurrentDesktop(
        IVirtualDesktop** desktop) = 0;

    virtual HRESULT STDMETHODCALLTYPE GetDesktops(
        IObjectArray **ppDesktops) = 0;

    virtual HRESULT STDMETHODCALLTYPE GetAdjacentDesktop(
        IVirtualDesktop *pDesktopReference,
        AdjacentDesktop uDirection,
        IVirtualDesktop **ppAdjacentDesktop) = 0;

    virtual HRESULT STDMETHODCALLTYPE SwitchDesktop(
        IVirtualDesktop *pDesktop) = 0;

    virtual HRESULT STDMETHODCALLTYPE CreateDesktopW(
        IVirtualDesktop **ppNewDesktop) = 0;

    virtual HRESULT STDMETHODCALLTYPE RemoveDesktop(
        IVirtualDesktop *pRemove,
        IVirtualDesktop *pFallbackDesktop) = 0;

    // Since build 10240
    virtual HRESULT STDMETHODCALLTYPE FindDesktop(
        GUID *desktopId,
        IVirtualDesktop **ppDesktop) = 0;
};

*/