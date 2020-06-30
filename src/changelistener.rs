use com::{
    co_class,
    interfaces::IUnknown,
    sys::{FAILED, HRESULT, S_OK},
    ComPtr, ComRc,
};

use winapi::shared::minwindef::DWORD;

use crate::{
    interfaces::{
        IApplicationView, IID_IVirtualDesktopNotification, IVirtualDesktop,
        IVirtualDesktopNotification, IVirtualDesktopNotificationService,
    },
    DesktopID,
};
use std::{cell::Cell, ptr};

/*
/// Create a new virtual desktop listener
fn create_change_listener() -> (
    *mut VirtualDesktopChangeListener,
    Option<ComPtr<dyn IVirtualDesktopNotification>>,
) {
    // TODO: This leaks memory, because when ComPtr reference gets to zero, the
    // VirtualDesktopChangeListener value in the heap is not deleted. This needs
    // a new wrapper that can release the memory of VirtualDesktopChangeListener
    // when ComPtr is dropped.
    let foo = Box::into_raw(VirtualDesktopChangeListener::new());
    let hr: Option<ComPtr<dyn IVirtualDesktopNotification>> = {
        let mut ipv = ptr::null_mut();
        let res = unsafe { (*foo).query_interface(&IID_IVirtualDesktopNotification, &mut ipv) };
        if FAILED(res) || ipv.is_null() {
            None
        } else {
            Some(unsafe { ComPtr::new(ipv as *mut *mut _) })
        }
    };
    return (foo, hr);
}

pub fn register_change_listener(
    virtualdesktop_notification_service: ComRc<dyn IVirtualDesktopNotificationService>,
) -> Box<dyn Fn() -> ()> {
    let (ptr, hr) = create_change_listener();
    let mut cookiee: DWORD = 0;
    let res: i32 =
        unsafe { virtualdesktop_notification_service.register(hr.unwrap(), &mut cookiee) };
    if FAILED(res) {
        println!("Failure to register {:?} {:?}", res as u32, cookiee);
    } else {
        println!("Registered listener {:?}", cookiee);
        unsafe {
            println!("Listener ptr ref count {:?}", (*ptr).__refcnt.clone());
        }
    }
    // Box::new(move || {
    //     unsafe {
    //         virtualdesktop_notification_service.unregister(cookiee);
    //         unsafe {
    //             println!("Listener ptr ref count {:?}", (*ptr).__refcnt.clone());
    //         }

    //         // This probably does not work correctly, but it's a first try
    //         // Idea is to cleanup the Box created in create_change_listener
    //         if (*ptr).__refcnt == 0.into() {
    //             Box::from_raw(ptr);
    //         }
    //     }
    //     ()
    // })
}
*/

#[co_class(implements(IVirtualDesktopNotification))]
pub struct VirtualDesktopChangeListener {
    service: Cell<Option<ComRc<dyn IVirtualDesktopNotificationService>>>,
    cookie: Cell<u32>,
    receiver: u32,
}

impl IVirtualDesktopNotification for VirtualDesktopChangeListener {
    unsafe fn virtual_desktop_created(&self, _desktop: ComPtr<dyn IVirtualDesktop>) -> HRESULT {
        // todo!()
        debug_print!("Desktop destroy created.");
        S_OK
    }
    unsafe fn virtual_desktop_destroy_begin(
        &self,
        _destroyed_desktop: ComPtr<dyn IVirtualDesktop>,
        _fallback_desktop: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT {
        // todo!()
        debug_print!("Desktop destroy begin.");
        S_OK
    }
    unsafe fn virtual_desktop_destroy_failed(
        &self,
        _destroyed_desktop: ComPtr<dyn IVirtualDesktop>,
        _fallback_desktop: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT {
        // todo!()
        debug_print!("Desktop destroy failed.");
        S_OK
    }
    unsafe fn virtual_desktop_destroyed(
        &self,
        _destroyed_desktop: ComPtr<dyn IVirtualDesktop>,
        _fallback_desktop: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT {
        // todo!()
        debug_print!("Desktop destroyed.");
        S_OK
    }
    unsafe fn view_virtual_desktop_changed(&self, view: ComPtr<dyn IApplicationView>) -> HRESULT {
        // todo!()
        debug_print!("View changed in desktop.");
        S_OK
    }
    unsafe fn current_virtual_desktop_changed(
        &self,
        old_desktop: ComPtr<dyn IVirtualDesktop>,
        new_desktop: ComPtr<dyn IVirtualDesktop>,
    ) -> HRESULT {
        // todo!()
        let mut g: DesktopID = Default::default();
        old_desktop.get_id(&mut g);
        // debug_print!("Desktop changed, old: {:?}", g);
        new_desktop.get_id(&mut g);
        // debug_print!("Desktop changed, new: {:?}", g);
        S_OK
    }
}

impl Drop for VirtualDesktopChangeListener {
    fn drop(&mut self) {
        match self.service.take() {
            Some(s) => {
                if self.cookie.get() != 0 {
                    unsafe {
                        debug_print!("Unregister a listener {:?}", self.cookie.get());
                        s.unregister(self.cookie.get());
                    }
                }
            }
            None => {}
        }
    }
}

impl VirtualDesktopChangeListener {
    // Try to create and register a change listener
    pub(crate) fn register(
        service: ComRc<dyn IVirtualDesktopNotificationService>,
    ) -> Result<Box<VirtualDesktopChangeListener>, i32> {
        let listener: Box<VirtualDesktopChangeListener> = VirtualDesktopChangeListener::new();

        // let ptr = unsafe { ComPtr::new(listener.__ivirtualdesktopnotificationvptr) };
        // Retrieve interface pointer to IVirtualDesktopNotification
        let mut ipv = ptr::null_mut();
        let res = unsafe { listener.query_interface(&IID_IVirtualDesktopNotification, &mut ipv) };
        if !FAILED(res) && !ipv.is_null() {
            let ptr: ComPtr<dyn IVirtualDesktopNotification> =
                unsafe { ComPtr::new(ipv as *mut *mut _) };

            // Register the IVirtualDesktopNotification to the service
            let mut cookie: DWORD = 0;
            let res2: i32 = unsafe { service.register(ptr, &mut cookie) };
            if FAILED(res2) {
                Err(res)
            } else {
                debug_print!("Register a listener {:?}", cookie);
                // dbg!(format!("Register a listener {:?}", cookie));
                listener.service.set(Some(service));
                listener.cookie.set(cookie);
                Ok(listener)
            }
        } else {
            Err(res)
        }
    }

    fn new() -> Box<VirtualDesktopChangeListener> {
        VirtualDesktopChangeListener::allocate(Cell::new(None), Cell::new(0), 0)
    }
}
