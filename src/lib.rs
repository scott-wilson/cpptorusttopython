#[cxx::bridge(namespace = cpptorusttopython)]
mod ffi {
    unsafe extern "C++" {
        include!("cpptorusttopython/cpp/lib.h");

        type Something;

        fn get_something() -> UniquePtr<Something>;
        fn get_something_value(something: &Something) -> i8;
        fn set_something_value(something: Pin<&mut Something>, value: i8);
    }
}

struct CSomething {
    something: std::sync::Arc<std::sync::Mutex<cxx::UniquePtr<ffi::Something>>>,
}

impl CSomething {
    fn new() -> Self {
        let something = unsafe { ffi::get_something() };

        Self {
            something: std::sync::Arc::new(std::sync::Mutex::new(something)),
        }
    }

    fn get_value(&self) -> i8 {
        let something_guard = self.something.as_ref().lock().unwrap();
        let something = something_guard.as_ref().unwrap();

        unsafe { ffi::get_something_value(something) }
    }

    fn set_value(&mut self, value: i8) {
        let mut something_guard = self.something.as_ref().lock().unwrap();
        let something = something_guard.pin_mut();

        unsafe {
            ffi::set_something_value(something, value);
        }
    }
}

unsafe impl Send for ffi::Something {}
unsafe impl Sync for ffi::Something {}

cpython::py_class!(class Something |py| {
    data value: std::cell::RefCell<CSomething>;

    def __new__(_cls) -> cpython::PyResult<Something> {
        Something::create_instance(py, std::cell::RefCell::new(CSomething::new()))
    }

    def get_value(&self) -> cpython::PyResult<i8> {
        Ok(self.value(py).borrow().get_value())
    }

    def set_value(&self, value: i8) -> cpython::PyResult<cpython::PyObject> {
        self.value(py).borrow_mut().set_value(value);

        Ok(py.None())
    }
});

cpython::py_module_initializer!(cpptorusttopython, |py, m| {
    m.add_class::<Something>(py)?;

    Ok(())
});
