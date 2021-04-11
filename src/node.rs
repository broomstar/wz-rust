use image::DynamicImage;

pub struct Node<T> {
    data: T,
    index: u32,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, index: 0 }
    }
}

impl<T> From<T> for Node<T>
where
    T: MapleNode<Item = T> + Copy,
{
    fn from(n: T) -> Self {
        Node::new(n)
    }
}

impl<T> Iterator for Node<T>
where
    T: MapleNode<Item = T> + Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.index >= self.data.get_len() {
            return None;
        }
        let index = self.index;
        self.index = index + 1;
        self.data.open_node_at(index)
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

    fn get_img(self) -> Option<DynamicImage>;

    fn iter(self) -> Node<Self::Item>;
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
