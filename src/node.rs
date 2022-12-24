use anyhow::Result;
use image::DynamicImage;

pub trait MapleNode {
    /// The type of the elements being indexed.
    type Item;

    /// Get child WzNode with given path.
    /// Return [`None`] when path not exists or error occurred.
    fn child<S: AsRef<str>>(&self, path: S) -> Option<Box<Self::Item>>;

    /// Get the i th child WzNode with given index i.
    /// Return [`None`] when no child or the WzNode is not [`Dtype::ARY`] or [`Dtype::IMG`].
    fn child_at(&self, i: u32) -> Option<Box<Self::Item>>;

    /// Get the number of children of node
    fn len(&self) -> u32;

    /// get [`Dtype`] of node
    fn dtype(&self) -> Result<Option<Dtype>>;

    /// Get the i32 value of node with type [`Dtype::I16`] or [`Dtype::I32`]
    fn int32(&self) -> Result<Option<i32>>;

    /// Get the i64 value of node with type [`Dtype::I64`]
    fn int64(&self) -> Result<Option<i64>>;

    /// Get the f32 value of node with type [`Dtype::F32`]
    fn float32(&self) -> Result<Option<f32>>;

    /// Get the f64 value of node with type [`Dtype::F64`]
    fn float64(&self) -> Result<Option<f64>>;

    /// Get the str of node with type [`Dtype::STR`]
    fn str(&self) -> Result<Option<&'static str>>;

    /// Get the name of node
    fn name(&self) -> Result<Option<&'static str>>;

    /// Get the number of children of convex of node with type [`Dtype::VEX`].
    fn vex_len(&self) -> Result<u32>;

    /// Get the vector of node with type [`Dtype::VEC`]
    fn vec(&self) -> Result<Option<glam::Vec2>>;

    /// Get the img of node with type[`Dtype::IMG`]
    fn img(&self) -> Result<Option<DynamicImage>>;
}

#[repr(u16)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum Dtype {
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

impl Dtype {
    pub fn to_str(&self) -> &'static str {
        match self {
            Dtype::NIL => "NIL",
            Dtype::I16 => "I16",
            Dtype::I32 => "I32",
            Dtype::I64 => "I64",
            Dtype::F32 => "F32",
            Dtype::F64 => "F64",
            Dtype::VEC => "VEC",
            Dtype::UNK => "UNK",
            Dtype::ARY => "ARY",
            Dtype::IMG => "IMG",
            Dtype::VEX => "VEX",
            Dtype::AO => "AO",
            Dtype::UOL => "UOL",
            Dtype::STR => "STR",
        }
    }
}
