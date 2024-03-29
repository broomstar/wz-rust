use crate::{
    c_wz::*,
    node::{Dtype, ImageBuffer, MapleNode},
};
use anyhow::{bail, Result};
use num_traits::FromPrimitive;
use std::{
    ffi::{CStr, CString},
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ptr::NonNull,
};

pub struct WzNode {
    pointer: NonNull<wznode>,
    pub path: Option<String>,
    marker: PhantomData<*const wznode>,
}

impl WzNode {
    pub fn new(pointer: NonNull<wznode>, path: &str) -> Self {
        WzNode { pointer, path: Some(path.to_owned()), marker: Default::default() }
    }
}

impl Debug for WzNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut path = "".to_string();
        if let Some(s) = self.path.as_ref() {
            path = s.clone();
        }
        let mut dtype = "Error".to_string();
        if let Ok(Some(t)) = self.dtype() {
            dtype = t.to_str().to_owned();
        }

        let mut val = String::new();
        if let Ok(Some(t)) = self.dtype() {
            match t {
                Dtype::NIL => {}
                Dtype::I16 | Dtype::I32 => {
                    if let Ok(Some(v)) = self.int32() {
                        val = v.to_string();
                    }
                }
                Dtype::I64 => {
                    if let Ok(Some(v)) = self.int64() {
                        val = v.to_string();
                    }
                }
                Dtype::F32 => {
                    if let Ok(Some(v)) = self.float32() {
                        val = v.to_string();
                    }
                }
                Dtype::F64 => {
                    if let Ok(Some(v)) = self.float64() {
                        val = v.to_string();
                    }
                }
                Dtype::VEC => {
                    if let Ok(Some(v)) = self.vec() {
                        val = format!("vec({})", v);
                    }
                }
                Dtype::UNK => {}
                Dtype::ARY => {
                    val = format!("child num={}", self.len());
                }
                Dtype::IMG => {
                    if let Ok(Some(p)) = self.img() {
                        val = format!("dim:{}x{},child num={}", p.width, p.height, self.len());
                    }
                }
                Dtype::VEX => {}
                Dtype::AO => {}
                Dtype::UOL => {}
                Dtype::STR => {
                    if let Ok(Some(s)) = self.str() {
                        val = s.to_string();
                    }
                }
            }
        }

        let _ = write!(f, "WzNode Path[{path}] Type[{dtype}] Value[{val}]", path = path, dtype = dtype, val = val);
        Ok(())
    }
}

impl WzNode {
    pub fn iter(&self) -> WzNodeIter<'_> {
        WzNodeIter::new(self)
    }
}

pub struct WzNodeIter<'a> {
    base: &'a WzNode,
    index: u32,
}

impl<'a> WzNodeIter<'a> {
    pub fn new(base: &'a WzNode) -> Self {
        Self { base, index: 0 }
    }
}

impl<'a> Iterator for WzNodeIter<'a> {
    type Item = Box<WzNode>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.base.len() {
            return None;
        }
        let child_node = self.base.child_at(self.index);
        self.index += 1;
        child_node
    }
}

pub struct WzCtx {
    pointer: NonNull<wzctx>,
    marker: PhantomData<*const wzctx>,
}

impl WzCtx {
    pub fn new() -> Result<Self> {
        let pointer = unsafe { wz_init_ctx() };
        let pointer = NonNull::new(pointer);
        match pointer {
            Some(pointer) => Ok(WzCtx { pointer, marker: PhantomData::default() }),
            None => bail!("new WzCtx failed!"),
        }
    }

    /// open wz file with given path.
    pub fn open_file(&self, path: &str) -> Result<Option<WzFile>> {
        let path = CString::new(path)?;
        let file = unsafe { wz_open_file(path.as_ptr(), self.pointer.as_ptr()) };
        Ok(NonNull::new(file).map(|f| WzFile::new(f)))
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
        WzFile { pointer, marker: Default::default() }
    }

    /// open root node with given wzfile.
    pub fn open_root<'a>(&self) -> Result<Option<Box<WzNode>>> {
        let root = unsafe { wz_open_root(self.pointer.as_ptr()) };
        Ok(NonNull::new(root).map(|root| Box::new(WzNode::new(root, ""))))
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

impl<T: MapleNode> MapleNode for &mut T {
    type Item = T::Item;

    fn child<S: AsRef<str>>(&self, path: S) -> Option<Box<Self::Item>> {
        (**self).child(path)
    }

    fn child_at(&self, i: u32) -> Option<Box<Self::Item>> {
        (**self).child_at(i)
    }

    fn len(&self) -> u32 {
        (**self).len()
    }

    fn dtype(&self) -> Result<Option<Dtype>> {
        (**self).dtype()
    }

    fn int32(&self) -> Result<Option<i32>> {
        (**self).int32()
    }

    fn int64(&self) -> Result<Option<i64>> {
        (**self).int64()
    }

    fn float32(&self) -> Result<Option<f32>> {
        (**self).float32()
    }

    fn float64(&self) -> Result<Option<f64>> {
        (**self).float64()
    }

    fn str(&self) -> Result<Option<&'static str>> {
        (**self).str()
    }

    fn name(&self) -> Result<Option<&'static str>> {
        (**self).name()
    }

    fn vex_len(&self) -> Result<u32> {
        (**self).vex_len()
    }

    fn vec(&self) -> Result<Option<glam::Vec2>> {
        (**self).vec()
    }

    fn img<'a>(&self) -> Result<Option<ImageBuffer<'a>>> {
        (**self).img()
    }
}

impl MapleNode for WzNode {
    type Item = WzNode;

    fn child<S: AsRef<str>>(&self, path: S) -> Option<Box<Self::Item>> {
        let self_path = match &self.path {
            Some(path) => &path,
            None => "",
        };
        let child_path = &*format!("{}/{}", self_path, path.as_ref());
        let path = CString::new(path.as_ref()).unwrap();
        let node = unsafe { wz_open_node(self.pointer.as_ptr(), path.as_ptr()) };
        NonNull::new(node).map(|node| Box::new(WzNode::new(node, child_path)))
    }

    fn child_at(&self, i: u32) -> Option<Box<Self::Item>> {
        let self_path = match &self.path {
            Some(path) => &path,
            None => "",
        };
        let node = unsafe { wz_open_node_at(self.pointer.as_ptr(), i) };
        NonNull::new(node).map(|node| {
            let mut node = WzNode::new(node, "");
            let node_name = match node.name() {
                Ok(Some(node_name)) => node_name,
                _ => "",
            };
            let child_path = format!("{}/{}", self_path, node_name);
            node.path = Some(child_path);
            Box::new(node)
        })
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
        let ret = unsafe { wz_get_i64(&mut val, self.pointer.as_ptr()) };
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

    fn vec(&self) -> Result<Option<glam::Vec2>> {
        unsafe {
            let mut x: wz_int32_t = 0;
            let mut y: wz_int32_t = 0;
            let ret = wz_get_vec(&mut x, &mut y, self.pointer.as_ptr());
            if ret == 1 {
                bail!("vec() failed");
            }
            Ok(Some(glam::Vec2::new(x as f32, y as f32)))
        }
    }

    fn img<'a>(&self) -> Result<Option<ImageBuffer<'a>>> {
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
            let data: &'a mut [u8] = std::slice::from_raw_parts_mut(ret, len);
            Ok(Some(ImageBuffer { width: w, height: h, depth: d, scale: s, data }))
        }
    }
}

impl<T: MapleNode> MapleNode for Option<Box<T>> {
    type Item = T::Item;

    fn child<S: AsRef<str>>(&self, path: S) -> Option<Box<Self::Item>> {
        self.as_ref().map(|node| node.child(path)).unwrap_or(None)
    }

    fn child_at(&self, i: u32) -> Option<Box<Self::Item>> {
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

    fn vec(&self) -> Result<Option<glam::Vec2>> {
        match self {
            Some(n) => n.vec(),
            None => Ok(None),
        }
    }

    fn img<'a>(&self) -> Result<Option<ImageBuffer<'a>>> {
        match self {
            Some(n) => n.img(),
            None => Ok(None),
        }
    }
}
