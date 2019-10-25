pub struct Window {
    _window: *mut fltk_sys::window::Fl_Window,
}

impl Window {
    pub fn new(x: i32, y: i32, width: i32, height: i32, title: &'static str) -> Window {
        unsafe {
            Window {
                _window: fltk_sys::window::Fl_Window_new(x, y, width, height, title.as_ptr() as *const i8),
            }
        }
    }
    pub fn begin(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_begin(self._window) }
    }
    pub fn end(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_end(self._window) }
    }
    pub fn show(&mut self) {
        unsafe { fltk_sys::window::Fl_Window_show(self._window) }   
    }
}