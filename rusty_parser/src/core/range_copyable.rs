/// Defined new RangeBound types.
/// They are clones of std::ops::Range, std::ops::RangeFrom, std::ops::RangeTo, std::ops::RangeFull, std::ops::RangeInclusive, std::ops::RangeToInclusive.
/// but implemements `Copy` trait.

pub trait ToCopyable {
    type Into: Clone + Copy;
    fn into(self) -> Self::Into;
}

pub trait RangeBound<Idx>: Clone + Copy {
    fn contains(&self, count: &Idx) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct Range<Idx>
where
    Idx: Clone + Copy,
{
    pub start: Idx,
    pub end: Idx,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeFrom<Idx>
where
    Idx: Clone + Copy,
{
    pub start: Idx,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeTo<Idx>
where
    Idx: Clone + Copy,
{
    pub end: Idx,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeFull;

#[derive(Debug, Clone, Copy)]
pub struct RangeInclusive<Idx>
where
    Idx: Clone + Copy,
{
    pub start: Idx,
    pub end: Idx,
}

#[derive(Debug, Clone, Copy)]
pub struct RangeToInclusive<Idx>
where
    Idx: Clone + Copy,
{
    pub end: Idx,
}

impl<Idx> ToCopyable for std::ops::Range<Idx>
where
    Idx: Clone + Copy,
{
    type Into = Range<Idx>;
    fn into(self) -> Self::Into {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

impl<Idx> ToCopyable for std::ops::RangeFrom<Idx>
where
    Idx: Clone + Copy,
{
    type Into = RangeFrom<Idx>;
    fn into(self) -> Self::Into {
        RangeFrom { start: self.start }
    }
}

impl<Idx> ToCopyable for std::ops::RangeTo<Idx>
where
    Idx: Clone + Copy,
{
    type Into = RangeTo<Idx>;
    fn into(self) -> Self::Into {
        RangeTo { end: self.end }
    }
}

impl<Idx> ToCopyable for std::ops::RangeInclusive<Idx>
where
    Idx: Clone + Copy,
{
    type Into = RangeInclusive<Idx>;
    fn into(self) -> Self::Into {
        RangeInclusive {
            start: *self.start(),
            end: *self.end(),
        }
    }
}

impl<Idx> ToCopyable for std::ops::RangeToInclusive<Idx>
where
    Idx: Clone + Copy,
{
    type Into = RangeToInclusive<Idx>;
    fn into(self) -> Self::Into {
        RangeToInclusive { end: self.end }
    }
}

impl ToCopyable for std::ops::RangeFull {
    type Into = RangeFull;
    fn into(self) -> Self::Into {
        RangeFull
    }
}

impl ToCopyable for i32 {
    type Into = i32;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for i64 {
    type Into = i64;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for i128 {
    type Into = i128;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for u32 {
    type Into = u32;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for u64 {
    type Into = u64;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for u128 {
    type Into = u128;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for usize {
    type Into = usize;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for isize {
    type Into = isize;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for u8 {
    type Into = u8;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for i8 {
    type Into = i8;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for u16 {
    type Into = u16;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for i16 {
    type Into = i16;
    fn into(self) -> Self::Into {
        self
    }
}
impl ToCopyable for char {
    type Into = char;
    fn into(self) -> Self::Into {
        self
    }
}

impl<Idx> RangeBound<Idx> for Range<Idx>
where
    Idx: Clone + Copy + PartialOrd,
{
    fn contains(&self, count: &Idx) -> bool {
        self.start <= *count && *count < self.end
    }
}

impl<Idx> RangeBound<Idx> for RangeFrom<Idx>
where
    Idx: Clone + Copy + PartialOrd,
{
    fn contains(&self, count: &Idx) -> bool {
        self.start <= *count
    }
}

impl<Idx> RangeBound<Idx> for RangeTo<Idx>
where
    Idx: Clone + Copy + PartialOrd,
{
    fn contains(&self, count: &Idx) -> bool {
        *count < self.end
    }
}

impl<Idx> RangeBound<Idx> for RangeFull
where
    Idx: Clone + Copy + PartialOrd,
{
    fn contains(&self, _count: &Idx) -> bool {
        true
    }
}

impl<Idx> RangeBound<Idx> for RangeInclusive<Idx>
where
    Idx: Clone + Copy + PartialOrd,
{
    fn contains(&self, count: &Idx) -> bool {
        self.start <= *count && *count <= self.end
    }
}

impl<Idx> RangeBound<Idx> for RangeToInclusive<Idx>
where
    Idx: Clone + Copy + PartialOrd,
{
    fn contains(&self, count: &Idx) -> bool {
        *count <= self.end
    }
}

impl<Idx> RangeBound<Idx> for Idx
where
    Idx: Clone + Copy + PartialEq,
{
    fn contains(&self, count: &Idx) -> bool {
        return *self == *count;
    }
}
