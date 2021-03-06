#[cfg(feature="v0_48")]
use std::path::PathBuf;
use std::ptr;

use CursorBlinkMode;
use CursorShape;
use EraseBinding;
use Error;
use pty::Pty;
use ffi;
use gdk_ffi;
use pango_ffi;
use std::ffi::{CStr, CString};
use std::str;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;


glib_wrapper! {
    pub struct Terminal(Object<ffi::VteTerminal>): [
        gtk::Widget => gtk_ffi::GtkWidget,
    ];

    match fn {
        get_type => || ffi::vte_terminal_get_type(),
    }
}

/// Trait containing all `Terminal` methods.
///
/// # Implementors
///
/// [`Terminal`](struct.Terminal.html)
pub trait TerminalExt {
    /// Places the selected text in the terminal in the `GDK_SELECTION_CLIPBOARD`
    /// selection.
    fn copy_clipboard(&self);

    /// Places the selected text in the terminal in the `GDK_SELECTION_PRIMARY`
    /// selection.
    fn copy_primary(&self);

    //fn feed(&self, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 });

    /// Sends a block of UTF-8 text to the child as if it were entered by the user
    /// at the keyboard.
    /// ## `text`
    /// data to send to the child
    /// ## `length`
    /// length of `text` in bytes, or -1 if `text` is NUL-terminated
    fn feed_child(&self, text: &str, length: isize);

    /// Sends a block of binary data to the child.
    /// ## `data`
    /// data to send to the child
    /// ## `length`
    /// length of `data`
    fn feed_child_binary(&self, data: u8, length: usize);

    /// Checks whether or not the terminal will attempt to draw bold text by
    /// repainting text with a one-pixel offset.
    ///
    /// # Returns
    ///
    /// `true` if bolding is enabled, `false` if not
    fn get_allow_bold(&self) -> bool;

    /// Checks whether or not the terminal will beep when the child outputs the
    /// "bl" sequence.
    ///
    /// # Returns
    ///
    /// `true` if audible bell is enabled, `false` if not
    fn get_audible_bell(&self) -> bool;

    ///
    /// # Returns
    ///
    /// the height of a character cell
    fn get_char_height(&self) -> libc::c_long;

    ///
    /// # Returns
    ///
    /// the width of a character cell
    fn get_char_width(&self) -> libc::c_long;

    /// Returns whether ambiguous-width characters are narrow or wide when using
    /// the UTF-8 encoding (`TerminalExt::set_encoding`).
    ///
    /// # Returns
    ///
    /// 1 if ambiguous-width characters are narrow, or 2 if they are wide
    fn get_cjk_ambiguous_width(&self) -> i32;

    ///
    /// # Returns
    ///
    /// the number of columns
    fn get_column_count(&self) -> libc::c_long;

    ///
    /// # Returns
    ///
    /// the URI of the current directory of the
    ///  process running in the terminal, or `None`
    fn get_current_directory_uri(&self) -> Option<String>;

    ///
    /// # Returns
    ///
    /// the URI of the current file the
    ///  process running in the terminal is operating on, or `None` if
    ///  not set
    fn get_current_file_uri(&self) -> Option<String>;

    /// Returns the currently set cursor blink mode.
    ///
    /// # Returns
    ///
    /// cursor blink mode.
    fn get_cursor_blink_mode(&self) -> CursorBlinkMode;

    /// Reads the location of the insertion cursor and returns it. The row
    /// coordinate is absolute.
    /// ## `column`
    /// a location to store the column, or `None`
    /// ## `row`
    /// a location to store the row, or `None`
    fn get_cursor_position(&self) -> (libc::c_long, libc::c_long);

    /// Returns the currently set cursor shape.
    ///
    /// # Returns
    ///
    /// cursor shape.
    fn get_cursor_shape(&self) -> CursorShape;

    /// Determines the name of the encoding in which the terminal expects data to be
    /// encoded.
    ///
    /// # Returns
    ///
    /// the current encoding for the terminal
    fn get_encoding(&self) -> Option<String>;

    /// Queries the terminal for information about the fonts which will be
    /// used to draw text in the terminal. The actual font takes the font scale
    /// into account, this is not reflected in the return value, the unscaled
    /// font is returned.
    ///
    /// # Returns
    ///
    /// a `pango::FontDescription` describing the font the
    /// terminal uses to render text at the default font scale of 1.0.
    fn get_font(&self) -> Option<pango::FontDescription>;

    ///
    /// # Returns
    ///
    /// the terminal's font scale
    fn get_font_scale(&self) -> f64;

    //fn get_geometry_hints(&self, hints: /*Ignored*/gdk::Geometry, min_rows: i32, min_columns: i32);

    /// Checks if the terminal currently contains selected text. Note that this
    /// is different from determining if the terminal is the owner of any
    /// `gtk::Clipboard` items.
    ///
    /// # Returns
    ///
    /// `true` if part of the text in the terminal is selected.
    fn get_has_selection(&self) -> bool;

    ///
    /// # Returns
    ///
    /// the icon title
    fn get_icon_title(&self) -> Option<String>;

    /// Returns whether the terminal allow user input.
    fn get_input_enabled(&self) -> bool;

    /// Determines the value of the terminal's mouse autohide setting. When
    /// autohiding is enabled, the mouse cursor will be hidden when the user presses
    /// a key and shown when the user moves the mouse. This setting can be changed
    /// using `TerminalExt::set_mouse_autohide`.
    ///
    /// # Returns
    ///
    /// `true` if autohiding is enabled, `false` if not
    fn get_mouse_autohide(&self) -> bool;

    /// Returns the `Pty` of `self`.
    ///
    /// # Returns
    ///
    /// a `Pty`, or `None`
    fn get_pty(&self) -> Option<Pty>;

    /// Checks whether or not the terminal will rewrap its contents upon resize.
    ///
    /// # Returns
    ///
    /// `true` if rewrapping is enabled, `false` if not
    fn get_rewrap_on_resize(&self) -> bool;

    ///
    /// # Returns
    ///
    /// the number of rows
    fn get_row_count(&self) -> libc::c_long;

    //fn get_text<'a, P: Into<Option<&'a /*Unimplemented*/SelectionFunc>>>(&self, is_selected: P, user_data: /*Unimplemented*/Fundamental: Pointer, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String>;

    //fn get_text_include_trailing_spaces<'a, P: Into<Option<&'a /*Unimplemented*/SelectionFunc>>>(&self, is_selected: P, user_data: /*Unimplemented*/Fundamental: Pointer, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String>;

    //fn get_text_range<'a, P: Into<Option<&'a /*Unimplemented*/SelectionFunc>>>(&self, start_row: libc::c_long, start_col: libc::c_long, end_row: libc::c_long, end_col: libc::c_long, is_selected: P, user_data: /*Unimplemented*/Fundamental: Pointer, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String>;

    ///
    /// # Returns
    ///
    /// the window title
    fn get_window_title(&self) -> Option<String>;

    /// Returns the set of characters which will be considered parts of a word
    /// when doing word-wise selection, in addition to the default which only
    /// considers alphanumeric characters part of a word.
    ///
    /// If `None`, a built-in set is used.
    ///
    /// Feature: `v0_40`
    ///
    ///
    /// # Returns
    ///
    /// a string, or `None`
    #[cfg(feature = "v0_40")]
    fn get_word_char_exceptions(&self) -> Option<String>;

    //fn match_add_gregex(&self, regex: /*Ignored*/&glib::Regex, flags: /*Ignored*/glib::RegexMatchFlags) -> i32;

    /// Checks if the text in and around the specified position matches any of the
    /// regular expressions previously set using `vte_terminal_match_add`. If a
    /// match exists, the text string is returned and if `tag` is not `None`, the number
    /// associated with the matched regular expression will be stored in `tag`.
    ///
    /// If more than one regular expression has been set with
    /// `vte_terminal_match_add`, then expressions are checked in the order in
    /// which they were added.
    /// ## `column`
    /// the text column
    /// ## `row`
    /// the text row
    /// ## `tag`
    /// a location to store the tag, or `None`
    ///
    /// # Returns
    ///
    /// a newly allocated string which matches one of the previously
    ///  set regular expressions
    fn match_check(&self, column: libc::c_long, row: libc::c_long) -> (String, i32);

    //fn match_check_event(&self, event: /*Ignored*/&mut gdk::Event) -> (String, i32);

    /// Removes the regular expression which is associated with the given `tag` from
    /// the list of expressions which the terminal will highlight when the user
    /// moves the mouse cursor over matching text.
    /// ## `tag`
    /// the tag of the regex to remove
    fn match_remove(&self, tag: i32);

    /// Clears the list of regular expressions the terminal uses to highlight text
    /// when the user moves the mouse cursor.
    fn match_remove_all(&self);

    //fn match_set_cursor<'a, P: Into<Option<&'a /*Ignored*/gdk::Cursor>>>(&self, tag: i32, cursor: P);

    /// Sets which cursor the terminal will use if the pointer is over the pattern
    /// specified by `tag`.
    /// ## `tag`
    /// the tag of the regex which should use the specified cursor
    /// ## `cursor_name`
    /// the name of the cursor
    fn match_set_cursor_name(&self, tag: i32, cursor_name: &str);

    /// Sets which cursor the terminal will use if the pointer is over the pattern
    /// specified by `tag`.
    /// ## `tag`
    /// the tag of the regex which should use the specified cursor
    /// ## `cursor_type`
    /// a `gdk::CursorType`
    fn match_set_cursor_type(&self, tag: i32, cursor_type: gdk::CursorType);

    /// Sends the contents of the `GDK_SELECTION_CLIPBOARD` selection to the
    /// terminal's child. If necessary, the data is converted from UTF-8 to the
    /// terminal's current encoding. It's called on paste menu item, or when
    /// user presses Shift+Insert.
    fn paste_clipboard(&self);

    /// Sends the contents of the `GDK_SELECTION_PRIMARY` selection to the terminal's
    /// child. If necessary, the data is converted from UTF-8 to the terminal's
    /// current encoding. The terminal will call also paste the
    /// `GDK_SELECTION_PRIMARY` selection when the user clicks with the the second
    /// mouse button.
    fn paste_primary(&self);

    //fn pty_new_sync<'a, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>>(&self, flags: PtyFlags, cancellable: P) -> Result<Pty, Error>;

    /// Resets as much of the terminal's internal state as possible, discarding any
    /// unprocessed input data, resetting character attributes, cursor state,
    /// national character set state, status line, terminal modes (insert/delete),
    /// selection state, and encoding.
    /// ## `clear_tabstops`
    /// whether to reset tabstops
    /// ## `clear_history`
    /// whether to empty the terminal's scrollback buffer
    fn reset(&self, clear_tabstops: bool, clear_history: bool);

    /// Searches the next string matching the search regex set with
    /// `TerminalExt::search_set_gregex`.
    ///
    /// # Returns
    ///
    /// `true` if a match was found
    fn search_find_next(&self) -> bool;

    /// Searches the previous string matching the search regex set with
    /// `TerminalExt::search_set_gregex`.
    ///
    /// # Returns
    ///
    /// `true` if a match was found
    fn search_find_previous(&self) -> bool;

    //fn search_get_gregex(&self) -> /*Ignored*/Option<glib::Regex>;

    ///
    /// # Returns
    ///
    /// whether searching will wrap around
    fn search_get_wrap_around(&self) -> bool;

    //fn search_set_gregex<'a, P: Into<Option<&'a /*Ignored*/glib::Regex>>>(&self, regex: P, flags: /*Ignored*/glib::RegexMatchFlags);

    /// Sets whether search should wrap around to the beginning of the
    /// terminal content when reaching its end.
    /// ## `wrap_around`
    /// whether search should wrap
    fn search_set_wrap_around(&self, wrap_around: bool);

    /// Selects all text within the terminal (including the scrollback buffer).
    fn select_all(&self);

    /// Controls whether or not the terminal will attempt to draw bold text,
    /// either by using a bold font variant or by repainting text with a different
    /// offset.
    /// ## `allow_bold`
    /// `true` if the terminal should attempt to draw bold text
    fn set_allow_bold(&self, allow_bold: bool);

    /// Controls whether or not the terminal will beep when the child outputs the
    /// "bl" sequence.
    /// ## `is_audible`
    /// `true` if the terminal should beep
    fn set_audible_bell(&self, is_audible: bool);

    /// Modifies the terminal's backspace key binding, which controls what
    /// string or control sequence the terminal sends to its child when the user
    /// presses the backspace key.
    /// ## `binding`
    /// a `EraseBinding` for the backspace key
    fn set_backspace_binding(&self, binding: EraseBinding);

    /// This setting controls whether ambiguous-width characters are narrow or wide
    /// when using the UTF-8 encoding (`TerminalExt::set_encoding`). In all other encodings,
    /// the width of ambiguous-width characters is fixed.
    /// ## `width`
    /// either 1 (narrow) or 2 (wide)
    fn set_cjk_ambiguous_width(&self, width: i32);

    //fn set_color_background(&self, background: /*Ignored*/&gdk::RGBA);

    //fn set_color_bold<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, bold: P);

    //fn set_color_cursor<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, cursor_background: P);

    //fn set_color_foreground(&self, foreground: /*Ignored*/&gdk::RGBA);

    //fn set_color_highlight<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, highlight_background: P);

    //fn set_color_highlight_foreground<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, highlight_foreground: P);

    //fn set_colors<'a, 'b, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>, Q: Into<Option<&'b /*Ignored*/gdk::RGBA>>>(&self, foreground: P, background: Q, palette: /*Ignored*/&[&gdk::RGBA]);

    /// Sets whether or not the cursor will blink. Using `CursorBlinkMode::System`
    /// will use the `gtk::Settings::gtk-cursor-blink` setting.
    /// ## `mode`
    /// the `CursorBlinkMode` to use
    fn set_cursor_blink_mode(&self, mode: CursorBlinkMode);

    /// Sets the shape of the cursor drawn.
    /// ## `shape`
    /// the `CursorShape` to use
    fn set_cursor_shape(&self, shape: CursorShape);

    /// Reset the terminal palette to reasonable compiled-in default color.
    fn set_default_colors(&self);

    /// Modifies the terminal's delete key binding, which controls what
    /// string or control sequence the terminal sends to its child when the user
    /// presses the delete key.
    /// ## `binding`
    /// a `EraseBinding` for the delete key
    fn set_delete_binding(&self, binding: EraseBinding);

    /// Changes the encoding the terminal will expect data from the child to
    /// be encoded with. For certain terminal types, applications executing in the
    /// terminal can change the encoding. If `codeset` is `None`, it uses "UTF-8".
    /// ## `codeset`
    /// a valid `glib::IConv` target, or `None` to use UTF-8
    ///
    /// # Returns
    ///
    /// `true` if the encoding could be changed to the specified one,
    ///  or `false` with `error` set to `glib::ConvertError::NoConversion`.
    fn set_encoding<'a, P: Into<Option<&'a str>>>(&self, codeset: P) -> Result<(), Error>;

    /// Sets the font used for rendering all text displayed by the terminal,
    /// overriding any fonts set using `gtk::WidgetExt::modify_font`. The terminal
    /// will immediately attempt to load the desired font, retrieve its
    /// metrics, and attempt to resize itself to keep the same number of rows
    /// and columns. The font scale is applied to the specified font.
    /// ## `font_desc`
    /// a `pango::FontDescription` for the desired font, or `None`
    fn set_font<'a, P: Into<Option<&'a pango::FontDescription>>>(&self, font_desc: P);

    /// Sets the terminal's font scale to `scale`.
    /// ## `scale`
    /// the font scale
    fn set_font_scale(&self, scale: f64);

    //fn set_geometry_hints_for_window<P: IsA</*Ignored*/gtk::Window>>(&self, window: &P);

    /// Enables or disables user input. When user input is disabled,
    /// the terminal's child will not receive any key press, or mouse button
    /// press or motion events sent to it.
    /// ## `enabled`
    /// whether to enable user input
    fn set_input_enabled(&self, enabled: bool);

    /// Changes the value of the terminal's mouse autohide setting. When autohiding
    /// is enabled, the mouse cursor will be hidden when the user presses a key and
    /// shown when the user moves the mouse. This setting can be read using
    /// `TerminalExt::get_mouse_autohide`.
    /// ## `setting`
    /// whether the mouse pointer should autohide
    fn set_mouse_autohide(&self, setting: bool);

    /// Sets `pty` as the PTY to use in `self`.
    /// Use `None` to unset the PTY.
    /// ## `pty`
    /// a `Pty`, or `None`
    fn set_pty<'a, P: Into<Option<&'a Pty>>>(&self, pty: P);

    /// Controls whether or not the terminal will rewrap its contents, including
    /// the scrollback history, whenever the terminal's width changes.
    /// ## `rewrap`
    /// `true` if the terminal should rewrap on resize
    fn set_rewrap_on_resize(&self, rewrap: bool);

    /// Controls whether or not the terminal will forcibly scroll to the bottom of
    /// the viewable history when the user presses a key. Modifier keys do not
    /// trigger this behavior.
    /// ## `scroll`
    /// whether the terminal should scroll on keystrokes
    fn set_scroll_on_keystroke(&self, scroll: bool);

    /// Controls whether or not the terminal will forcibly scroll to the bottom of
    /// the viewable history when the new data is received from the child.
    /// ## `scroll`
    /// whether the terminal should scroll on output
    fn set_scroll_on_output(&self, scroll: bool);

    /// Sets the length of the scrollback buffer used by the terminal. The size of
    /// the scrollback buffer will be set to the larger of this value and the number
    /// of visible rows the widget can display, so 0 can safely be used to disable
    /// scrollback.
    ///
    /// A negative value means "infinite scrollback".
    ///
    /// Note that this setting only affects the normal screen buffer.
    /// For terminal types which have an alternate screen buffer, no scrollback is
    /// allowed on the alternate screen buffer.
    /// ## `lines`
    /// the length of the history buffer
    fn set_scrollback_lines(&self, lines: libc::c_long);

    /// Attempts to change the terminal's size in terms of rows and columns. If
    /// the attempt succeeds, the widget will resize itself to the proper size.
    /// ## `columns`
    /// the desired number of columns
    /// ## `rows`
    /// the desired number of rows
    fn set_size(&self, columns: libc::c_long, rows: libc::c_long);

    /// With this function you can provide a set of characters which will
    /// be considered parts of a word when doing word-wise selection, in
    /// addition to the default which only considers alphanumeric characters
    /// part of a word.
    ///
    /// The characters in `exceptions` must be non-alphanumeric, each character
    /// must occur only once, and if `exceptions` contains the character
    /// U+002D HYPHEN-MINUS, it must be at the start of the string.
    ///
    /// Use `None` to reset the set of exception characters to the default.
    ///
    /// Feature: `v0_40`
    ///
    /// ## `exceptions`
    /// a string of ASCII punctuation characters, or `None`
    #[cfg(feature = "v0_40")]
    fn set_word_char_exceptions(&self, exceptions: &str);

    //fn spawn_sync<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b /*Ignored*/glib::SpawnChildSetupFunc>>, R: Into<Option<&'c /*Ignored*/gio::Cancellable>>>(&self, pty_flags: PtyFlags, working_directory: P, argv: &[&std::path::Path], envv: &[&std::path::Path], spawn_flags: /*Ignored*/glib::SpawnFlags, child_setup: Q, child_setup_data: /*Unimplemented*/Fundamental: Pointer, cancellable: R) -> Result</*Ignored*/glib::Pid, Error>;

    /// Clears the current selection.
    fn unselect_all(&self);

    //fn watch_child(&self, child_pid: /*Ignored*/glib::Pid);

    //fn write_contents_sync<'a, P: IsA</*Ignored*/gio::OutputStream>, Q: Into<Option<&'a /*Ignored*/gio::Cancellable>>>(&self, stream: &P, flags: WriteFlags, cancellable: Q) -> Result<(), Error>;

    fn get_property_backspace_binding(&self) -> EraseBinding;

    fn get_property_delete_binding(&self) -> EraseBinding;

   // fn get_property_font_desc(&self) -> Option<pango::FontDescription>;

   // fn set_property_font_desc(&self, font_desc: Option<&pango::FontDescription>);

    fn get_property_pointer_autohide(&self) -> bool;

    fn set_property_pointer_autohide(&self, pointer_autohide: bool);

    fn get_property_scroll_on_keystroke(&self) -> bool;

    fn get_property_scroll_on_output(&self) -> bool;

    fn get_property_scrollback_lines(&self) -> u32;

    fn connect_bell<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_char_size_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64;

    fn connect_child_exited<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;

    fn connect_commit<F: Fn(&Self, &str, u32) + 'static>(&self, f: F) -> u64;

    fn connect_contents_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_copy_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_current_directory_uri_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_current_file_uri_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_cursor_moved<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_decrease_font_size<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_deiconify_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_encoding_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_eof<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_icon_title_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_iconify_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_increase_font_size<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_lower_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_maximize_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_move_window<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64;

    fn connect_paste_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_raise_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_refresh_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_resize_window<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64;

    fn connect_restore_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_text_deleted<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_text_inserted<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_text_modified<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_text_scrolled<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64;

    fn connect_window_title_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}



impl Terminal {
     /// Creates a new terminal widget.
    ///
    /// # Returns
    ///
    /// a new `Terminal` object
    pub fn new() -> Terminal {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::vte_terminal_new())
        }
    }

    pub fn set_color_background(&self, background: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::vte_terminal_set_color_background(self.to_glib_none().0, background);
        }
    }

    pub fn set_color_bold(&self, bold: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_bold(self.to_glib_none().0, option_to_ptr(bold));
        }
    }

    pub fn set_color_cursor(&self, cursor_background: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor(self.to_glib_none().0, option_to_ptr(cursor_background));
        }
    }

    #[cfg(feature = "v0_44")]
    pub fn set_color_cursor_foreground(&self, cursor_foreground: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor_foreground(self.to_glib_none().0, option_to_ptr(cursor_foreground));
        }
    }

    pub fn set_color_foreground(&self, foreground: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::vte_terminal_set_color_foreground(self.to_glib_none().0, foreground);
        }
    }

    pub fn set_color_highlight(&self, highlight_background: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight(self.to_glib_none().0, option_to_ptr(highlight_background));
        }
    }

    pub fn set_color_highlight_foreground(&self, highlight_foreground: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight_foreground(self.to_glib_none().0, option_to_ptr(highlight_foreground));
        }
    }

    /* actually not true, since this feature is deprecated, see libvte */
    #[cfg(feature="v0_48")]
    pub fn spawn_sync(&self, working_directory: Option<PathBuf>, argv: &[&str], envv: &[&str]) {
         let directory = working_directory.as_ref().map(|path_buf| path_buf.as_path());
         unsafe {
             ffi::vte_terminal_spawn_sync(
                 self.to_glib_none().0,
                 ffi::VTE_PTY_DEFAULT,
                 directory.to_glib_none().0,
                 argv.to_glib_none().0,
                 envv.to_glib_none().0,
                 glib_ffi::G_SPAWN_DEFAULT, None, ptr::null_mut(),
                 ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
         }
     }
    
    #[cfg(feature="v0_48")]
    pub fn spawn_async(&self, working_directory: Option<PathBuf>, argv: &[&str], envv: &[&str]) {
        let directory = working_directory.as_ref().map(|path_buf| path_buf.as_path());
        unsafe {
            ffi::vte_terminal_spawn_async(self.to_glib_none().0, ffi::VTE_PTY_DEFAULT,
                directory.to_glib_none().0, argv.to_glib_none().0, envv.to_glib_none().0,
                glib_ffi::G_SPAWN_DEFAULT, None, ptr::null_mut(), None, -1, ptr::null_mut(),
                None, ptr::null_mut());
        }
    }

    pub fn feed(&mut self, data: &str) {
        unsafe { ffi::vte_terminal_feed(self.to_glib_none().0, data.as_bytes().as_ptr() as *mut u8, data.len() as isize) }
    }

    pub fn watch_child(&self, child_pid: i32) {
        unsafe { ffi::vte_terminal_watch_child(self.to_glib_none().0, child_pid) }
    }

    pub fn set_font_size(&mut self, size: i32) {
        unsafe {
            let font_desc = ffi::vte_terminal_get_font(self.to_glib_none().0)
                 as *mut pango_ffi::PangoFontDescription;
            pango_ffi::pango_font_description_set_size(font_desc, size * 1024);
            ffi::vte_terminal_set_font(self.to_glib_none().0, font_desc);
        }
    }

    pub fn get_font_size(&self) -> i32 {
        unsafe {
            let font_desc = ffi::vte_terminal_get_font(self.to_glib_none().0)
                 as *const pango_ffi::PangoFontDescription;
            pango_ffi::pango_font_description_get_size(font_desc) / 1024
        }
    }

    pub fn fork_command(&mut self, working_dir: &str, args: &[&str]) -> Result<i32, String> {
        let working_dir_cstr = CString::new(working_dir).unwrap();
        let args_vec_cstr: Vec<CString> = args.iter().map(|s| CString::new(*s).unwrap()).collect();
        let mut args_vec_ptr: Vec<*const i8> = args_vec_cstr.iter().map(|s| s.as_ptr()).collect();
        args_vec_ptr.push(::std::ptr::null());

        unsafe {
            let mut pid = -1;
            let mut err: *mut glib_ffi::GError = ::std::ptr::null_mut();
            let is_success = ffi::vte_terminal_spawn_sync(
                self.to_glib_none().0, // widget
                ffi::VTE_PTY_DEFAULT, // pty_flags
                working_dir_cstr.as_ptr(), // working_directory
                args_vec_ptr.as_ptr() as *mut *mut i8, // argv
                ::std::ptr::null_mut(), // envv
                glib_ffi::G_SPAWN_SEARCH_PATH, // spawn_flags
                None, // child_setup
                ::std::ptr::null_mut(), // child_setup_data
                &mut pid, // child_pid
		::std::ptr::null_mut(), // cancellable
                &mut err // error
            );
            if is_success == glib_ffi::GTRUE {
                Ok(pid)
            } else {
                let err_slice = str::from_utf8_unchecked(CStr::from_ptr((*err).message).to_bytes());
                let err_str = String::from(err_slice);
                glib_ffi::g_error_free(err);
                Err(err_str)
            }
        }
    }
}

fn option_to_ptr<T>(value: Option<&T>) -> *const T {
    match value {
        Some(value) => value as *const _,
        None => ptr::null(),
    }
}


impl<O: IsA<Terminal> + IsA<glib::object::Object>> TerminalExt for O {
    fn copy_clipboard(&self) {
        unsafe {
            ffi::vte_terminal_copy_clipboard(self.to_glib_none().0);
        }
    }

    fn copy_primary(&self) {
        unsafe {
            ffi::vte_terminal_copy_primary(self.to_glib_none().0);
        }
    }

    //fn feed(&self, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }) {
    //    unsafe { TODO: call ffi::vte_terminal_feed() }
    //}

    fn feed_child(&self, text: &str, length: isize) {
        unsafe {
            ffi::vte_terminal_feed_child(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    fn feed_child_binary(&self, data: u8, length: usize) {
        let ref mut d = data.clone();
        unsafe {
            ffi::vte_terminal_feed_child_binary(self.to_glib_none().0, d, length);
        }
    }

    fn get_allow_bold(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_allow_bold(self.to_glib_none().0))
        }
    }

    fn get_audible_bell(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_audible_bell(self.to_glib_none().0))
        }
    }

    fn get_char_height(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_char_height(self.to_glib_none().0)
        }
    }

    fn get_char_width(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_char_width(self.to_glib_none().0)
        }
    }

    fn get_cjk_ambiguous_width(&self) -> i32 {
        unsafe {
            ffi::vte_terminal_get_cjk_ambiguous_width(self.to_glib_none().0)
        }
    }

    fn get_column_count(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_column_count(self.to_glib_none().0)
        }
    }

    fn get_current_directory_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_current_directory_uri(self.to_glib_none().0))
        }
    }

    fn get_current_file_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_current_file_uri(self.to_glib_none().0))
        }
    }

    fn get_cursor_blink_mode(&self) -> CursorBlinkMode {
        unsafe {
            from_glib(ffi::vte_terminal_get_cursor_blink_mode(self.to_glib_none().0))
        }
    }

    fn get_cursor_position(&self) -> (libc::c_long, libc::c_long) {
        unsafe {
            let mut column = mem::uninitialized();
            let mut row = mem::uninitialized();
            ffi::vte_terminal_get_cursor_position(self.to_glib_none().0, &mut column, &mut row);
            (column, row)
        }
    }

    fn get_cursor_shape(&self) -> CursorShape {
        unsafe {
            from_glib(ffi::vte_terminal_get_cursor_shape(self.to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_encoding(self.to_glib_none().0))
        }
    }

    fn get_font(&self) -> Option<pango::FontDescription> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_font(self.to_glib_none().0))
        }
    }

    fn get_font_scale(&self) -> f64 {
        unsafe {
            ffi::vte_terminal_get_font_scale(self.to_glib_none().0)
        }
    }

    //fn get_geometry_hints(&self, hints: /*Ignored*/gdk::Geometry, min_rows: i32, min_columns: i32) {
    //    unsafe { TODO: call ffi::vte_terminal_get_geometry_hints() }
    //}

    fn get_has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_has_selection(self.to_glib_none().0))
        }
    }

    fn get_icon_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_icon_title(self.to_glib_none().0))
        }
    }

    fn get_input_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_input_enabled(self.to_glib_none().0))
        }
    }

    fn get_mouse_autohide(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_mouse_autohide(self.to_glib_none().0))
        }
    }

    fn get_pty(&self) -> Option<Pty> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_pty(self.to_glib_none().0))
        }
    }

    fn get_rewrap_on_resize(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_get_rewrap_on_resize(self.to_glib_none().0))
        }
    }

    fn get_row_count(&self) -> libc::c_long {
        unsafe {
            ffi::vte_terminal_get_row_count(self.to_glib_none().0)
        }
    }

    //fn get_text<'a, P: Into<Option<&'a /*Unimplemented*/SelectionFunc>>>(&self, is_selected: P, user_data: /*Unimplemented*/Fundamental: Pointer, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String> {
    //    unsafe { TODO: call ffi::vte_terminal_get_text() }
    //}

    //fn get_text_include_trailing_spaces<'a, P: Into<Option<&'a /*Unimplemented*/SelectionFunc>>>(&self, is_selected: P, user_data: /*Unimplemented*/Fundamental: Pointer, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String> {
    //    unsafe { TODO: call ffi::vte_terminal_get_text_include_trailing_spaces() }
    //}

    //fn get_text_range<'a, P: Into<Option<&'a /*Unimplemented*/SelectionFunc>>>(&self, start_row: libc::c_long, start_col: libc::c_long, end_row: libc::c_long, end_col: libc::c_long, is_selected: P, user_data: /*Unimplemented*/Fundamental: Pointer, attributes: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 1, id: 0 }) -> Option<String> {
    //    unsafe { TODO: call ffi::vte_terminal_get_text_range() }
    //}

    fn get_window_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_window_title(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v0_40")]
    fn get_word_char_exceptions(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::vte_terminal_get_word_char_exceptions(self.to_glib_none().0))
        }
    }

    //fn match_add_gregex(&self, regex: /*Ignored*/&glib::Regex, flags: /*Ignored*/glib::RegexMatchFlags) -> i32 {
    //    unsafe { TODO: call ffi::vte_terminal_match_add_gregex() }
    //}

    fn match_check(&self, column: libc::c_long, row: libc::c_long) -> (String, i32) {
        unsafe {
            let mut tag = mem::uninitialized();
            let ret = from_glib_full(ffi::vte_terminal_match_check(self.to_glib_none().0, column, row, &mut tag));
            (ret, tag)
        }
    }

    //fn match_check_event(&self, event: /*Ignored*/&mut gdk::Event) -> (String, i32) {
    //    unsafe { TODO: call ffi::vte_terminal_match_check_event() }
    //}

    fn match_remove(&self, tag: i32) {
        unsafe {
            ffi::vte_terminal_match_remove(self.to_glib_none().0, tag);
        }
    }

    fn match_remove_all(&self) {
        unsafe {
            ffi::vte_terminal_match_remove_all(self.to_glib_none().0);
        }
    }

    //fn match_set_cursor<'a, P: Into<Option<&'a /*Ignored*/gdk::Cursor>>>(&self, tag: i32, cursor: P) {
    //    unsafe { TODO: call ffi::vte_terminal_match_set_cursor() }
    //}

    fn match_set_cursor_name(&self, tag: i32, cursor_name: &str) {
        unsafe {
            ffi::vte_terminal_match_set_cursor_name(self.to_glib_none().0, tag, cursor_name.to_glib_none().0);
        }
    }

    fn match_set_cursor_type(&self, tag: i32, cursor_type: gdk::CursorType) {
        unsafe {
            ffi::vte_terminal_match_set_cursor_type(self.to_glib_none().0, tag, cursor_type.to_glib());
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::vte_terminal_paste_clipboard(self.to_glib_none().0);
        }
    }

    fn paste_primary(&self) {
        unsafe {
            ffi::vte_terminal_paste_primary(self.to_glib_none().0);
        }
    }

    //fn pty_new_sync<'a, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>>(&self, flags: PtyFlags, cancellable: P) -> Result<Pty, Error> {
    //    unsafe { TODO: call ffi::vte_terminal_pty_new_sync() }
    //}

    fn reset(&self, clear_tabstops: bool, clear_history: bool) {
        unsafe {
            ffi::vte_terminal_reset(self.to_glib_none().0, clear_tabstops.to_glib(), clear_history.to_glib());
        }
    }

    fn search_find_next(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_search_find_next(self.to_glib_none().0))
        }
    }

    fn search_find_previous(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_search_find_previous(self.to_glib_none().0))
        }
    }

    //fn search_get_gregex(&self) -> /*Ignored*/Option<glib::Regex> {
    //    unsafe { TODO: call ffi::vte_terminal_search_get_gregex() }
    //}

    fn search_get_wrap_around(&self) -> bool {
        unsafe {
            from_glib(ffi::vte_terminal_search_get_wrap_around(self.to_glib_none().0))
        }
    }

    //fn search_set_gregex<'a, P: Into<Option<&'a /*Ignored*/glib::Regex>>>(&self, regex: P, flags: /*Ignored*/glib::RegexMatchFlags) {
    //    unsafe { TODO: call ffi::vte_terminal_search_set_gregex() }
    //}

    fn search_set_wrap_around(&self, wrap_around: bool) {
        unsafe {
            ffi::vte_terminal_search_set_wrap_around(self.to_glib_none().0, wrap_around.to_glib());
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::vte_terminal_select_all(self.to_glib_none().0);
        }
    }

    fn set_allow_bold(&self, allow_bold: bool) {
        unsafe {
            ffi::vte_terminal_set_allow_bold(self.to_glib_none().0, allow_bold.to_glib());
        }
    }

    fn set_audible_bell(&self, is_audible: bool) {
        unsafe {
            ffi::vte_terminal_set_audible_bell(self.to_glib_none().0, is_audible.to_glib());
        }
    }

    fn set_backspace_binding(&self, binding: EraseBinding) {
        unsafe {
            ffi::vte_terminal_set_backspace_binding(self.to_glib_none().0, binding.to_glib());
        }
    }

    fn set_cjk_ambiguous_width(&self, width: i32) {
        unsafe {
            ffi::vte_terminal_set_cjk_ambiguous_width(self.to_glib_none().0, width);
        }
    }

    //fn set_color_background(&self, background: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_background() }
    //}

    //fn set_color_bold<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, bold: P) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_bold() }
    //}

    //fn set_color_cursor<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, cursor_background: P) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_cursor() }
    //}

    //fn set_color_foreground(&self, foreground: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_foreground() }
    //}

    //fn set_color_highlight<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, highlight_background: P) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_highlight() }
    //}

    //fn set_color_highlight_foreground<'a, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>>(&self, highlight_foreground: P) {
    //    unsafe { TODO: call ffi::vte_terminal_set_color_highlight_foreground() }
    //}

    //fn set_colors<'a, 'b, P: Into<Option<&'a /*Ignored*/gdk::RGBA>>, Q: Into<Option<&'b /*Ignored*/gdk::RGBA>>>(&self, foreground: P, background: Q, palette: /*Ignored*/&[&gdk::RGBA]) {
    //    unsafe { TODO: call ffi::vte_terminal_set_colors() }
    //}

    fn set_cursor_blink_mode(&self, mode: CursorBlinkMode) {
        unsafe {
            ffi::vte_terminal_set_cursor_blink_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_cursor_shape(&self, shape: CursorShape) {
        unsafe {
            ffi::vte_terminal_set_cursor_shape(self.to_glib_none().0, shape.to_glib());
        }
    }

    fn set_default_colors(&self) {
        unsafe {
            ffi::vte_terminal_set_default_colors(self.to_glib_none().0);
        }
    }

    fn set_delete_binding(&self, binding: EraseBinding) {
        unsafe {
            ffi::vte_terminal_set_delete_binding(self.to_glib_none().0, binding.to_glib());
        }
    }

    fn set_encoding<'a, P: Into<Option<&'a str>>>(&self, codeset: P) -> Result<(), Error> {
        let codeset = codeset.into();
        let codeset = codeset.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::vte_terminal_set_encoding(self.to_glib_none().0, codeset.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_font<'a, P: Into<Option<&'a pango::FontDescription>>>(&self, font_desc: P) {
        let font_desc = font_desc.into();
        let font_desc = font_desc.to_glib_none();
        unsafe {
            ffi::vte_terminal_set_font(self.to_glib_none().0, font_desc.0);
        }
    }

    fn set_font_scale(&self, scale: f64) {
        unsafe {
            ffi::vte_terminal_set_font_scale(self.to_glib_none().0, scale);
        }
    }

    //fn set_geometry_hints_for_window<P: IsA</*Ignored*/gtk::Window>>(&self, window: &P) {
    //    unsafe { TODO: call ffi::vte_terminal_set_geometry_hints_for_window() }
    //}

    fn set_input_enabled(&self, enabled: bool) {
        unsafe {
            ffi::vte_terminal_set_input_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_mouse_autohide(&self, setting: bool) {
        unsafe {
            ffi::vte_terminal_set_mouse_autohide(self.to_glib_none().0, setting.to_glib());
        }
    }

    fn set_pty<'a, P: Into<Option<&'a Pty>>>(&self, pty: P) {
        let pty = pty.into();
        let pty = pty.to_glib_none();
        unsafe {
            ffi::vte_terminal_set_pty(self.to_glib_none().0, pty.0);
        }
    }

    fn set_rewrap_on_resize(&self, rewrap: bool) {
        unsafe {
            ffi::vte_terminal_set_rewrap_on_resize(self.to_glib_none().0, rewrap.to_glib());
        }
    }

    fn set_scroll_on_keystroke(&self, scroll: bool) {
        unsafe {
            ffi::vte_terminal_set_scroll_on_keystroke(self.to_glib_none().0, scroll.to_glib());
        }
    }

    fn set_scroll_on_output(&self, scroll: bool) {
        unsafe {
            ffi::vte_terminal_set_scroll_on_output(self.to_glib_none().0, scroll.to_glib());
        }
    }

    fn set_scrollback_lines(&self, lines: libc::c_long) {
        unsafe {
            ffi::vte_terminal_set_scrollback_lines(self.to_glib_none().0, lines);
        }
    }

    fn set_size(&self, columns: libc::c_long, rows: libc::c_long) {
        unsafe {
            ffi::vte_terminal_set_size(self.to_glib_none().0, columns, rows);
        }
    }

    #[cfg(feature = "v0_40")]
    fn set_word_char_exceptions(&self, exceptions: &str) {
        unsafe {
            ffi::vte_terminal_set_word_char_exceptions(self.to_glib_none().0, exceptions.to_glib_none().0);
        }
    }

    //fn spawn_sync<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b /*Ignored*/glib::SpawnChildSetupFunc>>, R: Into<Option<&'c /*Ignored*/gio::Cancellable>>>(&self, pty_flags: PtyFlags, working_directory: P, argv: &[&std::path::Path], envv: &[&std::path::Path], spawn_flags: /*Ignored*/glib::SpawnFlags, child_setup: Q, child_setup_data: /*Unimplemented*/Fundamental: Pointer, cancellable: R) -> Result</*Ignored*/glib::Pid, Error> {
    //    unsafe { TODO: call ffi::vte_terminal_spawn_sync() }
    //}

    fn unselect_all(&self) {
        unsafe {
            ffi::vte_terminal_unselect_all(self.to_glib_none().0);
        }
    }

    //fn watch_child(&self, child_pid: /*Ignored*/glib::Pid) {
    //    unsafe { TODO: call ffi::vte_terminal_watch_child() }
    //}

    //fn write_contents_sync<'a, P: IsA</*Ignored*/gio::OutputStream>, Q: Into<Option<&'a /*Ignored*/gio::Cancellable>>>(&self, stream: &P, flags: WriteFlags, cancellable: Q) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::vte_terminal_write_contents_sync() }
    //}

    fn get_property_backspace_binding(&self) -> EraseBinding {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "backspace-binding".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn get_property_delete_binding(&self) -> EraseBinding {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "delete-binding".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    // fn get_property_font_desc(&self) -> Option<pango::FontDescription> {
    //     //let mut value = Value::from(None::<&pango::FontDescription>);
    //     let font_desc = self.into();
    //     let font_desc = self.to_glib_none();
       
    //     unsafe {
    //         gobject_ffi::g_object_get_property(self.to_glib_none().0, "font-desc".to_glib_none().0, font_desc.0);
    //     }
    //     font_desc.get()
    // }

    // fn set_property_font_desc(&self, font_desc: Option<&pango::FontDescription>) {
    //     unsafe {
    //         gobject_ffi::g_object_set_property(self.to_glib_none().0, "font-desc".to_glib_none().0, font_desc.to_glib_none ().0);
    //     }
    // }

    fn get_property_pointer_autohide(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pointer-autohide".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_pointer_autohide(&self, pointer_autohide: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pointer-autohide".to_glib_none().0, Value::from(&pointer_autohide).to_glib_none().0);
        }
    }

    fn get_property_scroll_on_keystroke(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scroll-on-keystroke".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_scroll_on_output(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scroll-on-output".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_scrollback_lines(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "scrollback-lines".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn connect_bell<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "bell",
                transmute(bell_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_char_size_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "char-size-changed",
                transmute(char_size_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_child_exited<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "child-exited",
                transmute(child_exited_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_commit<F: Fn(&Self, &str, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "commit",
                transmute(commit_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_contents_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "contents-changed",
                transmute(contents_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_copy_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "copy-clipboard",
                transmute(copy_clipboard_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_current_directory_uri_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "current-directory-uri-changed",
                transmute(current_directory_uri_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_current_file_uri_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "current-file-uri-changed",
                transmute(current_file_uri_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_cursor_moved<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cursor-moved",
                transmute(cursor_moved_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_decrease_font_size<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "decrease-font-size",
                transmute(decrease_font_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_deiconify_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deiconify-window",
                transmute(deiconify_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_encoding_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "encoding-changed",
                transmute(encoding_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_eof<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "eof",
                transmute(eof_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_icon_title_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "icon-title-changed",
                transmute(icon_title_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_iconify_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "iconify-window",
                transmute(iconify_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_increase_font_size<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "increase-font-size",
                transmute(increase_font_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_lower_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "lower-window",
                transmute(lower_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_maximize_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "maximize-window",
                transmute(maximize_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_window<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-window",
                transmute(move_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_paste_clipboard<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "paste-clipboard",
                transmute(paste_clipboard_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_raise_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "raise-window",
                transmute(raise_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_refresh_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "refresh-window",
                transmute(refresh_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_resize_window<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "resize-window",
                transmute(resize_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_restore_window<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "restore-window",
                transmute(restore_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_deleted<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-deleted",
                transmute(text_deleted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_inserted<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-inserted",
                transmute(text_inserted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_modified<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-modified",
                transmute(text_modified_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_text_scrolled<F: Fn(&Self, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "text-scrolled",
                transmute(text_scrolled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_window_title_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-title-changed",
                transmute(window_title_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn bell_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn char_size_changed_trampoline<P>(this: *mut ffi::VteTerminal, width: libc::c_uint, height: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P, u32, u32) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked(), width, height)
}

unsafe extern "C" fn child_exited_trampoline<P>(this: *mut ffi::VteTerminal, status: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked(), status)
}

unsafe extern "C" fn commit_trampoline<P>(this: *mut ffi::VteTerminal, text: *mut libc::c_char, size: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P, &str, u32) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(text), size)
}

unsafe extern "C" fn contents_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn copy_clipboard_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn current_directory_uri_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn current_file_uri_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn cursor_moved_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn decrease_font_size_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn deiconify_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn encoding_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn eof_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn icon_title_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn iconify_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn increase_font_size_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn lower_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn maximize_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn move_window_trampoline<P>(this: *mut ffi::VteTerminal, x: libc::c_uint, y: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P, u32, u32) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked(), x, y)
}

unsafe extern "C" fn paste_clipboard_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn raise_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn refresh_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn resize_window_trampoline<P>(this: *mut ffi::VteTerminal, width: libc::c_uint, height: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P, u32, u32) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked(), width, height)
}

unsafe extern "C" fn restore_window_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn text_deleted_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn text_inserted_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn text_modified_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn text_scrolled_trampoline<P>(this: *mut ffi::VteTerminal, delta: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked(), delta)
}

unsafe extern "C" fn window_title_changed_trampoline<P>(this: *mut ffi::VteTerminal, f: glib_ffi::gpointer)
where P: IsA<Terminal> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&Terminal::from_glib_none(this).downcast_unchecked())
}
