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

                let layout = alloc::Layout::array::<T>(self.capacity).expect("Invalid layout.");

                let ptr = alloc::realloc(self.data as *mut u8, layout, new_cap);

                self.capacity = new_cap;

                if ptr.is_null() {
                    panic!("Failed reallocating memory.");
                }

                self.data = ptr as *mut T;

                self.capacity = (self.scale_fn)(self.capacity);
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

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
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
            write!(f, "VECTOR {{\n  SIZE: {:?},\n  CAPACITY: {:?},\n  FIXED: {:?},\n  CONTENTS:\n  [\n   ", self.size, self.capacity, self.fixed)?;
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
                self.size,
                self.capacity,
                self.fixed
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
