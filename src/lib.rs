#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[macro_use]
extern crate num_derive;

use libz_sys::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use once_cell::sync::OnceCell;
use std::{
    ffi::{CStr, CString},
    path::Path,
    sync::Arc,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::sync::Mutex;

pub struct UnsafeSend<T>(pub T);

unsafe impl<T> Send for UnsafeSend<T> {}

fn init_ctx() -> &'static Arc<Mutex<UnsafeSend<*mut wzctx>>> {
    static INSTANCE: OnceCell<Arc<Mutex<UnsafeSend<*mut wzctx>>>> = OnceCell::new();
    INSTANCE.get_or_init(|| unsafe {
        let ctx = wz_init_ctx();
        Arc::new(Mutex::new(UnsafeSend(ctx)))
    })
}

/// open wz file with given path.
pub fn open_file(path: &Path) -> Option<*mut wzfile> {
    let p = CString::new(path.to_str().unwrap()).expect("path");
    unsafe {
        let f = wz_open_file(p.as_ptr(), init_ctx().lock().unwrap().0);
        match f.is_null() {
            false => Some(f),
            true => None,
        }
    }
}

/// open root node with given wzfile.
pub fn open_root(file: *mut wzfile) -> Option<*mut wznode> {
    unsafe {
        let n = wz_open_root(file.into());
        match n.is_null() {
            false => Some(n),
            true => None,
        }
    }
}

pub trait MapleNode {
    /// The type of the elements being opened.
    type Item;

    /// Get the wznode with given path.
    /// Return [`None`] when path not exists or error occurred.
    /// # Examples
    /// ```
    /// let file = open_file("Character.wz");
    /// let root = open_root(file);
    /// if let Some(z_node) = root.open_node("Cape/01102169.img/shootF/2/cape/z") {
    ///     println!("type={:?}", z_node);
    /// }
    ///
    /// ```
    fn open_node(self, path: &str) -> Option<Self::Item>;

    /// Get the i th child wznode of wznode with given index i.
    /// Return [`None`] when no child or the wznode is not [`Type::ARY`] or [`Type::IMG`].
    /// # Examples
    /// ```
    /// let file = open_file("Character.wz");
    /// let root = open_root(file);
    /// if let Some(child) = root.open_node_at(0) {
    ///     println!("name={:?}", child.get_node_name());
    /// }
    ///
    /// ```
    fn open_node_at(self, i: u32) -> Option<Self::Item>;

    /// Get the number of children of node
    fn get_len(self) -> u32;

    /// get [`Type`] of node
    fn get_type(self) -> Option<Type>;

    /// Get the i32 value of node with type [`Type::I16`] or [`Type::I32`]
    fn get_i32(self) -> Option<i32>;

    /// Get the i64 value of node with type [`Type::I64`]
    fn get_i64(self) -> Option<i64>;

    /// Get the f32 value of node with type [`Type::F32`]
    fn get_f32(self) -> Option<f32>;

    /// Get the f64 value of node with type [`Type::F64`]
    fn get_f64(self) -> Option<f64>;

    /// Get the str of node with type [`Type::STR`]
    fn get_str(self) -> Option<&'static str>;

    /// Get the name of node
    fn get_node_name(self) -> Option<&'static str>;

    /// Get the number of children of convex of node with type [`Type::VEX`].
    fn get_vex_len(self) -> u32;

    /// Get the vector of node with type [`Type::VEC`]
    fn get_vec(self) -> Option<(i32, i32)>;

    fn iter(self) -> Node<Self::Item>;
}

impl MapleNode for *mut wznode {
    type Item = *mut wznode;
    fn open_node(self, path: &str) -> Option<Self::Item> {
        unsafe {
            let path = CString::new(path).unwrap();
            let node = wz_open_node(self, path.as_ptr());
            if node.is_null() {
                return None;
            } else {
                return Some(node);
            }
        }
    }

    fn open_node_at(self, i: u32) -> Option<Self::Item> {
        unsafe {
            let node = wz_open_node_at(self, i);
            if node.is_null() {
                return None;
            } else {
                return Some(node);
            }
        }
    }

    fn get_len(self) -> u32 {
        unsafe {
            let mut len: wz_uint32_t = 0;
            let ret = wz_get_len(&mut len, self);
            if ret != 0 {
                return 0;
            }
            len
        }
    }

    fn get_type(self) -> Option<Type> {
        unsafe {
            let wz_type = wz_get_type(self);

            FromPrimitive::from_u8(wz_type)
        }
    }

    fn get_i32(self) -> Option<i32> {
        unsafe {
            let mut val: wz_int32_t = 0;
            let ret = wz_get_int(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn get_i64(self) -> Option<i64> {
        unsafe {
            let mut val: wz_int64_t = 0;
            let ret = wz_get_i64(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn get_f32(self) -> Option<f32> {
        unsafe {
            let mut val = 0.0f32;
            let ret = wz_get_f32(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn get_f64(self) -> Option<f64> {
        unsafe {
            let mut val = 0.0f64;
            let ret = wz_get_f64(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn get_str(self) -> Option<&'static str> {
        unsafe {
            let s = wz_get_str(self);
            if s.is_null() {
                return None;
            }

            match CStr::from_ptr(s).to_str() {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        }
    }

    fn get_node_name(self) -> Option<&'static str> {
        unsafe {
            let s = wz_get_name(self);
            if s.is_null() {
                return None;
            }

            match CStr::from_ptr(s).to_str() {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        }
    }

    fn get_vex_len(self) -> u32 {
        unsafe {
            let mut len: wz_uint32_t = 0;
            let ret = wz_get_vex_len(&mut len, self);
            match ret {
                0 => len,
                _ => 0,
            }
        }
    }

    fn get_vec(self) -> Option<(i32, i32)> {
        unsafe {
            let mut x: wz_int32_t = 0;
            let mut y: wz_int32_t = 0;
            let ret = wz_get_vec(&mut x, &mut y, self);
            match ret {
                0 => Some((x, y)),
                _ => None,
            }
        }
    }

    fn iter(self) -> Node<*mut wznode> {
        match self.is_null() {
            false => Node { data: self, count: self.get_len() as i32 },
            true => Node { data: std::ptr::null_mut(), count: 0 },
        }
    }
}

impl MapleNode for Option<*mut wznode> {
    type Item = *mut wznode;

    fn open_node(self, path: &str) -> Option<Self::Item> {
        match self {
            Some(n) => n.open_node(path),
            None => None,
        }
    }

    fn open_node_at(self, i: u32) -> Option<Self::Item> {
        match self {
            Some(n) => n.open_node_at(i),
            None => None,
        }
    }

    fn get_len(self) -> u32 {
        match self {
            Some(n) => n.get_len(),
            None => 0,
        }
    }

    fn get_type(self) -> Option<Type> {
        match self {
            Some(n) => n.get_type(),
            None => None,
        }
    }

    fn get_i32(self) -> Option<i32> {
        match self {
            Some(n) => n.get_i32(),
            None => None,
        }
    }

    fn get_i64(self) -> Option<i64> {
        match self {
            Some(n) => n.get_i64(),
            None => None,
        }
    }

    fn get_f32(self) -> Option<f32> {
        match self {
            Some(n) => n.get_f32(),
            None => None,
        }
    }

    fn get_f64(self) -> Option<f64> {
        match self {
            Some(n) => n.get_f64(),
            None => None,
        }
    }

    fn get_str(self) -> Option<&'static str> {
        match self {
            Some(n) => n.get_str(),
            None => None,
        }
    }

    fn get_node_name(self) -> Option<&'static str> {
        match self {
            Some(n) => n.get_node_name(),
            None => None,
        }
    }

    fn get_vex_len(self) -> u32 {
        match self {
            Some(n) => n.get_vex_len(),
            None => 0,
        }
    }

    fn get_vec(self) -> Option<(i32, i32)> {
        match self {
            Some(n) => n.get_vec(),
            None => None,
        }
    }

    fn iter(self) -> Node<*mut wznode> {
        match self {
            Some(n) => n.iter(),
            None => Node { data: std::ptr::null_mut(), count: 0 },
        }
    }
}


#[repr(u16)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum Type {
    /// A node with nothing
    NIL = 0,
    /// A node with i16
    I16 = 1,
    /// A node with i32
    I32 = 2,
    /// A node with i64
    I64 = 3,
    /// A node with f32
    F32 = 4,
    /// A node with f64
    F64 = 5,
    /// A node with vector (pair of i32 x and i32 y)
    VEC = 6,
    /// A node which is not read yet
    UNK = 7,
    /// A node with array (children of node)
    ARY = 8,
    /// A node with image (data: *i8, w: u32, h: u32, depth: u16, scale: u8)
    /// which may have children of node
    IMG = 9,
    /// a node with convex (multiple pairs of x: i32 and y: i32)
    VEX = 10,
    /// a node with audio (data: *u8, size: u32, ms: u32, format: u16)
    AO = 11,
    /// UOL
    UOL = 12,
    /// a node with string (const char * utf8)
    STR = 13,
}

pub struct Node<T> {
    pub data: T,
    pub count: i32,
}

impl<T> From<T> for Node<T> where T: MapleNode<Item=T> + Copy {
    fn from(n: T) -> Self {
        let len = n.get_len() as i32;
        Node { data: n, count: len }
    }
}

impl<T> Iterator for Node<T> where T: MapleNode<Item=T> + Copy {
    type Item = T;
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
    #[inline]
    fn next(&mut self) -> Option<T> {
        match self.count {
            0 => None,
            _ => {
                self.count -= 1;
                self.data.open_node_at(self.count as u32)
            }
        }
    }
}
