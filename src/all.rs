use *;

/// For all loop.
pub struct All<I>(pub Option<I>);

macro_rules! all_impl {
    ($usize:ty) => {
        impl Lup<$usize, bool> for All<$usize> {
            type Inner = Secret<$usize, bool>;
            fn start() -> All<$usize> {All(None)}
            fn it(&mut self, ind: $usize, val: bool) -> bool {
                if !val {
                    self.0 = Some(ind);
                    false
                } else {
                    true
                }
            }
            fn unwrap(self) -> Secret<$usize, bool> {
                Secret {value: self.0.is_none(), evidence: self.0}
            }
        }

        impl Lup<$usize, Secret<$usize, bool>> for All<($usize, $usize)> {
            type Inner = Secret<($usize, $usize), bool>;
            fn start() -> All<($usize, $usize)> {All(None)}
            fn it(&mut self, ind: $usize, val: Secret<$usize, bool>) -> bool {
                if val.value {
                    true
                } else {
                    if let Some(ind2) = val.evidence {
                        self.0 = Some((ind, ind2));
                        false
                    } else {
                        true
                    }
                }
            }
            fn unwrap(self) -> Secret<($usize, $usize), bool> {
                Secret {value: self.0.is_none(), evidence: self.0}
            }
        }

        impl Lup<$usize, Secret<($usize, $usize), bool>> for All<($usize, $usize, $usize)> {
            type Inner = Secret<($usize, $usize, $usize), bool>;
            fn start() -> All<($usize, $usize, $usize)> {All(None)}
            fn it(&mut self, ind: $usize, val: Secret<($usize, $usize), bool>) -> bool {
                if val.value {
                    true
                } else {
                    if let Some((a, b)) = val.evidence {
                        self.0 = Some((ind, a, b));
                        false
                    } else {
                        true
                    }
                }
            }
            fn unwrap(self) -> Secret<($usize, $usize, $usize), bool> {
                Secret {value: self.0.is_none(), evidence: self.0}
            }
        }
    }
}

all_impl!{usize}
