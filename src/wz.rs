use crate::node::{Dtype, MapleNode, Node};
use anyhow::Result;
use anyhow::bail;
use image::{DynamicImage, ImageBuffer};
use num_traits::FromPrimitive;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::c_wz::*;

pub struct WzNode {
    pointer: NonNull<wznode>,
    marker: PhantomData<*const wznode>,
}

impl WzNode {
    pub fn new(pointer: NonNull<wznode>) -> Self {
        WzNode {
            pointer,
            marker: Default::default(),
        }
    }
}

impl Drop for WzNode {
    fn drop(&mut self) {
        unsafe {
            wz_close_node(self.pointer.as_ptr());
        }
    }
}

pub struct WzCtx {
    pointer: NonNull<wzctx>,
    marker: PhantomData<*const wzctx>,
}

impl WzCtx {
    pub fn new(pointer: NonNull<wzctx>) -> Self {
        WzCtx {
            pointer,
            marker: Default::default(),
        }
    }
}

impl Drop for WzCtx {
    fn drop(&mut self) {
        unsafe {
            wz_free_ctx(self.pointer.as_ptr());
        }
    }
}

pub struct WzFile {
    pointer: NonNull<wzfile>,
    marker: PhantomData<*const wzfile>,
}

impl WzFile {
    pub fn new(pointer: NonNull<wzfile>) -> Self {
        WzFile {
            pointer,
            marker: Default::default(),
        }
    }
}

impl Drop for WzFile {
    fn drop(&mut self) {
        unsafe {
            wz_close_file(self.pointer.as_ptr());
        }
    }
}

pub struct UnsafeSend<T>(pub T);

unsafe impl<T> Send for UnsafeSend<T> {}

/// open wz file with given path.
pub fn open_file(path: &str, ctx: WzCtx) -> Result<Option<WzFile>> {
    let path = CString::new(path)?;
    let file = unsafe { wz_open_file(path.as_ptr(), ctx.pointer.as_ptr()) };
    Ok(NonNull::new(file).map(|f| WzFile::new(f)))
}

/// open root node with given wzfile.
pub fn open_root(file: WzFile) -> Result<Option<WzNode>> {
    let root = unsafe { wz_open_root(file.pointer.as_ptr()) };
    Ok(NonNull::new(root).map(|root| WzNode::new(root)))
}

impl MapleNode for WzNode {
    type Item = WzNode;
    fn child(&self, path: &str) -> Option<Self::Item> {
        let path = CString::new(path).unwrap();
        let node = unsafe { wz_open_node(self.pointer.as_ptr(), path.as_ptr()) };
        NonNull::new(node).map(|node| WzNode::new(node))
    }

    fn child_at(&self, i: u32) -> Option<Self::Item> {
        let node = unsafe { wz_open_node_at(self.pointer.as_ptr(), i) };
        NonNull::new(node).map(|node| WzNode::new(node))
    }

    fn len(&self) -> u32 {
        let mut len: wz_uint32_t = 0;
        let ret = unsafe { wz_get_len(&mut len, self.pointer.as_ptr()) };

        match ret {
            0 => len,
            _ => 0,
        }
    }

    fn dtype(&self) -> Result<Option<Dtype>> {
        let wz_type = unsafe { wz_get_type(self.pointer.as_ptr()) };

        Ok(FromPrimitive::from_u8(wz_type))
    }

    fn int32(&self) -> Result<Option<i32>> {
        let mut val: wz_int32_t = 0;
        let ret = unsafe { wz_get_int(&mut val, self.pointer.as_ptr()) };
        if ret == 1 {
            bail!("get int32 error");
        }
        Ok(Some(val as i32))
    }

    fn int64(&self) -> Result<Option<i64>> {
        let mut val: wz_int64_t = 0;
        let ret = unsafe {
            wz_get_i64(&mut val, self.pointer.as_ptr())
        };
        if ret == 1 {
            bail!("get int64 error");
        }
        Ok(Some(val as i64))
    }

    fn float32(&self) -> Result<Option<f32>> {
        let mut val = 0.0f32;
        let ret = unsafe { wz_get_f32(&mut val, self.pointer.as_ptr()) };
        if ret == 1 {
            bail!("get float32 error");
        }
        Ok(Some(val))
    }

    fn float64(&self) -> Result<Option<f64>> {
        let mut val = 0.0f64;
        let ret = unsafe { wz_get_f64(&mut val, self.pointer.as_ptr()) };
        if ret == 1 {
            bail!("get float64 error");
        }
        Ok(Some(val))
    }

    fn str(&self) -> Result<Option<&'static str>> {
        let s = unsafe { wz_get_str(self.pointer.as_ptr()) };

        if s.is_null() {
            bail!("get string error");
        }

        unsafe {
            let s = CStr::from_ptr(s).to_str()?;
            Ok(Some(s))
        }
    }

    fn name(&self) -> Result<Option<&'static str>> {
        let s = unsafe { wz_get_name(self.pointer.as_ptr()) };

        if s.is_null() {
            bail!("get string error");
        }
        unsafe {
            let s = CStr::from_ptr(s).to_str()?;
            Ok(Some(s))
        }
    }

    fn vex_len(&self) -> Result<u32> {
        unsafe {
            let mut len: wz_uint32_t = 0;
            let ret = wz_get_vex_len(&mut len, self.pointer.as_ptr());
            if ret == 1 {
                bail!("vec() failed");
            }
            Ok(len)
        }
    }

    fn vec(&self) -> Result<Option<(i32, i32)>> {
        unsafe {
            let mut x: wz_int32_t = 0;
            let mut y: wz_int32_t = 0;
            let ret = wz_get_vec(&mut x, &mut y, self.pointer.as_ptr());
            if ret == 1 {
                bail!("vec() failed");
            }
            Ok(Some((x, y)))
        }
    }

    fn img(&self) -> Result<Option<DynamicImage>> {
        unsafe {
            let mut w: wz_uint32_t = 0;
            let mut h: wz_uint32_t = 0;
            let mut d: wz_uint16_t = 0;
            let mut s: wz_uint8_t = 0;
            let ret = wz_get_img(&mut w, &mut h, &mut d, &mut s, self.pointer.as_ptr());

            if ret.is_null() {
                bail!("img() failed");
            }

            let len = (w * h * 4) as usize;
            let mut dst = Vec::with_capacity(len);
            std::ptr::copy(ret, dst.as_mut_ptr(), len);
            dst.set_len(len);

            Ok(ImageBuffer::from_raw(w, h, dst).map(DynamicImage::ImageBgra8))
        }
    }

    fn iter(&self) -> Node<Option<&WzNode>> {
        Node::new(Some(&self))
    }
}

impl MapleNode for Option<WzNode> {
    type Item = WzNode;

    fn child(&self, path: &str) -> Option<Self::Item> {
        self.as_ref().map(|node| node.child(path)).unwrap_or(None)
    }

    fn child_at(&self, i: u32) -> Option<Self::Item> {
        self.as_ref().map(|node| node.child_at(i)).unwrap_or(None)
    }

    fn len(&self) -> u32 {
        self.as_ref().map(|node| node.len()).unwrap_or(0)
    }

    fn dtype(&self) -> Result<Option<Dtype>> {
        match self {
            Some(n) => n.dtype(),
            None => Ok(None),
        }
    }

    fn int32(&self) -> Result<Option<i32>> {
        match self {
            Some(n) => n.int32(),
            None => Ok(None),
        }
    }

    fn int64(&self) -> Result<Option<i64>> {
        match self {
            Some(n) => n.int64(),
            None => Ok(None),
        }
    }

    fn float32(&self) -> Result<Option<f32>> {
        match self {
            Some(n) => n.float32(),
            None => Ok(None),
        }
    }

    fn float64(&self) -> Result<Option<f64>> {
        match self {
            Some(n) => n.float64(),
            None => Ok(None),
        }
    }

    fn str(&self) -> Result<Option<&'static str>> {
        match self {
            Some(n) => n.str(),
            None => Ok(None),
        }
    }

    fn name(&self) -> Result<Option<&'static str>> {
        match self {
            Some(n) => n.name(),
            None => Ok(None),
        }
    }

    fn vex_len(&self) -> Result<u32> {
        match self {
            Some(n) => n.vex_len(),
            None => Ok(0),
        }
    }

    fn vec(&self) -> Result<Option<(i32, i32)>> {
        match self {
            Some(n) => n.vec(),
            None => Ok(None),
        }
    }

    fn img(&self) -> Result<Option<DynamicImage>> {
        match self {
            Some(n) => n.img(),
            None => Ok(None),
        }
    }

    fn iter(&self) -> Node<Option<&WzNode>> {
        match self {
            Some(n) => n.iter(),
            None => Node::new(None),
        }
    }
}