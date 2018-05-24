use *;

/// Indexed for-loop.
pub struct For;

impl Lup<usize, ()> for For {
    type Inner = ();

    fn start() -> Self {For}
    fn it(&mut self, _ind: usize, _val: ()) -> bool {true}
    fn unwrap(self) -> Self::Inner {}
}
