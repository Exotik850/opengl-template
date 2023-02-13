use std::iter::Zip;

pub(crate) mod attribute;
pub(crate) mod bufferable;
pub(crate) mod vertex;

pub trait SliceZip<'a, A, B> {
    type Type;
    fn slice(left: Self::Type, right: &[B]) -> &'a [(&'a A, &'a B)];
}

// impl<'a, A, B> SliceZip<'a, A, B> for &[A] {
// type Type = &'a [A];
// fn slice(left: Self::Type, right: &[B]) -> &'a [(&'a A, &'a B)] {
//     assert_eq!(left.len(), right.len());
//     let mut out = Vec::with_capacity(left.len());
//     for i in 0..left.len() {
//         out.push((&left[i], &right[i]));
//     }
//     out.as_slice()
// }
// }
