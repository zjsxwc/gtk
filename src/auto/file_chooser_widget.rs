// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use Box;
use Container;
use FileChooser;
use FileChooserAction;
use Orientable;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget>): Box, Container, Widget, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.to_glib())).downcast_unchecked()
        }
    }
}

pub trait FileChooserWidgetExt {
    fn get_property_search_mode(&self) -> bool;

    fn set_property_search_mode(&self, search_mode: bool);

    fn get_property_subtitle(&self) -> Option<String>;

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64;

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<FileChooserWidget> + IsA<glib::object::Object>> FileChooserWidgetExt for O {
    fn get_property_search_mode(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "search-mode".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_search_mode(&self, search_mode: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "search-mode".to_glib_none().0, Value::from(&search_mode).to_glib_none().0);
        }
    }

    fn get_property_subtitle(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "subtitle".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "desktop-folder",
                transmute(desktop_folder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "down-folder",
                transmute(down_folder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "home-folder",
                transmute(home_folder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "location-popup",
                transmute(location_popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "location-popup-on-paste",
                transmute(location_popup_on_paste_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "location-toggle-popup",
                transmute(location_toggle_popup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "places-shortcut",
                transmute(places_shortcut_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "quick-bookmark",
                transmute(quick_bookmark_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "recent-shortcut",
                transmute(recent_shortcut_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search-shortcut",
                transmute(search_shortcut_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-hidden",
                transmute(show_hidden_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "up-folder",
                transmute(up_folder_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn desktop_folder_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn down_folder_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn home_folder_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn location_popup_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, path: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(path))
}

unsafe extern "C" fn location_popup_on_paste_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn location_toggle_popup_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn places_shortcut_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn quick_bookmark_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, bookmark_index: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked(), bookmark_index)
}

unsafe extern "C" fn recent_shortcut_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn search_shortcut_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn show_hidden_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn up_folder_trampoline<P>(this: *mut ffi::GtkFileChooserWidget, f: glib_ffi::gpointer)
where P: IsA<FileChooserWidget> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FileChooserWidget::from_glib_none(this).downcast_unchecked())
}
