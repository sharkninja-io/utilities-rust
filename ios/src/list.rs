use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct MantleList<T> {
    pointer: *const T,
    length: u32,
}

impl<T> MantleList<T> {
    pub fn pointer(&self) -> *const T {
        self.pointer
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

impl<T: Copy> MantleList<T> {
    /// # Safety
    ///
    /// Refer to the [std::slice::from_raw_parts] docs.
    pub unsafe fn as_slice(&self) -> &[T] {
        let len = self.length as usize;
        let ptr_props = self.pointer;
        std::slice::from_raw_parts(ptr_props, len)
    }

    /// Copies MantleList content to a new Vec.
    ///
    /// # Safety
    ///
    /// Refer to the [std::slice::from_raw_parts] docs.
    pub unsafe fn copy_to_vec(&self) -> Vec<T> {
        self.as_slice().to_vec()
    }

    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    pub unsafe fn copy_to_vec_ptr(list: *const Self) -> Vec<T> {
        (*list).as_slice().to_vec()
    }

    pub fn vec_to_list_ptr(vec: Vec<T>) -> *const Self {
        let list = MantleList::from(vec);
        Box::into_raw(Box::new(list))
    }
}

impl<T> From<Vec<T>> for MantleList<T> {
    fn from(list: Vec<T>) -> Self {
        let boxed_list = list.into_boxed_slice();
        let pointer = boxed_list.as_ptr();
        let length = boxed_list.len() as u32;
        mem::forget(boxed_list);
        MantleList { pointer, length }
    }
}

/// Copies slice content to MantleList.
impl<T: Copy> From<&[T]> for MantleList<T> {
    fn from(list: &[T]) -> Self {
        let copied_vec: Vec<T> = list.to_vec();
        MantleList::from(copied_vec)
    }
}
