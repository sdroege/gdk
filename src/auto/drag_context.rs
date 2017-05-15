// This file was generated by gir (8343e00) from gir-files (71d73f0)
// DO NOT EDIT

use Atom;
use Device;
use DragAction;
#[cfg(feature = "v3_20")]
use DragCancelReason;
use DragProtocol;
use Window;
use ffi;
use glib;
#[cfg(feature = "v3_20")]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_20")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_20")]
use glib_ffi;
#[cfg(feature = "v3_20")]
use libc;
#[cfg(feature = "v3_20")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_20")]
use std::mem::transmute;

glib_wrapper! {
    pub struct DragContext(Object<ffi::GdkDragContext>);

    match fn {
        get_type => || ffi::gdk_drag_context_get_type(),
    }
}

pub trait DragContextExt {
    fn get_actions(&self) -> DragAction;

    fn get_dest_window(&self) -> Window;

    fn get_device(&self) -> Device;

    #[cfg(feature = "v3_20")]
    fn get_drag_window(&self) -> Option<Window>;

    fn get_protocol(&self) -> DragProtocol;

    fn get_selected_action(&self) -> DragAction;

    fn get_source_window(&self) -> Window;

    fn get_suggested_action(&self) -> DragAction;

    fn list_targets(&self) -> Vec<Atom>;

    #[cfg(feature = "v3_20")]
    fn manage_dnd(&self, ipc_window: &Window, actions: DragAction) -> bool;

    fn set_device<P: IsA<Device>>(&self, device: &P);

    #[cfg(feature = "v3_20")]
    fn set_hotspot(&self, hot_x: i32, hot_y: i32);

    #[cfg(feature = "v3_20")]
    fn connect_action_changed<F: Fn(&Self, DragAction) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_20")]
    fn connect_cancel<F: Fn(&Self, DragCancelReason) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_20")]
    fn connect_dnd_finished<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_20")]
    fn connect_drop_performed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<DragContext> + IsA<glib::object::Object>> DragContextExt for O {
    fn get_actions(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_actions(self.to_glib_none().0))
        }
    }

    fn get_dest_window(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_dest_window(self.to_glib_none().0))
        }
    }

    fn get_device(&self) -> Device {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_device(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    fn get_drag_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_drag_window(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> DragProtocol {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_protocol(self.to_glib_none().0))
        }
    }

    fn get_selected_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_selected_action(self.to_glib_none().0))
        }
    }

    fn get_source_window(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_source_window(self.to_glib_none().0))
        }
    }

    fn get_suggested_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_suggested_action(self.to_glib_none().0))
        }
    }

    fn list_targets(&self) -> Vec<Atom> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gdk_drag_context_list_targets(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    fn manage_dnd(&self, ipc_window: &Window, actions: DragAction) -> bool {
        unsafe {
            from_glib(ffi::gdk_drag_context_manage_dnd(self.to_glib_none().0, ipc_window.to_glib_none().0, actions.to_glib()))
        }
    }

    fn set_device<P: IsA<Device>>(&self, device: &P) {
        unsafe {
            ffi::gdk_drag_context_set_device(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_20")]
    fn set_hotspot(&self, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gdk_drag_context_set_hotspot(self.to_glib_none().0, hot_x, hot_y);
        }
    }

    #[cfg(feature = "v3_20")]
    fn connect_action_changed<F: Fn(&Self, DragAction) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DragAction) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-changed",
                transmute(action_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    fn connect_cancel<F: Fn(&Self, DragCancelReason) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DragCancelReason) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel",
                transmute(cancel_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    fn connect_dnd_finished<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "dnd-finished",
                transmute(dnd_finished_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    fn connect_drop_performed<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drop-performed",
                transmute(drop_performed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn action_changed_trampoline<P>(this: *mut ffi::GdkDragContext, action: ffi::GdkDragAction, f: glib_ffi::gpointer)
where P: IsA<DragContext> {
    callback_guard!();
    let f: &Box_<Fn(&P, DragAction) + 'static> = transmute(f);
    f(&DragContext::from_glib_none(this).downcast_unchecked(), from_glib(action))
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn cancel_trampoline<P>(this: *mut ffi::GdkDragContext, reason: ffi::GdkDragCancelReason, f: glib_ffi::gpointer)
where P: IsA<DragContext> {
    callback_guard!();
    let f: &Box_<Fn(&P, DragCancelReason) + 'static> = transmute(f);
    f(&DragContext::from_glib_none(this).downcast_unchecked(), from_glib(reason))
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn dnd_finished_trampoline<P>(this: *mut ffi::GdkDragContext, f: glib_ffi::gpointer)
where P: IsA<DragContext> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&DragContext::from_glib_none(this).downcast_unchecked())
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn drop_performed_trampoline<P>(this: *mut ffi::GdkDragContext, time: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<DragContext> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&DragContext::from_glib_none(this).downcast_unchecked(), time)
}
