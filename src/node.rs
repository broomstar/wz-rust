use anyhow::Result;
use image::DynamicImage;

pub trait MapleNode {
    /// The type of the elements being opened.
    type Item;

    /// Get the wznode with given path.
    /// Return [`None`] when path not exists or error occurred.
    /// # Examples
    /// ```
    /// let file = open_file("Character.wz");
    /// let root = open_root(file);
    /// if let Some(z_node) = root.child("Cape/01102169.img/shootF/2/cape/z") {
    ///     println!("type={:?}", z_node);
    /// }
    ///
    /// ```
    fn child(&self, path: &str) -> Option<Box<Self::Item>>;

    /// Get the i th child wznode of wznode with given index i.
    /// Return [`None`] when no child or the wznode is not [`Type::ARY`] or [`Type::IMG`].
    /// # Examples
    /// ```
    /// let file = open_file("Character.wz");
    /// let root = open_root(file);
    /// if let Some(child) = root.child_at(0) {
    ///     println!("name={:?}", child.get_node_name());
    /// }
    ///
    /// ```
    fn child_at(&self, i: u32) -> Option<Box<Self::Item>>;

    /// Get the number of children of node
    fn len(&self) -> u32;

    /// get [`Type`] of node
    fn dtype(&self) -> Result<Option<Dtype>>;

    /// Get the i32 value of node with type [`Type::I16`] or [`Type::I32`]
    fn int32(&self) -> Result<Option<i32>>;

    /// Get the i64 value of node with type [`Type::I64`]
    fn int64(&self) -> Result<Option<i64>>;

    /// Get the f32 value of node with type [`Type::F32`]
    fn float32(&self) -> Result<Option<f32>>;

    /// Get the f64 value of node with type [`Type::F64`]
    fn float64(&self) -> Result<Option<f64>>;

    /// Get the str of node with type [`Type::STR`]
    fn str(&self) -> Result<Option<&'static str>>;

    /// Get the name of node
    fn name(&self) -> Result<Option<&'static str>>;

    /// Get the number of children of convex of node with type [`Type::VEX`].
    fn vex_len(&self) -> Result<u32>;

    /// Get the vector of node with type [`Type::VEC`]
    fn vec(&self) -> Result<Option<(i32, i32)>>;

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
