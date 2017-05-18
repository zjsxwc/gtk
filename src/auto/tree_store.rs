// This file was generated by gir (b010d34) from gir-files (71d73f0)
// DO NOT EDIT

use TreeIter;
use TreeModel;
use TreeSortable;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct TreeStore(Object<ffi::GtkTreeStore>): TreeModel, TreeSortable;

    match fn {
        get_type => || ffi::gtk_tree_store_get_type(),
    }
}

impl TreeStore {
    //pub fn new(n_columns: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeStore {
    //    unsafe { TODO: call ffi::gtk_tree_store_new() }
    //}

    //pub fn newv(n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) -> TreeStore {
    //    unsafe { TODO: call ffi::gtk_tree_store_newv() }
    //}
}

pub trait TreeStoreExt {
    fn append<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P) -> TreeIter;

    fn clear(&self);

    fn insert<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, position: i32) -> TreeIter;

    fn insert_after<'a, 'b, P: Into<Option<&'a TreeIter>>, Q: Into<Option<&'b TreeIter>>>(&self, parent: P, sibling: Q) -> TreeIter;

    fn insert_before<'a, 'b, P: Into<Option<&'a TreeIter>>, Q: Into<Option<&'b TreeIter>>>(&self, parent: P, sibling: Q) -> TreeIter;

    //fn insert_with_values<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeIter;

    //fn insert_with_valuesv<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, position: i32, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) -> TreeIter;

    fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool;

    fn iter_depth(&self, iter: &TreeIter) -> i32;

    fn iter_is_valid(&self, iter: &TreeIter) -> bool;

    fn move_after<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: &TreeIter, position: P);

    fn move_before<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: &TreeIter, position: P);

    fn prepend<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P) -> TreeIter;

    fn remove(&self, iter: &TreeIter) -> bool;

    //fn reorder<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 });

    //fn set(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn set_column_types(&self, n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 });

    //fn set_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn set_valuesv(&self, iter: &TreeIter, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32);

    fn swap(&self, a: &TreeIter, b: &TreeIter);
}

impl<O: IsA<TreeStore>> TreeStoreExt for O {
    fn append<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P) -> TreeIter {
        let parent = parent.into();
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_append(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            iter
        }
    }

    fn clear(&self) {
        unsafe {
            ffi::gtk_tree_store_clear(self.to_glib_none().0);
        }
    }

    fn insert<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, position: i32) -> TreeIter {
        let parent = parent.into();
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), position);
            iter
        }
    }

    fn insert_after<'a, 'b, P: Into<Option<&'a TreeIter>>, Q: Into<Option<&'b TreeIter>>>(&self, parent: P, sibling: Q) -> TreeIter {
        let parent = parent.into();
        let sibling = sibling.into();
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_after(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    fn insert_before<'a, 'b, P: Into<Option<&'a TreeIter>>, Q: Into<Option<&'b TreeIter>>>(&self, parent: P, sibling: Q) -> TreeIter {
        let parent = parent.into();
        let sibling = sibling.into();
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_insert_before(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0), mut_override(sibling.to_glib_none().0));
            iter
        }
    }

    //fn insert_with_values<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, position: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_tree_store_insert_with_values() }
    //}

    //fn insert_with_valuesv<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, position: i32, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) -> TreeIter {
    //    unsafe { TODO: call ffi::gtk_tree_store_insert_with_valuesv() }
    //}

    fn is_ancestor(&self, iter: &TreeIter, descendant: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_is_ancestor(self.to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(descendant.to_glib_none().0)))
        }
    }

    fn iter_depth(&self, iter: &TreeIter) -> i32 {
        unsafe {
            ffi::gtk_tree_store_iter_depth(self.to_glib_none().0, mut_override(iter.to_glib_none().0))
        }
    }

    fn iter_is_valid(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_iter_is_valid(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    fn move_after<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: &TreeIter, position: P) {
        let position = position.into();
        unsafe {
            ffi::gtk_tree_store_move_after(self.to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(position.to_glib_none().0));
        }
    }

    fn move_before<'a, P: Into<Option<&'a TreeIter>>>(&self, iter: &TreeIter, position: P) {
        let position = position.into();
        unsafe {
            ffi::gtk_tree_store_move_before(self.to_glib_none().0, mut_override(iter.to_glib_none().0), mut_override(position.to_glib_none().0));
        }
    }

    fn prepend<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P) -> TreeIter {
        let parent = parent.into();
        unsafe {
            let mut iter = TreeIter::uninitialized();
            ffi::gtk_tree_store_prepend(self.to_glib_none().0, iter.to_glib_none_mut().0, mut_override(parent.to_glib_none().0));
            iter
        }
    }

    fn remove(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_store_remove(self.to_glib_none().0, mut_override(iter.to_glib_none().0)))
        }
    }

    //fn reorder<'a, P: Into<Option<&'a TreeIter>>>(&self, parent: P, new_order: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }) {
    //    unsafe { TODO: call ffi::gtk_tree_store_reorder() }
    //}

    //fn set(&self, iter: &TreeIter, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set() }
    //}

    //fn set_column_types(&self, n_columns: i32, types: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 30 }) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_column_types() }
    //}

    //fn set_valist(&self, iter: &TreeIter, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_valist() }
    //}

    //fn set_valuesv(&self, iter: &TreeIter, columns: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, values: &[&glib::Value], n_values: i32) {
    //    unsafe { TODO: call ffi::gtk_tree_store_set_valuesv() }
    //}

    fn swap(&self, a: &TreeIter, b: &TreeIter) {
        unsafe {
            ffi::gtk_tree_store_swap(self.to_glib_none().0, mut_override(a.to_glib_none().0), mut_override(b.to_glib_none().0));
        }
    }
}
