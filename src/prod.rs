use *;

/// Product loop.
pub struct Prod<T>(pub T);

macro_rules! prod_impl {
    ($usize:ty , $f32:ty) => {
        impl Lup<$usize, $f32> for Prod<$f32> {
            type Inner = $f32;
            fn start() -> Prod<$f32> {Prod(1.0)}
            fn it(&mut self, _ind: $usize, val: $f32) -> bool {self.0 *= val; true}
            fn unwrap(self) -> $f32 {self.0}
        }

        impl Lup<$usize, [$f32; 2]> for Prod<[$f32; 2]> {
            type Inner = [$f32; 2];
            fn start() -> Prod<[$f32; 2]> {Prod([1.0; 2])}
            fn it(&mut self, _ind: $usize, val: [$f32; 2]) -> bool {
                self.0 = [self.0[0] * val[0], self.0[1] * val[1]];
                true
            }
            fn unwrap(self) -> [$f32; 2] {self.0}
        }

        impl Lup<$usize, [$f32; 3]> for Prod<[$f32; 3]> {
            type Inner = [$f32; 3];
            fn start() -> Prod<[$f32; 3]> {Prod([1.0; 3])}
            fn it(&mut self, _ind: $usize, val: [$f32; 3]) -> bool {
                self.0 = [self.0[0] * val[0], self.0[1] * val[1], self.0[2] * val[2]];
                true
            }
            fn unwrap(self) -> [$f32; 3] {self.0}
        }

        impl Lup<$usize, [$f32; 4]> for Prod<[$f32; 4]> {
            type Inner = [$f32; 4];
            fn start() -> Prod<[$f32; 4]> {Prod([1.0; 4])}
            fn it(&mut self, _ind: $usize, val: [$f32; 4]) -> bool {
                self.0 = [
                    self.0[0] * val[0],
                    self.0[1] * val[1],
                    self.0[2] * val[2],
                    self.0[3] * val[3],
                ];
                true
            }
            fn unwrap(self) -> [$f32; 4] {self.0}
        }
    }
}

prod_impl!{usize, f32}

prod_impl!{usize, f64}
