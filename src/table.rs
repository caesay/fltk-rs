use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::Widget;
use fltk_sys::table::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a table
#[derive(WidgetExt, GroupExt, TableExt, Debug, Clone)]
pub struct Table {
    _inner: *mut Fl_Table,
}

/// Creates a table row
#[derive(WidgetExt, GroupExt, TableExt, Debug, Clone)]
pub struct TableRow {
    _inner: *mut Fl_Table_Row,
}

#[repr(i32)]
pub enum TableRowSelectMode {
    SelectNone,
    SelectSingle,
    SelectMulti,
}

impl TableRow {
    pub fn set_type(&mut self, val: TableRowSelectMode) {
        unsafe { Fl_Table_Row_set_type(self._inner, val as i32) }
    }

    pub fn get_type(&self) -> TableRowSelectMode {
        unsafe { mem::transmute(Fl_Table_Row_get_type(self._inner)) }
    }

    pub fn row_selected(&mut self, row: i32) -> bool {
        unsafe {
            match Fl_Table_Row_row_selected(self._inner, row) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn select_row(&mut self, row: i32) -> Result<(), FltkError> {
        unsafe {
            match Fl_Table_Row_select_row(self._inner, row) {
                1 => Ok(()),
                0 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                -1 => Err(FltkError::Internal(FltkErrorKind::TableError)),
                _ => unreachable!(),
            }
        }
    }

    pub fn select_all_rows(&mut self) {
        unsafe { Fl_Table_Row_select_all_rows(self._inner) }
    }
}
