#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::node::{MapleNode, Node, Type};
use image::{DynamicImage, ImageBuffer};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::{CStr, CString};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct UnsafeSend<T>(pub T);

unsafe impl<T> Send for UnsafeSend<T> {}

/// open wz file with given path.
pub fn open_file(path: &str, ctx: *mut wzctx) -> Option<*mut wzfile> {
    unsafe {
        let p = CString::new(path).expect("path");
        let f = wz_open_file(p.as_ptr(), ctx);
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

impl MapleNode for *mut wznode {
    type Item = *mut wznode;
    fn child(self, path: &str) -> Option<Self::Item> {
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

    fn child_at(self, i: u32) -> Option<Self::Item> {
        unsafe {
            let node = wz_open_node_at(self, i);
            if node.is_null() {
                return None;
            } else {
                return Some(node);
            }
        }
    }

    fn len(self) -> u32 {
        unsafe {
            let mut len: wz_uint32_t = 0;
            let ret = wz_get_len(&mut len, self);
            if ret != 0 {
                return 0;
            }
            len
        }
    }

    fn ntype(self) -> Option<Type> {
        unsafe {
            let wz_type = wz_get_type(self);

            FromPrimitive::from_u8(wz_type)
        }
    }

    fn int32(self) -> Option<i32> {
        unsafe {
            let mut val: wz_int32_t = 0;
            let ret = wz_get_int(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn int64(self) -> Option<i64> {
        unsafe {
            let mut val: wz_int64_t = 0;
            let ret = wz_get_i64(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn float32(self) -> Option<f32> {
        unsafe {
            let mut val = 0.0f32;
            let ret = wz_get_f32(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn float64(self) -> Option<f64> {
        unsafe {
            let mut val = 0.0f64;
            let ret = wz_get_f64(&mut val, self);
            match ret {
                0 => Some(val),
                _ => None,
            }
        }
    }

    fn str(self) -> Option<&'static str> {
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

    fn name(self) -> Option<&'static str> {
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

    fn vex_len(self) -> u32 {
        unsafe {
            let mut len: wz_uint32_t = 0;
            let ret = wz_get_vex_len(&mut len, self);
            match ret {
                0 => len,
                _ => 0,
            }
        }
    }

    fn vec(self) -> Option<(i32, i32)> {
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

    fn img(self) -> Option<DynamicImage> {
        unsafe {
            let mut w: wz_uint32_t = 0;
            let mut h: wz_uint32_t = 0;
            let mut d: wz_uint16_t = 0;
            let mut s: wz_uint8_t = 0;
            let ret = wz_get_img(&mut w, &mut h, &mut d, &mut s, self);

            if ret.is_null() {
                return None;
            }

            let len = (w * h * 4) as usize;
            let mut dst = Vec::with_capacity(len);
            std::ptr::copy(ret, dst.as_mut_ptr(), len);
            dst.set_len(len);

            ImageBuffer::from_raw(w, h, dst).map(DynamicImage::ImageBgra8)

            // return Some(DynamicImage {
            //     width: w,
            //     height: h,
            //     depth: d,
            //     scale: s,
            //     pixels: dst,
            // });
        }
    }

    fn iter(self) -> Node<*mut wznode> {
        Node::new(self)
    }
}

impl MapleNode for Option<*mut wznode> {
    type Item = *mut wznode;

    fn child(self, path: &str) -> Option<Self::Item> {
        match self {
            Some(n) => n.child(path),
            None => None,
        }
    }

    fn child_at(self, i: u32) -> Option<Self::Item> {
        match self {
            Some(n) => n.child_at(i),
            None => None,
        }
    }

    fn len(self) -> u32 {
        match self {
            Some(n) => n.len(),
            None => 0,
        }
    }

    fn ntype(self) -> Option<Type> {
        match self {
            Some(n) => n.ntype(),
            None => None,
        }
    }

    fn int32(self) -> Option<i32> {
        match self {
            Some(n) => n.int32(),
            None => None,
        }
    }

    fn int64(self) -> Option<i64> {
        match self {
            Some(n) => n.int64(),
            None => None,
        }
    }

    fn float32(self) -> Option<f32> {
        match self {
            Some(n) => n.float32(),
            None => None,
        }
    }

    fn float64(self) -> Option<f64> {
        match self {
            Some(n) => n.float64(),
            None => None,
        }
    }

    fn str(self) -> Option<&'static str> {
        match self {
            Some(n) => n.str(),
            None => None,
        }
    }

    fn name(self) -> Option<&'static str> {
        match self {
            Some(n) => n.name(),
            None => None,
        }
    }

    fn vex_len(self) -> u32 {
        match self {
            Some(n) => n.vex_len(),
            None => 0,
        }
    }

    fn vec(self) -> Option<(i32, i32)> {
        match self {
            Some(n) => n.vec(),
            None => None,
        }
    }

    fn img(self) -> Option<DynamicImage> {
        match self {
            Some(n) => n.img(),
            None => None,
        }
    }

    fn iter(self) -> Node<*mut wznode> {
        match self {
            Some(n) => n.iter(),
            None => Node::new(std::ptr::null_mut()),
        }
    }
}
