use core::panic;
use std::alloc;
use std::fmt;
use std::ptr;

pub struct Vector<T> {
    data: *mut T,
    fixed: bool,
    capacity: usize,
    size: usize,
    scale_fn: Box<dyn Fn(usize) -> usize>,
}

impl<T> Vector<T> {
    pub fn new(capacity: usize, fixed: bool, scale: Box<dyn Fn(usize) -> usize>) -> Self {
        assert!(capacity > 0);
        let layout = alloc::Layout::array::<T>(capacity).expect("Invalid layout.");
        let data = unsafe { alloc::alloc(layout) as *mut T };

        if data.is_null() {
            panic!("Failed allocating memory!");
        }

        Self {
            data,
            fixed,
            capacity,
            size: 0,
            scale_fn: scale,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.size >= self.capacity {
            if self.fixed {
                panic!("Memory exceeded!");
            }
            unsafe {
                let new_cap = (self.scale_fn)(self.capacity);
                assert!(new_cap > self.capacity, "Scale must increase capacity.");

                let layout = alloc::Layout::array::<T>(self.capacity).expect("Invalid layout.");
                let new_layout = alloc::Layout::array::<T>(new_cap).expect("Invalid layout.");

                let ptr = alloc::realloc(self.data as *mut u8, layout, new_layout.size());

                self.capacity = new_cap;

                if ptr.is_null() {
                    panic!("Failed reallocating memory.");
                }

                self.data = ptr as *mut T;
            }
        }
        unsafe {
            ptr::write(self.data.add(self.size), value);
        }

        self.size += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        unsafe { Some(&*self.data.add(index)) }
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        unsafe {
            ptr::write(self.data.add(index), value);
        }
    }

    pub fn drop_last(&mut self) {
        if self.is_empty() {
            return;
        }

        self.size -= 1;

        unsafe {
            ptr::drop_in_place(self.data.add(self.size));
        }
    }

    pub fn fit_in(&mut self) {
        if self.fixed || self.size >= self.capacity || self.size == 0 {
            return;
        }

        unsafe {
            let layout = alloc::Layout::array::<T>(self.capacity).expect("Invalid layout");
            let new_layout = alloc::Layout::array::<T>(self.size).expect("Invalid layout");

            let ptr = alloc::realloc(self.data as *mut u8, layout, new_layout.size());

            if ptr.is_null() {
                panic!("Failed reallocating memory.");
            }

            self.data = ptr as *mut T;
            self.capacity = self.size;
        }
    }

    pub fn reallocate(&mut self, size: usize) {
        assert!(size > 0);
        if self.size > size {
            panic!("Can not reallocate to a lower number than the actual size of the Vector.");
        }
        unsafe {
            let layout = alloc::Layout::array::<T>(self.capacity).expect("Invalid layout");
            let new_layout = alloc::Layout::array::<T>(size).expect("Invalid layout");

            let ptr = alloc::realloc(self.data as *mut u8, layout, new_layout.size());

            if ptr.is_null() {
                panic!("Failed reallocating memory.");
            }

            self.data = ptr as *mut T;
            self.capacity = size;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.size {
                ptr::drop_in_place(self.data.add(i));
            }

            let layout = alloc::Layout::array::<T>(self.capacity).expect("Invalid layout");
            alloc::dealloc(self.data as *mut u8, layout);
        }
    }
}

impl<T: fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, ", ")?;
            }
            unsafe {
                write!(f, "{}", &*self.data.add(i))?;
            }
        }
        write!(f, "]")
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "VECTOR {{\n  SIZE: {:?},\n  CAPACITY: {:?},\n  FIXED: {:?},\n  CONTENTS:\n  [\n   ",
                self.size, self.capacity, self.fixed
            )?;
            for i in 0..self.size {
                if i > 0 {
                    write!(f, ",\n   ")?;
                }
                unsafe {
                    write!(f, "{:?}", &*self.data.add(i))?;
                }
            }
            write!(f, "\n  ]\n}}")
        } else {
            write!(
                f,
                "VECTOR {{ SIZE: {:?}, CAPACITY: {:?}, FIXED: {:?}, CONTENTS: ",
                self.size, self.capacity, self.fixed
            )?;
            write!(f, "[")?;
            for i in 0..self.size {
                if i > 0 {
                    write!(f, ", ")?;
                }
                unsafe {
                    write!(f, "{:?}", &*self.data.add(i))?;
                }
            }
            write!(f, "] }}")
        }
    }
}
