use std::ptr;

use ffi;
use gdk;
use glib::translate::*;
use Terminal;

impl Terminal {
    pub fn set_color_background(&self, background: &gdk::RGBA) {
        unsafe {
            ffi::vte_terminal_set_color_background(self.to_glib_none().0, background);
        }
    }

    pub fn set_color_bold(&self, bold: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_bold(self.to_glib_none().0, option_to_ptr(bold));
        }
    }

    pub fn set_color_cursor(&self, cursor_background: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor(self.to_glib_none().0, option_to_ptr(cursor_background));
        }
    }

    #[cfg(feature = "v0_44")]
    pub fn set_color_cursor_foreground(&self, cursor_foreground: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor_foreground(self.to_glib_none().0, option_to_ptr(cursor_foreground));
        }
    }

    pub fn set_color_foreground(&self, foreground: &gdk::RGBA) {
        unsafe {
            ffi::vte_terminal_set_color_foreground(self.to_glib_none().0, foreground);
        }
    }

    pub fn set_color_highlight(&self, highlight_background: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight(self.to_glib_none().0, option_to_ptr(highlight_background));
        }
    }

    pub fn set_color_highlight_foreground(&self, highlight_foreground: Option<&gdk::RGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight_foreground(self.to_glib_none().0, option_to_ptr(highlight_foreground));
        }
    }
}

fn option_to_ptr<T>(value: Option<&T>) -> *const T {
    match value {
        Some(value) => value as *const _,
        None => ptr::null(),
    }
}