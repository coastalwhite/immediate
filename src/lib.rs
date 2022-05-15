#[derive(Default, Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct Immediate32<const N: usize>(u32);

impl Immediate32<32> {
    #[inline(always)]
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl<const N: usize> Into<u32> for Immediate32<N> {
    #[inline(always)]
    fn into(self) -> u32 {
        let Self(value) = self;
        value
    }
}

impl Into<bool> for Immediate32<1> {
    #[inline(always)]
    fn into(self) -> bool {
        if self == 1 {
            true
        } else {
            false
        }
    }
}

impl<const N: usize> PartialEq<u32> for Immediate32<N> {
    #[inline(always)]
    fn eq(&self, other: &u32) -> bool {
        let Self(value) = self;
        value == other
    }
}
impl PartialEq<bool> for Immediate32<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        (*self == 1 && *other) || (*self == 0 && !other)
    }
}

impl<const N: usize> Eq for Immediate32<N> {}

// F > T
pub trait Immediate32DownCast<const F: usize, const T: usize>: Copy + Into<u32> {
    #[inline(always)]
    fn take_low(self) -> Immediate32<T> {
        let value: u32 = self.into();
        Immediate32(value & (!((!0) << T)))
    }
    #[inline(always)]
    fn take_high(self) -> Immediate32<T> {
        let value: u32 = self.into();
        Immediate32(value >> (F - T))
    }
}

pub trait Immediate32UpCast<const F: usize, const T: usize>: Copy + Into<u32> {
    #[inline(always)]
    fn zero_extend(self) -> Immediate32<T> {
        let value = self.into();
        Immediate32(value)
    }
}

pub trait Immediate32Concatable<const R: usize, const O: usize>: Copy + Into<u32> {
    fn concat(self, rhs: Immediate32<R>) -> Immediate32<O> {
        let lhs = self.into();
        let rhs: u32 = rhs.into();

        Immediate32((lhs << R) + rhs)
    }
}

impl<const F: usize, const T: usize> Immediate32UpCast<F, T> for Immediate32<F> where
    Immediate32<T>: Immediate32DownCast<T, F>
{
}

macro_rules! impl_imm32 {
    ($i:literal, [$($l:literal),* $(,)?], [$(($add:literal, $sum:literal)),* $(,)?]) => {
        impl Immediate32<$i> {
            pub fn zero() -> Immediate32<$i> {
                Self(0)
            }
        }

        // Implement up/downcasting to same size
        $(
            impl Immediate32DownCast<$i, $l> for Immediate32<$i> {}
        )*

        $(
            impl Immediate32Concatable<$add, $sum> for Immediate32<$i> {}
        )*
    };
}

impl_imm32!(1, [1], [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11), (11, 12), (12, 13), (13, 14), (14, 15), (15, 16), (16, 17), (17, 18), (18, 19), (19, 20), (20, 21), (21, 22), (22, 23), (23, 24), (24, 25), (25, 26), (26, 27), (27, 28), (28, 29), (29, 30), (30, 31), (31, 32)]);
impl_imm32!(2, [1, 2], [(1, 3), (2, 4), (3, 5), (4, 6), (5, 7), (6, 8), (7, 9), (8, 10), (9, 11), (10, 12), (11, 13), (12, 14), (13, 15), (14, 16), (15, 17), (16, 18), (17, 19), (18, 20), (19, 21), (20, 22), (21, 23), (22, 24), (23, 25), (24, 26), (25, 27), (26, 28), (27, 29), (28, 30), (29, 31), (30, 32)]);
impl_imm32!(3, [1, 2, 3], [(1, 4), (2, 5), (3, 6), (4, 7), (5, 8), (6, 9), (7, 10), (8, 11), (9, 12), (10, 13), (11, 14), (12, 15), (13, 16), (14, 17), (15, 18), (16, 19), (17, 20), (18, 21), (19, 22), (20, 23), (21, 24), (22, 25), (23, 26), (24, 27), (25, 28), (26, 29), (27, 30), (28, 31), (29, 32)]);
impl_imm32!(4, [1, 2, 3, 4], [(1, 5), (2, 6), (3, 7), (4, 8), (5, 9), (6, 10), (7, 11), (8, 12), (9, 13), (10, 14), (11, 15), (12, 16), (13, 17), (14, 18), (15, 19), (16, 20), (17, 21), (18, 22), (19, 23), (20, 24), (21, 25), (22, 26), (23, 27), (24, 28), (25, 29), (26, 30), (27, 31), (28, 32)]);
impl_imm32!(5, [1, 2, 3, 4, 5], [(1, 6), (2, 7), (3, 8), (4, 9), (5, 10), (6, 11), (7, 12), (8, 13), (9, 14), (10, 15), (11, 16), (12, 17), (13, 18), (14, 19), (15, 20), (16, 21), (17, 22), (18, 23), (19, 24), (20, 25), (21, 26), (22, 27), (23, 28), (24, 29), (25, 30), (26, 31), (27, 32)]);
impl_imm32!(6, [1, 2, 3, 4, 5, 6], [(1, 7), (2, 8), (3, 9), (4, 10), (5, 11), (6, 12), (7, 13), (8, 14), (9, 15), (10, 16), (11, 17), (12, 18), (13, 19), (14, 20), (15, 21), (16, 22), (17, 23), (18, 24), (19, 25), (20, 26), (21, 27), (22, 28), (23, 29), (24, 30), (25, 31), (26, 32)]);
impl_imm32!(7, [1, 2, 3, 4, 5, 6, 7], [(1, 8), (2, 9), (3, 10), (4, 11), (5, 12), (6, 13), (7, 14), (8, 15), (9, 16), (10, 17), (11, 18), (12, 19), (13, 20), (14, 21), (15, 22), (16, 23), (17, 24), (18, 25), (19, 26), (20, 27), (21, 28), (22, 29), (23, 30), (24, 31), (25, 32)]);
impl_imm32!(8, [1, 2, 3, 4, 5, 6, 7, 8], [(1, 9), (2, 10), (3, 11), (4, 12), (5, 13), (6, 14), (7, 15), (8, 16), (9, 17), (10, 18), (11, 19), (12, 20), (13, 21), (14, 22), (15, 23), (16, 24), (17, 25), (18, 26), (19, 27), (20, 28), (21, 29), (22, 30), (23, 31), (24, 32)]);
impl_imm32!(9, [1, 2, 3, 4, 5, 6, 7, 8, 9], [(1, 10), (2, 11), (3, 12), (4, 13), (5, 14), (6, 15), (7, 16), (8, 17), (9, 18), (10, 19), (11, 20), (12, 21), (13, 22), (14, 23), (15, 24), (16, 25), (17, 26), (18, 27), (19, 28), (20, 29), (21, 30), (22, 31), (23, 32)]);
impl_imm32!(10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], [(1, 11), (2, 12), (3, 13), (4, 14), (5, 15), (6, 16), (7, 17), (8, 18), (9, 19), (10, 20), (11, 21), (12, 22), (13, 23), (14, 24), (15, 25), (16, 26), (17, 27), (18, 28), (19, 29), (20, 30), (21, 31), (22, 32)]);
impl_imm32!(11, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], [(1, 12), (2, 13), (3, 14), (4, 15), (5, 16), (6, 17), (7, 18), (8, 19), (9, 20), (10, 21), (11, 22), (12, 23), (13, 24), (14, 25), (15, 26), (16, 27), (17, 28), (18, 29), (19, 30), (20, 31), (21, 32)]);
impl_imm32!(12, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], [(1, 13), (2, 14), (3, 15), (4, 16), (5, 17), (6, 18), (7, 19), (8, 20), (9, 21), (10, 22), (11, 23), (12, 24), (13, 25), (14, 26), (15, 27), (16, 28), (17, 29), (18, 30), (19, 31), (20, 32)]);
impl_imm32!(13, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13], [(1, 14), (2, 15), (3, 16), (4, 17), (5, 18), (6, 19), (7, 20), (8, 21), (9, 22), (10, 23), (11, 24), (12, 25), (13, 26), (14, 27), (15, 28), (16, 29), (17, 30), (18, 31), (19, 32)]);
impl_imm32!(14, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14], [(1, 15), (2, 16), (3, 17), (4, 18), (5, 19), (6, 20), (7, 21), (8, 22), (9, 23), (10, 24), (11, 25), (12, 26), (13, 27), (14, 28), (15, 29), (16, 30), (17, 31), (18, 32)]);
impl_imm32!(15, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], [(1, 16), (2, 17), (3, 18), (4, 19), (5, 20), (6, 21), (7, 22), (8, 23), (9, 24), (10, 25), (11, 26), (12, 27), (13, 28), (14, 29), (15, 30), (16, 31), (17, 32)]);
impl_imm32!(16, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], [(1, 17), (2, 18), (3, 19), (4, 20), (5, 21), (6, 22), (7, 23), (8, 24), (9, 25), (10, 26), (11, 27), (12, 28), (13, 29), (14, 30), (15, 31), (16, 32)]);
impl_imm32!(17, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17], [(1, 18), (2, 19), (3, 20), (4, 21), (5, 22), (6, 23), (7, 24), (8, 25), (9, 26), (10, 27), (11, 28), (12, 29), (13, 30), (14, 31), (15, 32)]);
impl_imm32!(18, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18], [(1, 19), (2, 20), (3, 21), (4, 22), (5, 23), (6, 24), (7, 25), (8, 26), (9, 27), (10, 28), (11, 29), (12, 30), (13, 31), (14, 32)]);
impl_imm32!(19, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19], [(1, 20), (2, 21), (3, 22), (4, 23), (5, 24), (6, 25), (7, 26), (8, 27), (9, 28), (10, 29), (11, 30), (12, 31), (13, 32)]);
impl_imm32!(20, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], [(1, 21), (2, 22), (3, 23), (4, 24), (5, 25), (6, 26), (7, 27), (8, 28), (9, 29), (10, 30), (11, 31), (12, 32)]);
impl_imm32!(21, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21], [(1, 22), (2, 23), (3, 24), (4, 25), (5, 26), (6, 27), (7, 28), (8, 29), (9, 30), (10, 31), (11, 32)]);
impl_imm32!(22, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22], [(1, 23), (2, 24), (3, 25), (4, 26), (5, 27), (6, 28), (7, 29), (8, 30), (9, 31), (10, 32)]);
impl_imm32!(23, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23], [(1, 24), (2, 25), (3, 26), (4, 27), (5, 28), (6, 29), (7, 30), (8, 31), (9, 32)]);
impl_imm32!(24, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24], [(1, 25), (2, 26), (3, 27), (4, 28), (5, 29), (6, 30), (7, 31), (8, 32)]);
impl_imm32!(25, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], [(1, 26), (2, 27), (3, 28), (4, 29), (5, 30), (6, 31), (7, 32)]);
impl_imm32!(26, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26], [(1, 27), (2, 28), (3, 29), (4, 30), (5, 31), (6, 32)]);
impl_imm32!(27, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27], [(1, 28), (2, 29), (3, 30), (4, 31), (5, 32)]);
impl_imm32!(28, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28], [(1, 29), (2, 30), (3, 31), (4, 32)]);
impl_imm32!(29, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29], [(1, 30), (2, 31), (3, 32)]);
impl_imm32!(30, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30], [(1, 31), (2, 32)]);
impl_imm32!(31, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31], [(1, 32)]);
impl_imm32!(32, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32], []);
