use *;

/// Vector construction loop.
///
/// Example:
///
/// ```
/// #[macro_use]
/// extern crate lup;
///
/// use lup::Vector;
///
/// fn main() {
///     let a = lup!(Vector<[f64; 4]>: i in 0..4 => {i as f64});
///     println!("{:?}", a); // Prints `[0.0, 1.0, 2.0, 3.0]`.
/// }
/// ```
pub struct Vector<T>(pub T);

impl<T: Default + Copy> Lup<usize, T> for Vector<[T; 2]> {
    type Inner = [T; 2];
    fn start() -> Self {Vector([Default::default(); 2])}
    fn it(&mut self, ind: usize, val: T) -> bool {
        self.0[ind] = val;
        true
    }
    fn unwrap(self) -> Self::Inner {self.0}
}

impl<T: Default + Copy> Lup<usize, T> for Vector<[T; 3]> {
    type Inner = [T; 3];
    fn start() -> Self {Vector([Default::default(); 3])}
    fn it(&mut self, ind: usize, val: T) -> bool {
        self.0[ind] = val;
        true
    }
    fn unwrap(self) -> Self::Inner {self.0}
}

impl<T: Default + Copy> Lup<usize, T> for Vector<[T; 4]> {
    type Inner = [T; 4];
    fn start() -> Self {Vector([Default::default(); 4])}
    fn it(&mut self, ind: usize, val: T) -> bool {
        self.0[ind] = val;
        true
    }
    fn unwrap(self) -> Self::Inner {self.0}
}
