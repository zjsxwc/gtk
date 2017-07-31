// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TextAttributes(Shared<ffi::GtkTextAttributes>);

    match fn {
        ref => |ptr| ffi::gtk_text_attributes_ref(ptr),
        unref => |ptr| ffi::gtk_text_attributes_unref(ptr),
        get_type => || ffi::gtk_text_attributes_get_type(),
    }
}

impl TextAttributes {
    pub fn new() -> TextAttributes {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_attributes_new())
        }
    }

    pub fn copy(&self) -> Option<TextAttributes> {
        unsafe {
            from_glib_full(ffi::gtk_text_attributes_copy(self.to_glib_none().0))
        }
    }

    pub fn copy_values(&self, dest: &TextAttributes) {
        unsafe {
            ffi::gtk_text_attributes_copy_values(self.to_glib_none().0, dest.to_glib_none().0);
        }
    }
}
