// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use TextBuffer;
use ffi;
use gdk;
use gdk_pixbuf;
use glib;
use glib::translate::*;

glib_wrapper! {
    pub struct Clipboard(Object<ffi::GtkClipboard>);

    match fn {
        get_type => || ffi::gtk_clipboard_get_type(),
    }
}

impl Clipboard {
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_clipboard_clear(self.to_glib_none().0);
        }
    }

    pub fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_owner(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_owner(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_22")]
    //pub fn get_selection(&self) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_clipboard_get_selection() }
    //}

    //pub fn request_contents(&self, target: /*Ignored*/&gdk::Atom, callback: /*Unknown conversion*//*Unimplemented*/ClipboardReceivedFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_contents() }
    //}

    //pub fn request_image(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardImageReceivedFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_image() }
    //}

    //pub fn request_rich_text(&self, buffer: &TextBuffer, callback: /*Unknown conversion*//*Unimplemented*/ClipboardRichTextReceivedFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_rich_text() }
    //}

    //pub fn request_targets(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTargetsReceivedFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_targets() }
    //}

    //pub fn request_text(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardTextReceivedFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_text() }
    //}

    //pub fn request_uris(&self, callback: /*Unknown conversion*//*Unimplemented*/ClipboardURIReceivedFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_clipboard_request_uris() }
    //}

    //pub fn set_can_store(&self, targets: /*Ignored*/&[&TargetEntry], n_targets: i32) {
    //    unsafe { TODO: call ffi::gtk_clipboard_set_can_store() }
    //}

    pub fn set_image(&self, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_clipboard_set_image(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    //pub fn set_with_data(&self, targets: /*Ignored*/&[&TargetEntry], n_targets: u32, get_func: /*Unknown conversion*//*Unimplemented*/ClipboardGetFunc, clear_func: /*Unknown conversion*//*Unimplemented*/ClipboardClearFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi::gtk_clipboard_set_with_data() }
    //}

    //pub fn set_with_owner<T: IsA<glib::Object>>(&self, targets: /*Ignored*/&[&TargetEntry], n_targets: u32, get_func: /*Unknown conversion*//*Unimplemented*/ClipboardGetFunc, clear_func: /*Unknown conversion*//*Unimplemented*/ClipboardClearFunc, owner: &T) -> bool {
    //    unsafe { TODO: call ffi::gtk_clipboard_set_with_owner() }
    //}

    pub fn store(&self) {
        unsafe {
            ffi::gtk_clipboard_store(self.to_glib_none().0);
        }
    }

    //pub fn wait_for_contents(&self, target: /*Ignored*/&gdk::Atom) -> Option<SelectionData> {
    //    unsafe { TODO: call ffi::gtk_clipboard_wait_for_contents() }
    //}

    pub fn wait_for_image(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_image(self.to_glib_none().0))
        }
    }

    //pub fn wait_for_rich_text(&self, buffer: &TextBuffer, format: /*Ignored*/&mut gdk::Atom) -> (/*Unimplemented*/Option<CArray TypeId { ns_id: 0, id: 3 }>, /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::gtk_clipboard_wait_for_rich_text() }
    //}

    //pub fn wait_for_targets(&self, targets: /*Unimplemented*/Vec<gdk::Atom>) -> Option<i32> {
    //    unsafe { TODO: call ffi::gtk_clipboard_wait_for_targets() }
    //}

    pub fn wait_for_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_clipboard_wait_for_text(self.to_glib_none().0))
        }
    }

    pub fn wait_for_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_clipboard_wait_for_uris(self.to_glib_none().0))
        }
    }

    pub fn wait_is_image_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_image_available(self.to_glib_none().0))
        }
    }

    pub fn wait_is_rich_text_available(&self, buffer: &TextBuffer) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_rich_text_available(self.to_glib_none().0, buffer.to_glib_none().0))
        }
    }

    //pub fn wait_is_target_available(&self, target: /*Ignored*/&gdk::Atom) -> bool {
    //    unsafe { TODO: call ffi::gtk_clipboard_wait_is_target_available() }
    //}

    pub fn wait_is_text_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_text_available(self.to_glib_none().0))
        }
    }

    pub fn wait_is_uris_available(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_clipboard_wait_is_uris_available(self.to_glib_none().0))
        }
    }

    //pub fn get(selection: /*Ignored*/&gdk::Atom) -> Option<Clipboard> {
    //    unsafe { TODO: call ffi::gtk_clipboard_get() }
    //}

    #[cfg(feature = "v3_16")]
    pub fn get_default(display: &gdk::Display) -> Option<Clipboard> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_clipboard_get_default(display.to_glib_none().0))
        }
    }

    //pub fn get_for_display(display: &gdk::Display, selection: /*Ignored*/&gdk::Atom) -> Option<Clipboard> {
    //    unsafe { TODO: call ffi::gtk_clipboard_get_for_display() }
    //}

    //pub fn connect_owner_change<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored event: Gdk.EventOwnerChange
    //}
}