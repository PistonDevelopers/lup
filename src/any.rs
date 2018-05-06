use *;

/// There exists loop.
///
/// Returns `None` if there was no item in the loop satisfying the condition.
/// Returns `Some(item)` if an item satisfies the condition.
pub struct Any<I>(pub Option<I>);

macro_rules! any_impl {
    ($usize:ty) => {
        impl Lup<$usize, bool> for Any<$usize> {
            type Inner = Secret<$usize, bool>;
            fn start() -> Any<$usize> {Any(None)}
            fn it(&mut self, ind: $usize, val: bool) -> bool {
                if val {
                    self.0 = Some(ind);
                    false
                } else {
                    true
                }
            }
            fn unwrap(self) -> Self::Inner {
                Secret {value: self.0.is_some(), evidence: self.0}
            }
        }

        impl Lup<$usize, Secret<$usize, bool>> for Any<($usize, $usize)> {
            type Inner = Secret<($usize, $usize), bool>;
            fn start() -> Any<($usize, $usize)> {Any(None)}
            fn it(&mut self, ind: $usize, val: Secret<$usize, bool>) -> bool {
                if val.value {
                    if let Some(ind2) = val.evidence {
                        self.0 = Some((ind, ind2));
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            fn unwrap(self) -> Self::Inner {
                Secret {value: self.0.is_some(), evidence: self.0}
            }
        }

        impl Lup<$usize, Secret<($usize, $usize), bool>> for Any<($usize, $usize, $usize)> {
            type Inner = Secret<($usize, $usize, $usize), bool>;
            fn start() -> Any<($usize, $usize, $usize)> {Any(None)}
            fn it(&mut self, ind: $usize, val: Secret<($usize, $usize), bool>) -> bool {
                if val.value {
                    if let Some((a, b)) = val.evidence {
                        self.0 = Some((ind, a, b));
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            fn unwrap(self) -> Self::Inner {
                Secret {value: self.0.is_some(), evidence: self.0}
            }
        }
    }
}

any_impl!{usize}
