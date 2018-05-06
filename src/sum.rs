use *;

/// Sum loop.
pub struct Sum<T>(pub T);

macro_rules! sum_impl {
    ($usize:ty , $f32:ty) => {
        impl Lup<$usize, $f32> for Sum<$f32> {
            type Inner = $f32;
            fn start() -> Sum<$f32> {Sum(0.0)}
            fn it(&mut self, _ind: $usize, val: $f32) -> bool {self.0 += val; true}
            fn unwrap(self) -> $f32 {self.0}
        }

        impl Lup<$usize, [$f32; 2]> for Sum<[$f32; 2]> {
            type Inner = [$f32; 2];
            fn start() -> Sum<[$f32; 2]> {Sum([0.0; 2])}
            fn it(&mut self, _ind: $usize, val: [$f32; 2]) -> bool {
                self.0 = [self.0[0] + val[0], self.0[1] + val[1]];
                true
            }
            fn unwrap(self) -> [$f32; 2] {self.0}
        }

        impl Lup<$usize, [$f32; 3]> for Sum<[$f32; 3]> {
            type Inner = [$f32; 3];
            fn start() -> Sum<[$f32; 3]> {Sum([0.0; 3])}
            fn it(&mut self, _ind: $usize, val: [$f32; 3]) -> bool {
                self.0 = [self.0[0] + val[0], self.0[1] + val[1], self.0[2] + val[2]];
                true
            }
            fn unwrap(self) -> [$f32; 3] {self.0}
        }

        impl Lup<$usize, [$f32; 4]> for Sum<[$f32; 4]> {
            type Inner = [$f32; 4];
            fn start() -> Sum<[$f32; 4]> {Sum([0.0; 4])}
            fn it(&mut self, _ind: $usize, val: [$f32; 4]) -> bool {
                self.0 = [
                    self.0[0] + val[0],
                    self.0[1] + val[1],
                    self.0[2] + val[2],
                    self.0[3] + val[3],
                ];
                true
            }
            fn unwrap(self) -> [$f32; 4] {self.0}
        }
    }
}

sum_impl!{usize, f32}

sum_impl!{usize, f64}
