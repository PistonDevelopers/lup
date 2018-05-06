use *;

/// A loop that creates a list.
pub struct Sift<T>(pub T);

impl<T> Lup<usize, T> for Sift<Vec<T>> {
    type Inner = Vec<T>;

    fn start() -> Self {Sift(vec![])}
    fn it(&mut self, _ind: usize, val: T) -> bool {
        self.0.push(val);
        true
    }
    fn unwrap(self) -> Self::Inner {self.0}
}
