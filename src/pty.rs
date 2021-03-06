
use Error;
use PtyFlags;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Pty(Object<ffi::VtePty>);

    match fn {
        get_type => || ffi::vte_pty_get_type(),
    }
}

impl Pty {

     pub fn new() -> Pty {
        unsafe {
            from_glib_none(ffi::vte_pty_new_sync(ffi::VTE_PTY_DEFAULT, ::std::ptr::null_mut(), ::std::ptr::null_mut()))
        }
    }
    
    //pub fn new_foreign_sync<'a, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>>(fd: i32, cancellable: P) -> Result<Pty, Error> {
    //    unsafe { TODO: call ffi::vte_pty_new_foreign_sync() }
    //}

    //pub fn new_sync<'a, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>>(flags: PtyFlags, cancellable: P) -> Result<Pty, Error> {
    //    unsafe { TODO: call ffi::vte_pty_new_sync() }
    //}
}

/// Trait containing all `Pty` methods.
///
/// # Implementors
///
/// [`Pty`](struct.Pty.html)
pub trait PtyExt {
    /// FIXMEchpe
    fn child_setup(&self);

    /// Cleans up the PTY, specifically any logging performed for the session.
    /// The file descriptor to the PTY master remains open.
    fn close(&self);

    ///
    /// # Returns
    ///
    /// the file descriptor of the PTY master in `self`. The
    ///  file descriptor belongs to `self` and must not be closed
    fn get_fd(&self) -> i32;

    /// Reads the pseudo terminal's window size.
    ///
    /// If getting the window size failed, `error` will be set to a `glib::IOError`.
    /// ## `rows`
    /// a location to store the number of rows, or `None`
    /// ## `columns`
    /// a location to store the number of columns, or `None`

    ///
    /// # Returns
    ///
    /// `true` on success, `false` on failure with `error` filled in
    fn get_size(&self) -> Result<(i32, i32), Error>;

    /// Attempts to resize the pseudo terminal's window size. If successful, the
    /// OS kernel will send `SIGWINCH` to the child process group.
    ///
    /// If setting the window size failed, `error` will be set to a `glib::IOError`.
    /// ## `rows`
    /// the desired number of rows
    /// ## `columns`
    /// the desired number of columns
    ///
    /// # Returns
    ///
    /// `true` on success, `false` on failure with `error` filled in
    fn set_size(&self, rows: i32, columns: i32) -> Result<(), Error>;

    /// Tells the kernel whether the terminal is UTF-8 or not, in case it can make
    /// use of the info. Linux 2.6.5 or so defines IUTF8 to make the line
    /// discipline do multibyte backspace correctly.
    /// ## `utf8`
    /// whether or not the pty is in UTF-8 mode
    ///
    /// # Returns
    ///
    /// `true` on success, `false` on failure with `error` filled in
    fn set_utf8(&self, utf8: bool) -> Result<(), Error>;

    fn get_property_flags(&self) -> PtyFlags;
}

impl<O: IsA<Pty> + IsA<glib::object::Object>> PtyExt for O {
    fn child_setup(&self) {
        unsafe {
            ffi::vte_pty_child_setup(self.to_glib_none().0);
        }
    }

    fn close(&self) {
        unsafe {
            ffi::vte_pty_close(self.to_glib_none().0);
        }
    }

    fn get_fd(&self) -> i32 {
        unsafe {
            ffi::vte_pty_get_fd(self.to_glib_none().0)
        }
    }

    fn get_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut rows = mem::uninitialized();
            let mut columns = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::vte_pty_get_size(self.to_glib_none().0, &mut rows, &mut columns, &mut error);
            if error.is_null() { Ok((rows, columns)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_size(&self, rows: i32, columns: i32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::vte_pty_set_size(self.to_glib_none().0, rows, columns, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_utf8(&self, utf8: bool) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::vte_pty_set_utf8(self.to_glib_none().0, utf8.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_flags(&self) -> PtyFlags {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "flags".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }
}
