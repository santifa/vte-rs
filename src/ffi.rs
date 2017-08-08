// This file was generated by gir (0d8699e+) from gir-files (11e0e6d+)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals)]

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, time_t, FILE};

#[allow(unused_imports)]
use glib_ffi::{gboolean, gconstpointer, gpointer, GType, Volatile};
use gdk_ffi as gdk;
use glib_ffi as glib;
use gio_ffi as gio;
use gtk_ffi as gtk;
use pango_ffi as pango;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum VteCursorBlinkMode {
    System = 0,
    On = 1,
    Off = 2,
}
pub const VTE_CURSOR_BLINK_SYSTEM: VteCursorBlinkMode = VteCursorBlinkMode::System;
pub const VTE_CURSOR_BLINK_ON: VteCursorBlinkMode = VteCursorBlinkMode::On;
pub const VTE_CURSOR_BLINK_OFF: VteCursorBlinkMode = VteCursorBlinkMode::Off;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum VteCursorShape {
    Block = 0,
    Ibeam = 1,
    Underline = 2,
}
pub const VTE_CURSOR_SHAPE_BLOCK: VteCursorShape = VteCursorShape::Block;
pub const VTE_CURSOR_SHAPE_IBEAM: VteCursorShape = VteCursorShape::Ibeam;
pub const VTE_CURSOR_SHAPE_UNDERLINE: VteCursorShape = VteCursorShape::Underline;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum VteEraseBinding {
    Auto = 0,
    AsciiBackspace = 1,
    AsciiDelete = 2,
    DeleteSequence = 3,
    Tty = 4,
}
pub const VTE_ERASE_AUTO: VteEraseBinding = VteEraseBinding::Auto;
pub const VTE_ERASE_ASCII_BACKSPACE: VteEraseBinding = VteEraseBinding::AsciiBackspace;
pub const VTE_ERASE_ASCII_DELETE: VteEraseBinding = VteEraseBinding::AsciiDelete;
pub const VTE_ERASE_DELETE_SEQUENCE: VteEraseBinding = VteEraseBinding::DeleteSequence;
pub const VTE_ERASE_TTY: VteEraseBinding = VteEraseBinding::Tty;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum VtePtyError {
    PtyHelperFailed = 0,
    Pty98Failed = 1,
}
pub const VTE_PTY_ERROR_PTY_HELPER_FAILED: VtePtyError = VtePtyError::PtyHelperFailed;
pub const VTE_PTY_ERROR_PTY98_FAILED: VtePtyError = VtePtyError::Pty98Failed;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum VteRegexError {
    Incompatible = 2147483646,
    NotSupported = 2147483647,
}
pub const VTE_REGEX_ERROR_INCOMPATIBLE: VteRegexError = VteRegexError::Incompatible;
pub const VTE_REGEX_ERROR_NOT_SUPPORTED: VteRegexError = VteRegexError::NotSupported;

pub type WriteFlags = c_int;
pub const VTE_WRITE_DEFAULT: WriteFlags = 0;
pub type VteWriteFlags = WriteFlags;


pub const VTE_MAJOR_VERSION: i32 = 0;
pub const VTE_MICRO_VERSION: i32 = 3;
pub const VTE_MINOR_VERSION: i32 = 48;
pub const VTE_REGEX_FLAGS_DEFAULT: i32 = 1075314688;
pub const VTE_SPAWN_NO_PARENT_ENVV: i32 = 33554432;

bitflags! {
    #[repr(C)]
    pub flags VtePtyFlags: c_uint {
        const VTE_PTY_NO_LASTLOG = 1,
        const VTE_PTY_NO_UTMP = 2,
        const VTE_PTY_NO_WTMP = 4,
        const VTE_PTY_NO_HELPER = 8,
        const VTE_PTY_NO_FALLBACK = 16,
        const VTE_PTY_DEFAULT = 0,
    }
}



pub type VteSelectionFunc = Option<unsafe extern "C" fn(*mut VteTerminal, c_long, c_long, gpointer) -> gboolean>;
pub type VteTerminalSpawnAsyncCallback = Option<unsafe extern "C" fn(*mut VteTerminal, glib::GPid, *mut glib::GError, gpointer)>;

#[repr(C)]
pub struct VteCharAttributes {
    row: c_long,
    column: c_long,
    fore: pango::PangoColor,
    back: pango::PangoColor,
    _truncated_record_marker: c_void,
    //underline: guint: 1,
    //strikethrough: guint: 1,
}

#[repr(C)]
pub struct VtePtyClass(c_void);

#[repr(C)]
pub struct VteRegex(c_void);

#[repr(C)]
pub struct VteTerminalClass {
    pub parent_class: gtk::GtkWidgetClass,
    pub eof: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub child_exited: Option<unsafe extern "C" fn(*mut VteTerminal, c_int)>,
    pub encoding_changed: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub char_size_changed: Option<unsafe extern "C" fn(*mut VteTerminal, c_uint, c_uint)>,
    pub window_title_changed: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub icon_title_changed: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub selection_changed: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub contents_changed: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub cursor_moved: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub commit: Option<unsafe extern "C" fn(*mut VteTerminal, *const c_char, c_uint)>,
    pub deiconify_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub iconify_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub raise_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub lower_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub refresh_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub restore_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub maximize_window: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub resize_window: Option<unsafe extern "C" fn(*mut VteTerminal, c_uint, c_uint)>,
    pub move_window: Option<unsafe extern "C" fn(*mut VteTerminal, c_uint, c_uint)>,
    pub increase_font_size: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub decrease_font_size: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub text_modified: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub text_inserted: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub text_deleted: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub text_scrolled: Option<unsafe extern "C" fn(*mut VteTerminal, c_int)>,
    pub copy_clipboard: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub paste_clipboard: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub bell: Option<unsafe extern "C" fn(*mut VteTerminal)>,
    pub padding: [gpointer; 16],
    pub priv_: *mut VteTerminalClassPrivate,
}

#[repr(C)]
pub struct VteTerminalClassPrivate(c_void);


#[repr(C)]
pub struct VtePty(c_void);
#[repr(C)]
pub struct VteTerminal(c_void);


extern "C" {

    //=========================================================================
    // VtePtyError
    //=========================================================================
    pub fn vte_pty_error_quark() -> glib::GQuark;

    //=========================================================================
    // VteRegexError
    //=========================================================================
    pub fn vte_regex_error_quark() -> glib::GQuark;

    //=========================================================================
    // VteRegex
    //=========================================================================
    pub fn vte_regex_get_type() -> GType;
    pub fn vte_regex_new_for_match(pattern: *const c_char, pattern_length: ssize_t, flags: u32, error: *mut *mut glib::GError) -> *mut VteRegex;
    pub fn vte_regex_new_for_search(pattern: *const c_char, pattern_length: ssize_t, flags: u32, error: *mut *mut glib::GError) -> *mut VteRegex;
    pub fn vte_regex_jit(regex: *mut VteRegex, flags: u32, error: *mut *mut glib::GError) -> gboolean;
    pub fn vte_regex_ref(regex: *mut VteRegex) -> *mut VteRegex;
    pub fn vte_regex_unref(regex: *mut VteRegex) -> *mut VteRegex;

    //=========================================================================
    // VtePty
    //=========================================================================
    pub fn vte_pty_get_type() -> GType;
    pub fn vte_pty_new_foreign_sync(fd: c_int, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut VtePty;
    pub fn vte_pty_new_sync(flags: VtePtyFlags, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut VtePty;
    pub fn vte_pty_child_setup(pty: *mut VtePty);
    pub fn vte_pty_close(pty: *mut VtePty);
    pub fn vte_pty_get_fd(pty: *mut VtePty) -> c_int;
    pub fn vte_pty_get_size(pty: *mut VtePty, rows: *mut c_int, columns: *mut c_int, error: *mut *mut glib::GError) -> gboolean;
    pub fn vte_pty_set_size(pty: *mut VtePty, rows: c_int, columns: c_int, error: *mut *mut glib::GError) -> gboolean;
    pub fn vte_pty_set_utf8(pty: *mut VtePty, utf8: gboolean, error: *mut *mut glib::GError) -> gboolean;
    #[cfg(feature = "v0_48")]
    pub fn vte_pty_spawn_async(pty: *mut VtePty, working_directory: *const c_char, argv: *mut *mut c_char, envv: *mut *mut c_char, spawn_flags: glib::GSpawnFlags, child_setup: glib::GSpawnChildSetupFunc, child_setup_data: gpointer, child_setup_data_destroy: glib::GDestroyNotify, timeout: c_int, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    #[cfg(feature = "v0_48")]
    pub fn vte_pty_spawn_finish(pty: *mut VtePty, result: *mut gio::GAsyncResult, child_pid: *mut glib::GPid, error: *mut *mut glib::GError) -> gboolean;

    //=========================================================================
    // VteTerminal
    //=========================================================================
    pub fn vte_terminal_get_type() -> GType;
    pub fn vte_terminal_new() -> *mut VteTerminal;
    pub fn vte_terminal_copy_clipboard(terminal: *mut VteTerminal);
    pub fn vte_terminal_copy_primary(terminal: *mut VteTerminal);
    #[cfg(feature = "v0_44")]
    pub fn vte_terminal_event_check_gregex_simple(terminal: *mut VteTerminal, event: *mut gdk::GdkEvent, regexes: *mut *mut glib::GRegex, n_regexes: size_t, match_flags: glib::GRegexMatchFlags, matches: *mut *mut c_char) -> gboolean;
    #[cfg(feature = "v0_46")]
    pub fn vte_terminal_event_check_regex_simple(terminal: *mut VteTerminal, event: *mut gdk::GdkEvent, regexes: *mut *mut VteRegex, n_regexes: size_t, match_flags: u32, matches: *mut *mut c_char) -> gboolean;
    pub fn vte_terminal_feed(terminal: *mut VteTerminal, data: *mut u8, length: ssize_t);
    pub fn vte_terminal_feed_child(terminal: *mut VteTerminal, text: *const c_char, length: ssize_t);
    pub fn vte_terminal_feed_child_binary(terminal: *mut VteTerminal, data: *mut u8, length: size_t);
    pub fn vte_terminal_get_allow_bold(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_get_audible_bell(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_get_char_height(terminal: *mut VteTerminal) -> c_long;
    pub fn vte_terminal_get_char_width(terminal: *mut VteTerminal) -> c_long;
    pub fn vte_terminal_get_cjk_ambiguous_width(terminal: *mut VteTerminal) -> c_int;
    pub fn vte_terminal_get_column_count(terminal: *mut VteTerminal) -> c_long;
    pub fn vte_terminal_get_current_directory_uri(terminal: *mut VteTerminal) -> *const c_char;
    pub fn vte_terminal_get_current_file_uri(terminal: *mut VteTerminal) -> *const c_char;
    pub fn vte_terminal_get_cursor_blink_mode(terminal: *mut VteTerminal) -> VteCursorBlinkMode;
    pub fn vte_terminal_get_cursor_position(terminal: *mut VteTerminal, column: *mut c_long, row: *mut c_long);
    pub fn vte_terminal_get_cursor_shape(terminal: *mut VteTerminal) -> VteCursorShape;
    pub fn vte_terminal_get_encoding(terminal: *mut VteTerminal) -> *const c_char;
    pub fn vte_terminal_get_font(terminal: *mut VteTerminal) -> *const pango::PangoFontDescription;
    pub fn vte_terminal_get_font_scale(terminal: *mut VteTerminal) -> c_double;
    pub fn vte_terminal_get_geometry_hints(terminal: *mut VteTerminal, hints: *mut gdk::GdkGeometry, min_rows: c_int, min_columns: c_int);
    pub fn vte_terminal_get_has_selection(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_get_icon_title(terminal: *mut VteTerminal) -> *const c_char;
    pub fn vte_terminal_get_input_enabled(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_get_mouse_autohide(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_get_pty(terminal: *mut VteTerminal) -> *mut VtePty;
    pub fn vte_terminal_get_rewrap_on_resize(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_get_row_count(terminal: *mut VteTerminal) -> c_long;
    pub fn vte_terminal_get_text(terminal: *mut VteTerminal, is_selected: VteSelectionFunc, user_data: gpointer, attributes: *mut glib::GArray) -> *mut c_char;
    pub fn vte_terminal_get_text_include_trailing_spaces(terminal: *mut VteTerminal, is_selected: VteSelectionFunc, user_data: gpointer, attributes: *mut glib::GArray) -> *mut c_char;
    pub fn vte_terminal_get_text_range(terminal: *mut VteTerminal, start_row: c_long, start_col: c_long, end_row: c_long, end_col: c_long, is_selected: VteSelectionFunc, user_data: gpointer, attributes: *mut glib::GArray) -> *mut c_char;
    pub fn vte_terminal_get_window_title(terminal: *mut VteTerminal) -> *const c_char;
    #[cfg(feature = "v0_40")]
    pub fn vte_terminal_get_word_char_exceptions(terminal: *mut VteTerminal) -> *const c_char;
    pub fn vte_terminal_match_add_gregex(terminal: *mut VteTerminal, gregex: *mut glib::GRegex, gflags: glib::GRegexMatchFlags) -> c_int;
    #[cfg(feature = "v0_46")]
    pub fn vte_terminal_match_add_regex(terminal: *mut VteTerminal, regex: *mut VteRegex, flags: u32) -> c_int;
    pub fn vte_terminal_match_check(terminal: *mut VteTerminal, column: c_long, row: c_long, tag: *mut c_int) -> *mut c_char;
    pub fn vte_terminal_match_check_event(terminal: *mut VteTerminal, event: *mut gdk::GdkEvent, tag: *mut c_int) -> *mut c_char;
    pub fn vte_terminal_match_remove(terminal: *mut VteTerminal, tag: c_int);
    pub fn vte_terminal_match_remove_all(terminal: *mut VteTerminal);
    pub fn vte_terminal_match_set_cursor(terminal: *mut VteTerminal, tag: c_int, cursor: *mut gdk::GdkCursor);
    pub fn vte_terminal_match_set_cursor_name(terminal: *mut VteTerminal, tag: c_int, cursor_name: *const c_char);
    pub fn vte_terminal_match_set_cursor_type(terminal: *mut VteTerminal, tag: c_int, cursor_type: gdk::GdkCursorType);
    pub fn vte_terminal_paste_clipboard(terminal: *mut VteTerminal);
    pub fn vte_terminal_paste_primary(terminal: *mut VteTerminal);
    pub fn vte_terminal_pty_new_sync(terminal: *mut VteTerminal, flags: VtePtyFlags, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut VtePty;
    pub fn vte_terminal_reset(terminal: *mut VteTerminal, clear_tabstops: gboolean, clear_history: gboolean);
    pub fn vte_terminal_search_find_next(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_search_find_previous(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_search_get_gregex(terminal: *mut VteTerminal) -> *mut glib::GRegex;
    #[cfg(feature = "v0_46")]
    pub fn vte_terminal_search_get_regex(terminal: *mut VteTerminal) -> *mut VteRegex;
    pub fn vte_terminal_search_get_wrap_around(terminal: *mut VteTerminal) -> gboolean;
    pub fn vte_terminal_search_set_gregex(terminal: *mut VteTerminal, gregex: *mut glib::GRegex, gflags: glib::GRegexMatchFlags);
    #[cfg(feature = "v0_46")]
    pub fn vte_terminal_search_set_regex(terminal: *mut VteTerminal, regex: *mut VteRegex, flags: u32);
    pub fn vte_terminal_search_set_wrap_around(terminal: *mut VteTerminal, wrap_around: gboolean);
    pub fn vte_terminal_select_all(terminal: *mut VteTerminal);
    pub fn vte_terminal_set_allow_bold(terminal: *mut VteTerminal, allow_bold: gboolean);
    pub fn vte_terminal_set_audible_bell(terminal: *mut VteTerminal, is_audible: gboolean);
    pub fn vte_terminal_set_backspace_binding(terminal: *mut VteTerminal, binding: VteEraseBinding);
    pub fn vte_terminal_set_cjk_ambiguous_width(terminal: *mut VteTerminal, width: c_int);
    pub fn vte_terminal_set_color_background(terminal: *mut VteTerminal, background: *const gdk::GdkRGBA);
    pub fn vte_terminal_set_color_bold(terminal: *mut VteTerminal, bold: *const gdk::GdkRGBA);
    pub fn vte_terminal_set_color_cursor(terminal: *mut VteTerminal, cursor_background: *const gdk::GdkRGBA);
    #[cfg(feature = "v0_44")]
    pub fn vte_terminal_set_color_cursor_foreground(terminal: *mut VteTerminal, cursor_foreground: *const gdk::GdkRGBA);
    pub fn vte_terminal_set_color_foreground(terminal: *mut VteTerminal, foreground: *const gdk::GdkRGBA);
    pub fn vte_terminal_set_color_highlight(terminal: *mut VteTerminal, highlight_background: *const gdk::GdkRGBA);
    pub fn vte_terminal_set_color_highlight_foreground(terminal: *mut VteTerminal, highlight_foreground: *const gdk::GdkRGBA);
    pub fn vte_terminal_set_colors(terminal: *mut VteTerminal, foreground: *const gdk::GdkRGBA, background: *const gdk::GdkRGBA, palette: *mut gdk::GdkRGBA, palette_size: size_t);
    pub fn vte_terminal_set_cursor_blink_mode(terminal: *mut VteTerminal, mode: VteCursorBlinkMode);
    pub fn vte_terminal_set_cursor_shape(terminal: *mut VteTerminal, shape: VteCursorShape);
    pub fn vte_terminal_set_default_colors(terminal: *mut VteTerminal);
    pub fn vte_terminal_set_delete_binding(terminal: *mut VteTerminal, binding: VteEraseBinding);
    pub fn vte_terminal_set_encoding(terminal: *mut VteTerminal, codeset: *const c_char, error: *mut *mut glib::GError) -> gboolean;
    pub fn vte_terminal_set_font(terminal: *mut VteTerminal, font_desc: *const pango::PangoFontDescription);
    pub fn vte_terminal_set_font_scale(terminal: *mut VteTerminal, scale: c_double);
    pub fn vte_terminal_set_geometry_hints_for_window(terminal: *mut VteTerminal, window: *mut gtk::GtkWindow);
    pub fn vte_terminal_set_input_enabled(terminal: *mut VteTerminal, enabled: gboolean);
    pub fn vte_terminal_set_mouse_autohide(terminal: *mut VteTerminal, setting: gboolean);
    pub fn vte_terminal_set_pty(terminal: *mut VteTerminal, pty: *mut VtePty);
    pub fn vte_terminal_set_rewrap_on_resize(terminal: *mut VteTerminal, rewrap: gboolean);
    pub fn vte_terminal_set_scroll_on_keystroke(terminal: *mut VteTerminal, scroll: gboolean);
    pub fn vte_terminal_set_scroll_on_output(terminal: *mut VteTerminal, scroll: gboolean);
    pub fn vte_terminal_set_scrollback_lines(terminal: *mut VteTerminal, lines: c_long);
    pub fn vte_terminal_set_size(terminal: *mut VteTerminal, columns: c_long, rows: c_long);
    #[cfg(feature = "v0_40")]
    pub fn vte_terminal_set_word_char_exceptions(terminal: *mut VteTerminal, exceptions: *const c_char);
    #[cfg(feature = "v0_48")]
    pub fn vte_terminal_spawn_async(terminal: *mut VteTerminal, pty_flags: VtePtyFlags, working_directory: *const c_char, argv: *mut *mut c_char, envv: *mut *mut c_char, spawn_flags_: glib::GSpawnFlags, child_setup: glib::GSpawnChildSetupFunc, child_setup_data: gpointer, child_setup_data_destroy: glib::GDestroyNotify, timeout: c_int, cancellable: *mut gio::GCancellable, callback: VteTerminalSpawnAsyncCallback, user_data: gpointer);
    pub fn vte_terminal_spawn_sync(terminal: *mut VteTerminal, pty_flags: VtePtyFlags, working_directory: *const c_char, argv: *mut *mut c_char, envv: *mut *mut c_char, spawn_flags: glib::GSpawnFlags, child_setup: glib::GSpawnChildSetupFunc, child_setup_data: gpointer, child_pid: *mut glib::GPid, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    pub fn vte_terminal_unselect_all(terminal: *mut VteTerminal);
    pub fn vte_terminal_watch_child(terminal: *mut VteTerminal, child_pid: glib::GPid);
    pub fn vte_terminal_write_contents_sync(terminal: *mut VteTerminal, stream: *mut gio::GOutputStream, flags: VteWriteFlags, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;

    //=========================================================================
    // Other functions
    //=========================================================================
    #[cfg(feature = "v0_40")]
    pub fn vte_get_features() -> *const c_char;
    #[cfg(feature = "v0_40")]
    pub fn vte_get_major_version() -> c_uint;
    #[cfg(feature = "v0_40")]
    pub fn vte_get_micro_version() -> c_uint;
    #[cfg(feature = "v0_40")]
    pub fn vte_get_minor_version() -> c_uint;
    pub fn vte_get_user_shell() -> *mut c_char;

}
