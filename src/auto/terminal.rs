// This file was generated by gir (e48471c+) from gir-files (???)
// DO NOT EDIT

use CursorBlinkMode;
use CursorShape;
use EraseBinding;
use Error;
use Pty;
use ffi;
use gdk;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Terminal(Object<ffi::VteTerminal>);

    match fn {
        get_type => || ffi::vte_terminal_get_type(),
    }
}

impl Terminal {
    pub fn new() -> Terminal {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::vte_terminal_new())
        }
    }

    pub fn copy_clipboard(&self) {
        unsafe {
            ffi::vte_terminal_copy_clipboard(self.to_glib_none().0);
        }
    }

    pub fn copy_primary(&self) {
        unsafe {
            ffi::vte_terminal_copy_primary(self.to_glib_none().0);
        }
    }

    //pub fn event_check_gregex_simple(&self, event: /*Ignored*/&mut gdk::Event, regexes: /*Ignored*/&[&glib::Regex], match_flags: /*Ignored*/glib::RegexMatchFlags, matches: /*Unimplemented*/Vec<String>) -> Option</*Unimplemented*/Fundamental: Size> {
    //    unsafe { TODO: call ffi::vte_terminal_event_check_gregex_simple() }
    //}

    //pub fn feed(&self, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, length: /*Unimplemented*/Fundamental: SSize) {
    //    unsafe { TODO: call ffi::vte_terminal_feed() }
    //}

    //pub fn feed_child(&self, text: &str, length: /*Unimplemented*/Fundamental: SSize) {
    //    unsafe { TODO: call ffi::vte_terminal_feed_child() }
    //}

    //pub fn feed_child_binary(&self, data: u8, length: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::vte_terminal_feed_child_binary() }
    //}

    pub fn get_allow_bold(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_allow_bold(self.to_glib_none().0))
        }
    }

    pub fn get_audible_bell(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_audible_bell(self.to_glib_none().0))
        }
    }

    pub fn get_char_height(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_char_height(self.to_glib_none().0)
        }
    }

    pub fn get_char_width(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_char_width(self.to_glib_none().0)
        }
    }

    pub fn get_cjk_ambiguous_width(&self) -> i32 {
        unsafe {
            ffi::vte_terminal_get_cjk_ambiguous_width(self.to_glib_none().0)
        }
    }

    pub fn get_column_count(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_column_count(self.to_glib_none().0)
        }
    }

    pub fn get_current_directory_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_current_directory_uri(self.to_glib_none().0))
        }
    }

    pub fn get_current_file_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_current_file_uri(self.to_glib_none().0))
        }
    }

    pub fn get_cursor_blink_mode(&self) -> CursorBlinkMode {
        unsafe {
            from_glib(ffi::vte_terminal_get_cursor_blink_mode(self.to_glib_none().0))
        }
    }

    pub fn get_cursor_position(&self) -> (libc::c_long, libc::c_long) {
        unsafe {
            let mut column = mem::uninitialized();
            let mut row = mem::uninitialized();
            ffi::vte_terminal_get_cursor_position(self.to_glib_none().0, &mut column, &mut row);
            (column, row)
        }
    }

    pub fn get_cursor_shape(&self) -> CursorShape {
        unsafe {
            from_glib(ffi::vte_terminal_get_cursor_shape(self.to_glib_none().0))
        }
    }

    pub fn get_encoding(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_encoding(self.to_glib_none().0))
        }
    }

    pub fn get_font(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_font(self.to_glib_none().0))
        }
    }

    pub fn get_font_scale(&self) -> f64 {
        unsafe {
            ffi::vte_terminal_get_font_scale(self.to_glib_none().0)
        }
    }

    //pub fn get_geometry_hints(&self, hints: /*Ignored*/gdk::Geometry, min_rows: i32, min_columns: i32) {
    //    unsafe { TODO: call ffi::vte_terminal_get_geometry_hints() }
    //}

    pub fn get_has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_has_selection(self.to_glib_none().0))
        }
    }

    pub fn get_icon_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_icon_title(self.to_glib_none().0))
        }
    }

    pub fn get_input_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_input_enabled(self.to_glib_none().0))
        }
    }

    pub fn get_mouse_autohide(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_mouse_autohide(self.to_glib_none().0))
        }
    }

    pub fn get_pty(&self) -> Option<Pty> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_pty(self.to_glib_none().0))
        }
    }

    pub fn get_rewrap_on_resize(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_rewrap_on_resize(self.to_glib_none().0))
        }
    }

    pub fn get_row_count(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_row_count(self.to_glib_none().0)
        }
    }

    //pub fn get_text(&self, is_selected: /*Unknown conversion*//*Unimplemented*/SelectionFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String> {
    //    unsafe { TODO: call ffi::vte_terminal_get_text() }
    //}

    //pub fn get_text_include_trailing_spaces(&self, is_selected: /*Unknown conversion*//*Unimplemented*/SelectionFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String> {
    //    unsafe { TODO: call ffi::vte_terminal_get_text_include_trailing_spaces() }
    //}

    //pub fn get_text_range(&self, start_row: libc::c_long, start_col: libc::c_long, end_row: libc::c_long, end_col: libc::c_long, is_selected: /*Unknown conversion*//*Unimplemented*/SelectionFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String> {
    //    unsafe { TODO: call ffi::vte_terminal_get_text_range() }
    //}

    pub fn get_window_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_window_title(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v0_40")]
    pub fn get_word_char_exceptions(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_word_char_exceptions(self.to_glib_none().0))
        }
    }

    //pub fn match_add_gregex(&self, gregex: /*Ignored*/&glib::Regex, gflags: /*Ignored*/glib::RegexMatchFlags) -> i32 {
    //    unsafe { TODO: call ffi::vte_terminal_match_add_gregex() }
    //}

    pub fn match_check(&self, column: libc::c_long, row: libc::c_long) -> (String, i32) {
        unsafe {
            let mut tag = mem::uninitialized();
            let ret = from_glib_full(ffi::vte_terminal_match_check(self.to_glib_none().0, column, row, &mut tag));
            (ret, tag)
        }
    }

    //pub fn match_check_event(&self, event: /*Ignored*/&mut gdk::Event) -> (String, i32) {
    //    unsafe { TODO: call ffi::vte_terminal_match_check_event() }
    //}

    pub fn match_remove(&self, tag: i32) {
        unsafe {
            ffi::vte_terminal_match_remove(self.to_glib_none().0, tag);
        }
    }

    pub fn match_remove_all(&self) {
        unsafe {
            ffi::vte_terminal_match_remove_all(self.to_glib_none().0);
        }
    }

    //pub fn match_set_cursor(&self, tag: i32, cursor: /*Ignored*/Option<&gdk::Cursor>) {
    //    unsafe { TODO: call ffi::vte_terminal_match_set_cursor() }
    //}

    pub fn match_set_cursor_name(&self, tag: i32, cursor_name: &str) {
        unsafe {
            ffi::vte_terminal_match_set_cursor_name(self.to_glib_none().0, tag, cursor_name.to_glib_none().0);
        }
    }

    pub fn match_set_cursor_type(&self, tag: i32, cursor_type: gdk::CursorType) {
        unsafe {
            ffi::vte_terminal_match_set_cursor_type(self.to_glib_none().0, tag, cursor_type.to_glib());
        }
    }

    pub fn paste_clipboard(&self) {
        unsafe {
            ffi::vte_terminal_paste_clipboard(self.to_glib_none().0);
        }
    }

    pub fn paste_primary(&self) {
        unsafe {
            ffi::vte_terminal_paste_primary(self.to_glib_none().0);
        }
    }

    //pub fn pty_new_sync(&self, flags: PtyFlags, cancellable: /*Ignored*/Option<&gio::Cancellable>) -> Result<Pty, Error> {
    //    unsafe { TODO: call ffi::vte_terminal_pty_new_sync() }
    //}

    pub fn reset(&self, clear_tabstops: bool, clear_history: bool) {
        unsafe {
            ffi::vte_terminal_reset(self.to_glib_none().0, clear_tabstops.to_glib(), clear_history.to_glib());
        }
    }

    pub fn search_find_next(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_search_find_next(self.to_glib_none().0))
        }
    }

    pub fn search_find_previous(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_search_find_previous(self.to_glib_none().0))
        }
    }

    //pub fn search_get_gregex(&self) -> /*Ignored*/Option<glib::Regex> {
    //    unsafe { TODO: call ffi::vte_terminal_search_get_gregex() }
    //}

    pub fn search_get_wrap_around(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_search_get_wrap_around(self.to_glib_none().0))
        }
    }

    //pub fn search_set_gregex(&self, gregex: /*Ignored*/Option<&glib::Regex>, gflags: /*Ignored*/glib::RegexMatchFlags) {
    //    unsafe { TODO: call ffi::vte_terminal_search_set_gregex() }
    //}

    pub fn search_set_wrap_around(&self, wrap_around: bool) {
        unsafe {
            ffi::vte_terminal_search_set_wrap_around(self.to_glib_none().0, wrap_around.to_glib());
        }
    }

    pub fn select_all(&self) {
        unsafe {
            ffi::vte_terminal_select_all(self.to_glib_none().0);
        }
    }

    pub fn set_allow_bold(&self, allow_bold: bool) {
        unsafe {
            ffi::vte_terminal_set_allow_bold(self.to_glib_none().0, allow_bold.to_glib());
        }
    }

    pub fn set_audible_bell(&self, is_audible: bool) {
        unsafe {
            ffi::vte_terminal_set_audible_bell(self.to_glib_none().0, is_audible.to_glib());
        }
    }

    pub fn set_backspace_binding(&self, binding: EraseBinding) {
        unsafe {
            ffi::vte_terminal_set_backspace_binding(self.to_glib_none().0, binding.to_glib());
        }
    }

    pub fn set_cjk_ambiguous_width(&self, width: i32) {
        unsafe {
            ffi::vte_terminal_set_cjk_ambiguous_width(self.to_glib_none().0, width);
        }
    }

    //pub fn set_color_background(&self, background: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_background() }
    //}

    //pub fn set_color_bold(&self, bold: /*Ignored*/Option<&gdk::RGBA>) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_bold() }
    //}

    //pub fn set_color_cursor(&self, cursor_background: /*Ignored*/Option<&gdk::RGBA>) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_cursor() }
    //}

    //#[cfg(feature = "v0_44")]
    //pub fn set_color_cursor_foreground(&self, cursor_foreground: /*Ignored*/Option<&gdk::RGBA>) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_cursor_foreground() }
    //}

    //pub fn set_color_foreground(&self, foreground: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_foreground() }
    //}

    //pub fn set_color_highlight(&self, highlight_background: /*Ignored*/Option<&gdk::RGBA>) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_highlight() }
    //}

    //pub fn set_color_highlight_foreground(&self, highlight_foreground: /*Ignored*/Option<&gdk::RGBA>) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_highlight_foreground() }
    //}

    //pub fn set_colors(&self, foreground: /*Ignored*/Option<&gdk::RGBA>, background: /*Ignored*/Option<&gdk::RGBA>, palette: /*Ignored*/&[&gdk::RGBA], palette_size: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::vte_terminal_set_colors() }
    //}

    pub fn set_cursor_blink_mode(&self, mode: CursorBlinkMode) {
        unsafe {
            ffi::vte_terminal_set_cursor_blink_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    pub fn set_cursor_shape(&self, shape: CursorShape) {
        unsafe {
            ffi::vte_terminal_set_cursor_shape(self.to_glib_none().0, shape.to_glib());
        }
    }

    pub fn set_default_colors(&self) {
        unsafe {
            ffi::vte_terminal_set_default_colors(self.to_glib_none().0);
        }
    }

    pub fn set_delete_binding(&self, binding: EraseBinding) {
        unsafe {
            ffi::vte_terminal_set_delete_binding(self.to_glib_none().0, binding.to_glib());
        }
    }

    pub fn set_encoding(&self, codeset: Option<&str>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::vte_terminal_set_encoding(self.to_glib_none().0, codeset.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_font(&self, font_desc: Option<&pango::FontDescription>) {
        unsafe {
            ffi::vte_terminal_set_font(self.to_glib_none().0, font_desc.to_glib_none().0);
        }
    }

    pub fn set_font_scale(&self, scale: f64) {
        unsafe {
            ffi::vte_terminal_set_font_scale(self.to_glib_none().0, scale);
        }
    }

    //pub fn set_geometry_hints_for_window<T: IsA</*Ignored*/gtk::Window>>(&self, window: &T) {
    //    unsafe { TODO: call ffi::vte_terminal_set_geometry_hints_for_window() }
    //}

    pub fn set_input_enabled(&self, enabled: bool) {
        unsafe {
            ffi::vte_terminal_set_input_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    pub fn set_mouse_autohide(&self, setting: bool) {
        unsafe {
            ffi::vte_terminal_set_mouse_autohide(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn set_pty(&self, pty: Option<&Pty>) {
        unsafe {
            ffi::vte_terminal_set_pty(self.to_glib_none().0, pty.to_glib_none().0);
        }
    }

    pub fn set_rewrap_on_resize(&self, rewrap: bool) {
        unsafe {
            ffi::vte_terminal_set_rewrap_on_resize(self.to_glib_none().0, rewrap.to_glib());
        }
    }

    pub fn set_scroll_on_keystroke(&self, scroll: bool) {
        unsafe {
            ffi::vte_terminal_set_scroll_on_keystroke(self.to_glib_none().0, scroll.to_glib());
        }
    }

    pub fn set_scroll_on_output(&self, scroll: bool) {
        unsafe {
            ffi::vte_terminal_set_scroll_on_output(self.to_glib_none().0, scroll.to_glib());
        }
    }

    pub fn set_scrollback_lines(&self, lines: libc::c_long) {
        unsafe {
            ffi::vte_terminal_set_scrollback_lines(self.to_glib_none().0, lines);
        }
    }

    pub fn set_size(&self, columns: libc::c_long, rows: libc::c_long) {
        unsafe {
            ffi::vte_terminal_set_size(self.to_glib_none().0, columns, rows);
        }
    }

    #[cfg(feature = "v0_40")]
    pub fn set_word_char_exceptions(&self, exceptions: &str) {
        unsafe {
            ffi::vte_terminal_set_word_char_exceptions(self.to_glib_none().0, exceptions.to_glib_none().0);
        }
    }

    //pub fn spawn_sync(&self, pty_flags: PtyFlags, working_directory: Option<&str>, argv: &[&std::path::Path], envv: &[&std::path::Path], spawn_flags: /*Ignored*/glib::SpawnFlags, child_setup: /*Unknown conversion*//*Unimplemented*/SpawnChildSetupFunc, child_setup_data: /*Unimplemented*/Option<Fundamental: Pointer>, cancellable: /*Ignored*/Option<&gio::Cancellable>) -> Result</*Ignored*/glib::Pid, Error> {
    //    unsafe { TODO: call ffi::vte_terminal_spawn_sync() }
    //}

    pub fn unselect_all(&self) {
        unsafe {
            ffi::vte_terminal_unselect_all(self.to_glib_none().0);
        }
    }

    //pub fn watch_child(&self, child_pid: /*Ignored*/glib::Pid) {
    //    unsafe { TODO: call ffi::vte_terminal_watch_child() }
    //}

    //pub fn write_contents_sync<T: IsA</*Ignored*/gio::OutputStream>>(&self, stream: &T, flags: WriteFlags, cancellable: /*Ignored*/Option<&gio::Cancellable>) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::vte_terminal_write_contents_sync() }
    //}

    pub fn connect_bell<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "bell",
                transmute(bell_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_char_size_changed<F: Fn(&Terminal, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "char-size-changed",
                transmute(char_size_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_child_exited<F: Fn(&Terminal, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "child-exited",
                transmute(child_exited_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_commit<F: Fn(&Terminal, &str, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal, &str, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "commit",
                transmute(commit_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_contents_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "contents-changed",
                transmute(contents_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_copy_clipboard<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "copy-clipboard",
                transmute(copy_clipboard_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_current_directory_uri_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "current-directory-uri-changed",
                transmute(current_directory_uri_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_current_file_uri_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "current-file-uri-changed",
                transmute(current_file_uri_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_cursor_moved<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cursor-moved",
                transmute(cursor_moved_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_decrease_font_size<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "decrease-font-size",
                transmute(decrease_font_size_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_deiconify_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deiconify-window",
                transmute(deiconify_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_encoding_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "encoding-changed",
                transmute(encoding_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_eof<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "eof",
                transmute(eof_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_icon_title_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "icon-title-changed",
                transmute(icon_title_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_iconify_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "iconify-window",
                transmute(iconify_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_increase_font_size<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "increase-font-size",
                transmute(increase_font_size_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_lower_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "lower-window",
                transmute(lower_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_maximize_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "maximize-window",
                transmute(maximize_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_move_window<F: Fn(&Terminal, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-window",
                transmute(move_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_paste_clipboard<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "paste-clipboard",
                transmute(paste_clipboard_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_raise_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "raise-window",
                transmute(raise_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_refresh_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "refresh-window",
                transmute(refresh_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_resize_window<F: Fn(&Terminal, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "resize-window",
                transmute(resize_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_restore_window<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "restore-window",
                transmute(restore_window_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_selection_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_text_deleted<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-deleted",
                transmute(text_deleted_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_text_inserted<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-inserted",
                transmute(text_inserted_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_text_modified<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-modified",
                transmute(text_modified_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_text_scrolled<F: Fn(&Terminal, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-scrolled",
                transmute(text_scrolled_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_window_title_changed<F: Fn(&Terminal) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Terminal) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-title-changed",
                transmute(window_title_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn bell_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn char_size_changed_trampoline(this: *mut ffi::VteTerminal, width: libc::c_uint, height: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal, u32, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), width, height)
}

unsafe extern "C" fn child_exited_trampoline(this: *mut ffi::VteTerminal, status: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), status)
}

unsafe extern "C" fn commit_trampoline(this: *mut ffi::VteTerminal, text: *mut libc::c_char, size: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal, &str, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(text), size)
}

unsafe extern "C" fn contents_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn copy_clipboard_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn current_directory_uri_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn current_file_uri_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn cursor_moved_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn decrease_font_size_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn deiconify_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn encoding_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn eof_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn icon_title_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn iconify_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn increase_font_size_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn lower_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn maximize_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn move_window_trampoline(this: *mut ffi::VteTerminal, x: libc::c_uint, y: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal, u32, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), x, y)
}

unsafe extern "C" fn paste_clipboard_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn raise_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn refresh_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn resize_window_trampoline(this: *mut ffi::VteTerminal, width: libc::c_uint, height: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal, u32, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), width, height)
}

unsafe extern "C" fn restore_window_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn selection_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn text_deleted_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn text_inserted_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn text_modified_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn text_scrolled_trampoline(this: *mut ffi::VteTerminal, delta: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), delta)
}

unsafe extern "C" fn window_title_changed_trampoline(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Terminal) + 'static> = transmute(f);
    f(&from_glib_none(this))
}