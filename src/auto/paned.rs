// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Buildable;
use Container;
use Orientable;
use Orientation;
use ResizeMode;
use ScrollType;
use Widget;

glib_wrapper! {
    pub struct Paned(Object<gtk_sys::GtkPaned, gtk_sys::GtkPanedClass, PanedClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_paned_get_type(),
    }
}

impl Paned {
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_paned_new(orientation.to_glib())).unsafe_cast()
        }
    }
}

pub struct PanedBuilder {
    position: Option<i32>,
    position_set: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    wide_handle: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl PanedBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            position_set: None,
            #[cfg(any(feature = "v3_16", feature = "dox"))]
            wide_handle: None,
            border_width: None,
            child: None,
            resize_mode: None,
            app_paintable: None,
            can_default: None,
            can_focus: None,
            events: None,
            expand: None,
            #[cfg(any(feature = "v3_20", feature = "dox"))]
            focus_on_click: None,
            halign: None,
            has_default: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            no_show_all: None,
            opacity: None,
            parent: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
            orientation: None,
        }
    }

    pub fn build(self) -> Paned {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref position) = self.position {
            properties.push(("position", position));
        }
        if let Some(ref position_set) = self.position_set {
            properties.push(("position-set", position_set));
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref wide_handle) = self.wide_handle {
                properties.push(("wide-handle", wide_handle));
            }
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new(Paned::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn position(mut self, position: i32) -> Self {
        self.position = Some(position);
        self
    }

    pub fn position_set(mut self, position_set: bool) -> Self {
        self.position_set = Some(position_set);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn wide_handle(mut self, wide_handle: bool) -> Self {
        self.wide_handle = Some(wide_handle);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child(mut self, child: &Widget) -> Self {
        self.child = Some(child.clone());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent(mut self, parent: &Container) -> Self {
        self.parent = Some(parent.clone());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_PANED: Option<&Paned> = None;

pub trait PanedExt: 'static {
    fn add1<P: IsA<Widget>>(&self, child: &P);

    fn add2<P: IsA<Widget>>(&self, child: &P);

    fn get_child1(&self) -> Option<Widget>;

    fn get_child2(&self) -> Option<Widget>;

    fn get_handle_window(&self) -> Option<gdk::Window>;

    fn get_position(&self) -> i32;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_wide_handle(&self) -> bool;

    fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool);

    fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool);

    fn set_position(&self, position: i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_wide_handle(&self, wide: bool);

    fn get_property_max_position(&self) -> i32;

    fn get_property_min_position(&self) -> i32;

    fn get_property_position_set(&self) -> bool;

    fn set_property_position_set(&self, position_set: bool);

    fn get_child_resize<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool);

    fn get_child_shrink<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool);

    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_accept_position(&self) -> bool;

    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel_position(&self) -> bool;

    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool;

    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool;

    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool;

    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_handle_focus(&self) -> bool;

    fn connect_property_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Paned>> PanedExt for O {
    fn add1<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_paned_add1(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn add2<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_paned_add2(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_child1(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_paned_get_child1(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_child2(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_paned_get_child2(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_handle_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(gtk_sys::gtk_paned_get_handle_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_position(&self) -> i32 {
        unsafe { gtk_sys::gtk_paned_get_position(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_wide_handle(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_paned_get_wide_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            gtk_sys::gtk_paned_pack1(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.to_glib(),
                shrink.to_glib(),
            );
        }
    }

    fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            gtk_sys::gtk_paned_pack2(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.to_glib(),
                shrink.to_glib(),
            );
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            gtk_sys::gtk_paned_set_position(self.as_ref().to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_wide_handle(&self, wide: bool) {
        unsafe {
            gtk_sys::gtk_paned_set_wide_handle(self.as_ref().to_glib_none().0, wide.to_glib());
        }
    }

    fn get_property_max_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-position` getter")
                .unwrap()
        }
    }

    fn get_property_min_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"min-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-position` getter")
                .unwrap()
        }
    }

    fn get_property_position_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"position-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `position-set` getter")
                .unwrap()
        }
    }

    fn set_property_position_set(&self, position_set: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"position-set\0".as_ptr() as *const _,
                Value::from(&position_set).to_glib_none().0,
            );
        }
    }

    fn get_child_resize<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"resize\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `resize` getter")
                .unwrap()
        }
    }

    fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"resize\0".as_ptr() as *const _,
                Value::from(&resize).to_glib_none().0,
            );
        }
    }

    fn get_child_shrink<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gtk_sys::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"shrink\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shrink` getter")
                .unwrap()
        }
    }

    fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool) {
        unsafe {
            gtk_sys::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut gtk_sys::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"shrink\0".as_ptr() as *const _,
                Value::from(&shrink).to_glib_none().0,
            );
        }
    }

    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accept_position_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-position\0".as_ptr() as *const _,
                Some(transmute(accept_position_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_accept_position(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("accept-position", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_accept_position`")
            .unwrap()
    }

    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_position_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel-position\0".as_ptr() as *const _,
                Some(transmute(cancel_position_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancel_position(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("cancel-position", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cancel_position`")
            .unwrap()
    }

    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_child_focus_trampoline<P, F: Fn(&P, bool) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            reversed: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Paned::from_glib_borrow(this).unsafe_cast(),
                from_glib(reversed),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-child-focus\0".as_ptr() as *const _,
                Some(transmute(cycle_child_focus_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("cycle-child-focus", &[&reversed])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cycle_child_focus`")
            .unwrap()
    }

    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_handle_focus_trampoline<P, F: Fn(&P, bool) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            reversed: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Paned::from_glib_borrow(this).unsafe_cast(),
                from_glib(reversed),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-handle-focus\0".as_ptr() as *const _,
                Some(transmute(cycle_handle_focus_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("cycle-handle-focus", &[&reversed])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cycle_handle_focus`")
            .unwrap()
    }

    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_handle_trampoline<P, F: Fn(&P, ScrollType) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            scroll_type: gtk_sys::GtkScrollType,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Paned::from_glib_borrow(this).unsafe_cast(),
                from_glib(scroll_type),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-handle\0".as_ptr() as *const _,
                Some(transmute(move_handle_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("move-handle", &[&scroll_type])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_move_handle`")
            .unwrap()
    }

    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_handle_focus_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut gtk_sys::GtkPaned,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-handle-focus\0".as_ptr() as *const _,
                Some(transmute(
                    toggle_handle_focus_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_toggle_handle_focus(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject)
                .emit("toggle-handle-focus", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_toggle_handle_focus`")
            .unwrap()
    }

    fn connect_property_max_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_position_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPaned,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-position\0".as_ptr() as *const _,
                Some(transmute(
                    notify_max_position_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_min_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_position_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPaned,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-position\0".as_ptr() as *const _,
                Some(transmute(
                    notify_min_position_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPaned,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute(notify_position_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_position_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPaned,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position-set\0".as_ptr() as *const _,
                Some(transmute(
                    notify_position_set_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wide_handle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkPaned,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Paned>,
        {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wide-handle\0".as_ptr() as *const _,
                Some(transmute(notify_wide_handle_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Paned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Paned")
    }
}
