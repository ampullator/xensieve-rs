use std::fmt;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;
use std::rc::Rc;

mod parser;
mod util;

//------------------------------------------------------------------------------

pub trait UnsignedInt:
    Copy
    + Ord
    + Eq
    + fmt::Display
    + fmt::Debug
    + std::str::FromStr
    + TryFrom<u128>
    + Into<u128>
    + std::ops::Rem<Output = Self>
{
}

impl<T> UnsignedInt for T where
    T: Copy
        + Ord
        + Eq
        + fmt::Display
        + fmt::Debug
        + std::str::FromStr
        + TryFrom<u128>
        + Into<u128>
        + std::ops::Rem<Output = Self>
{
}

//------------------------------------------------------------------------------

/// Container of integer values for the modulus and the shift of a Residual class.
///
/// # Fields
/// * `modulus` - The modulus.
/// * `shift` - The shift.
///
#[derive(Clone, Debug, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Residual<T = u64> {
    modulus: T,
    shift: T,
}

impl<T> Residual<T>
where
    T: UnsignedInt,
{
    pub(crate) fn new(modulus: T, mut shift: T) -> Self {
        if modulus.into() == 0 {
            shift = modulus;
        } else {
            shift = shift % modulus;
        }
        Self { modulus, shift }
    }

    /// Return `true` if the value is contained with this Sieve.
    ///
    pub(crate) fn contains(&self, value: i128) -> bool {
        let modulus: u128 = self.modulus.into();
        if modulus == 0 {
            return false;
        }
        let shift = self.shift.into() % modulus;
        let value_mod = if value >= 0 {
            (value as u128) % modulus
        } else {
            let mag = value.unsigned_abs() % modulus;
            if mag == 0 {
                0
            } else {
                modulus - mag
            }
        };
        value_mod == shift
    }
}

impl<T> fmt::Display for Residual<T>
where
    T: UnsignedInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let n = if self.invert {String::from("!")} else {String::new()};
        write!(f, "{}@{}", self.modulus, self.shift)
    }
}

impl<T> BitAnd for Residual<T>
where
    T: UnsignedInt,
{
    type Output = Residual<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let (m, s) = util::intersection(self.modulus, rhs.modulus, self.shift, rhs.shift).unwrap();
        Self::new(m, s)
    }
}

//------------------------------------------------------------------------------

/// A node in the graph of Residuals combined by logical operations.
///
#[derive(Clone, Debug)]
pub(crate) enum SieveNode<T = u64> {
    Unit(Residual<T>),
    Intersection(Rc<SieveNode<T>>, Rc<SieveNode<T>>),
    Union(Rc<SieveNode<T>>, Rc<SieveNode<T>>),
    SymmetricDifference(Rc<SieveNode<T>>, Rc<SieveNode<T>>),
    Inversion(Rc<SieveNode<T>>),
}

impl<T> fmt::Display for SieveNode<T>
where
    T: UnsignedInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = match self {
            SieveNode::Unit(residual) => residual.to_string(),
            SieveNode::Intersection(lhs, rhs) => {
                let lhs_str = lhs.to_string();
                let rhs_str = rhs.to_string();
                format!("{lhs_str}&{rhs_str}")
            }
            SieveNode::Union(lhs, rhs) => {
                let lhs_str = lhs.to_string();
                let rhs_str = rhs.to_string();
                format!("{lhs_str}|{rhs_str}")
            }
            SieveNode::SymmetricDifference(lhs, rhs) => {
                let lhs_str = lhs.to_string();
                let rhs_str = rhs.to_string();
                format!("{lhs_str}^{rhs_str}")
            }
            SieveNode::Inversion(part) => {
                let r = part.to_string();
                format!("!({r})")
            }
        };
        write!(f, "{}", s)
    }
}

impl<T> SieveNode<T>
where
    T: UnsignedInt,
{
    /// Return `true` if the values is contained within this Sieve.
    ///
    pub fn contains(&self, value: i128) -> bool {
        match self {
            SieveNode::Unit(residual) => residual.contains(value),
            SieveNode::Intersection(lhs, rhs) => lhs.contains(value) && rhs.contains(value),
            SieveNode::Union(lhs, rhs) => lhs.contains(value) || rhs.contains(value),
            SieveNode::SymmetricDifference(lhs, rhs) => lhs.contains(value) ^ rhs.contains(value),
            SieveNode::Inversion(part) => !part.contains(value),
        }
    }
}

//------------------------------------------------------------------------------

/// The representation of a Xenakis Sieve, constructed from a string notation of one or more Residual classes combined with logical operators. This Rust implementation follows the Python implementation in Ariza (2005), with significant performance and interface enhancements: https://direct.mit.edu/comj/article/29/2/40/93957
#[derive(Clone, Debug)]
pub struct Sieve<T = u64> {
    root: Rc<SieveNode<T>>,
}

impl<T> BitAnd for Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::Intersection(self.root, rhs.root)),
        }
    }
}

impl<T> BitAnd for &Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::Intersection(self.root.clone(), rhs.root.clone())),
        }
    }
}

impl<T> BitOr for Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::Union(self.root, rhs.root)),
        }
    }
}

impl<T> BitOr for &Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::Union(self.root.clone(), rhs.root.clone())),
        }
    }
}

impl<T> BitXor for Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::SymmetricDifference(self.root, rhs.root)),
        }
    }
}

impl<T> BitXor for &Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::SymmetricDifference(
                self.root.clone(),
                rhs.root.clone(),
            )),
        }
    }
}

impl<T> Not for Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn not(self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::Inversion(self.root)),
        }
    }
}

impl<T> Not for &Sieve<T>
where
    T: UnsignedInt,
{
    type Output = Sieve<T>;

    fn not(self) -> Self::Output {
        Sieve {
            root: Rc::new(SieveNode::Inversion(self.root.clone())),
        }
    }
}

impl<T> fmt::Display for Sieve<T>
where
    T: UnsignedInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sieve{{{}}}", self.root)
    }
}

impl<T> Sieve<T>
where
    T: UnsignedInt,
{
    /// Construct a Xenakis Sieve from a string representation for an explicit unsigned integer type.
    pub fn new_typed(value: &str) -> Self {
        let mut stack: Vec<Self> = Vec::new();
        for token in parser::infix_to_postfix(value).expect("Parsing failure") {
            match token.as_str() {
                "!" => {
                    let s = stack.pop().expect("Invalid syntax: missing operand");
                    stack.push(!s);
                }
                "&" => {
                    let right = stack.pop().expect("Invalid syntax: missing operand");
                    let left = stack.pop().expect("Invalid syntax: missing operand");
                    stack.push(left & right);
                }
                "^" => {
                    let right = stack.pop().expect("Invalid syntax: missing operand");
                    let left = stack.pop().expect("Invalid syntax: missing operand");
                    stack.push(left ^ right);
                }
                "|" => {
                    let right = stack.pop().expect("Invalid syntax: missing operand");
                    let left = stack.pop().expect("Invalid syntax: missing operand");
                    stack.push(left | right);
                }
                operand => {
                    let (m, s) = parser::residual_to_ints::<T>(operand)
                        .expect("Invalid syntax: cannot parse Residual");
                    let r = Residual::new(m, s);
                    let s = Self {
                        root: Rc::new(SieveNode::Unit(r)),
                    };
                    stack.push(s);
                }
            }
        }
        stack.pop().expect("Invalid syntax: no result")
    }

    /// Return `true` if the value is contained with this Sieve.
    ///
    /// ```
    /// let s = xensieve::Sieve::new("3@0 & 5@0");
    /// assert_eq!(s.contains(15), true);
    /// assert_eq!(s.contains(16), false);
    /// assert_eq!(s.contains(30), true);
    /// ```
    pub fn contains(&self, value: i128) -> bool {
        self.root.contains(value)
    }

    /// For the iterator provided as an input, iterate the subset of values that are contained within the sieve.
    /// ```
    /// let s = xensieve::Sieve::new("3@0|4@0");
    /// assert_eq!(s.iter_value(0..=12).collect::<Vec<_>>(), vec![0, 3, 4, 6, 8, 9, 12])
    /// ````
    pub fn iter_value(
        &self,
        iterator: impl Iterator<Item = i128>,
    ) -> IterValue<impl Iterator<Item = i128>, T> {
        // NOTE: do not want to clone self here...
        IterValue {
            iterator,
            sieve_node: self.root.clone(),
        }
    }

    /// For the iterator provided as an input, iterate the Boolean status of contained.
    /// ```
    /// let s = xensieve::Sieve::new("3@0|4@0");
    /// assert_eq!(s.iter_state(0..=6).collect::<Vec<_>>(), vec![true, false, false, true, true, false, true])
    /// ````
    pub fn iter_state(
        &self,
        iterator: impl Iterator<Item = i128>,
    ) -> IterState<impl Iterator<Item = i128>, T> {
        IterState {
            iterator,
            sieve_node: self.root.clone(),
        }
    }

    /// Iterate over integer intervals between values in the sieve.
    /// ```
    /// let s = xensieve::Sieve::new("3@0|4@0");
    /// assert_eq!(s.iter_interval(0..=12).collect::<Vec<_>>(), vec![3, 1, 2, 2, 1, 3])
    /// ````
    pub fn iter_interval(
        &self,
        iterator: impl Iterator<Item = i128>,
    ) -> IterInterval<impl Iterator<Item = i128>, T> {
        IterInterval {
            iterator,
            sieve_node: self.root.clone(),
            last: PositionLast::Init,
        }
    }
}

impl Sieve<u64> {
    /// Construct a Xenakis Sieve from a string representation.
    ///
    /// ```
    /// let s = xensieve::Sieve::new("3@0|5@1");
    /// assert_eq!(s.iter_value(0..15).collect::<Vec<_>>(), vec![0, 1, 3, 6, 9, 11, 12])
    /// ````
    pub fn new(value: &str) -> Self {
        Self::new_typed(value)
    }
}

//------------------------------------------------------------------------------

/// The iterator returned by `iter_value`.
/// ```
/// let s = xensieve::Sieve::new("3@0|4@0");
/// let mut s_iter = s.iter_value(17..);
/// assert_eq!(s_iter.next().unwrap(), 18);
/// assert_eq!(s_iter.next().unwrap(), 20);
/// ```
pub struct IterValue<I, T = u64>
where
    I: Iterator<Item = i128>,
{
    iterator: I,
    sieve_node: Rc<SieveNode<T>>,
}

impl<I, T> Iterator for IterValue<I, T>
where
    I: Iterator<Item = i128>,
    T: UnsignedInt,
{
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .by_ref()
            .find(|&p| self.sieve_node.contains(p))
    }
}

//------------------------------------------------------------------------------

/// The iterator returned by `iter_state`.
/// ```
/// let s = xensieve::Sieve::new("3@0|4@0");
/// let mut s_iter = s.iter_state(17..);
/// assert_eq!(s_iter.next().unwrap(), false);
/// assert_eq!(s_iter.next().unwrap(), true);
/// assert_eq!(s_iter.next().unwrap(), false);
/// assert_eq!(s_iter.next().unwrap(), true);
/// ```
pub struct IterState<I, T = u64>
where
    I: Iterator<Item = i128>,
{
    iterator: I,
    sieve_node: Rc<SieveNode<T>>,
}

impl<I, T> Iterator for IterState<I, T>
where
    I: Iterator<Item = i128>, // the values returned by iterator
    T: UnsignedInt,
{
    type Item = bool; // the value returned

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(p) => Some(self.sieve_node.contains(p)),
            None => None,
        }
    }
}

//------------------------------------------------------------------------------

enum PositionLast {
    Init,
    Value(i128),
}

/// The iterator returned by `iter_interval`.
/// ```
/// let s = xensieve::Sieve::new("3@0|4@0");
/// let mut s_iter = s.iter_interval(17..);
/// assert_eq!(s_iter.next().unwrap(), 2);
/// assert_eq!(s_iter.next().unwrap(), 1);
/// assert_eq!(s_iter.next().unwrap(), 3);
/// ```
pub struct IterInterval<I, T = u64>
where
    I: Iterator<Item = i128>,
{
    iterator: I,
    sieve_node: Rc<SieveNode<T>>,
    last: PositionLast,
}

impl<I, T> Iterator for IterInterval<I, T>
where
    I: Iterator<Item = i128>,
    T: UnsignedInt,
{
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.iterator.by_ref() {
            // while let Some(p) = self.iterator.next() {
            if self.sieve_node.contains(p) {
                match self.last {
                    PositionLast::Init => {
                        // drop the first value
                        self.last = PositionLast::Value(p);
                        continue;
                    }
                    PositionLast::Value(last) => {
                        let post = p - last;
                        self.last = PositionLast::Value(p);
                        return Some(post);
                    }
                }
            }
        }
        None
    }
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_residual_a() {
        let r1 = Residual::<u64>::new(3, 0);
        assert_eq!(r1.to_string(), String::from("3@0"));
    }

    #[test]
    fn test_residual_b() {
        let r1 = Residual::<u64>::new(0, 2);
        assert_eq!(r1.to_string(), "0@0");
    }

    //--------------------------------------------------------------------------
    #[test]
    fn test_residual_to_string_a() {
        let r1 = Residual::<u64>::new(3, 0);
        assert_eq!(r1.to_string(), "3@0");
    }

    #[test]
    fn test_residual_to_string_b() {
        let r1 = Residual::<u64>::new(8, 3);
        assert_eq!(r1.to_string(), "8@3");
    }

    #[test]
    fn test_residual_to_string_c() {
        let r1 = Residual::<u64>::new(5, 8);
        assert_eq!(r1.to_string(), "5@3");
    }

    #[test]
    fn test_residual_to_string_d() {
        let r1 = Residual::<u64>::new(5, 9);
        assert_eq!(r1.to_string(), "5@4");
    }

    #[test]
    fn test_residual_to_string_e() {
        let r1 = Residual::<u64>::new(5, 10);
        assert_eq!(r1.to_string(), "5@0");
    }

    //--------------------------------------------------------------------------

    // #[test]
    // fn test_residual_not_a() {
    //     let r1 = Residual::<u64>::new(5, 10);
    //     assert_eq!(r1.to_string(), String::from("!5@0"));
    //     let r2 = !r1;
    //     assert_eq!(r2.to_string(), "5@0");
    //     let r3 = !r2;
    //     assert_eq!(r3.to_string(), "!5@0");
    // }

    #[test]
    fn test_residual_eq_a() {
        let r1 = Residual::<u64>::new(5, 2);
        let r2 = Residual::<u64>::new(5, 3);
        assert_eq!(r1 == r2, false);
        assert_eq!(r1 != r2, true);
    }

    #[test]
    fn test_residual_eq_b() {
        let r1 = Residual::<u64>::new(5, 2);
        let r2 = Residual::<u64>::new(5, 2);
        assert_eq!(r1 == r2, true);
        assert_eq!(r1 != r2, false);
    }

    #[test]
    fn test_residual_ord_a() {
        let r1 = Residual::<u64>::new(5, 2);
        let r2 = Residual::<u64>::new(5, 3);
        assert!(r1 < r2);
    }

    #[test]
    fn test_residual_ord_b() {
        let r1 = Residual::<u64>::new(2, 3);
        let r2 = Residual::<u64>::new(5, 3);
        assert!(r1 < r2);
    }

    #[test]
    fn test_residual_ord_c() {
        let r1 = Residual::<u64>::new(5, 3);
        let r2 = Residual::<u64>::new(5, 3);
        assert!(r1 == r2);
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_residual_bitand_a() {
        let r1 = Residual::<u64>::new(4, 0);
        let r2 = Residual::<u64>::new(3, 0);
        assert_eq!((r1 & r2).to_string(), "12@0");
    }

    #[test]
    fn test_residual_bitand_b() {
        let r1 = Residual::<u64>::new(4, 0);
        let r2 = Residual::<u64>::new(3, 1);
        assert_eq!((r1 & r2).to_string(), "12@4");
    }

    #[test]
    fn test_residual_bitand_c() {
        let r1 = Residual::<u64>::new(5, 2);
        let r2 = Residual::<u64>::new(10, 3);
        assert_eq!((r1 & r2).to_string(), "0@0");
    }

    #[test]
    fn test_residual_bitand_d() {
        let r1 = Residual::<u64>::new(3, 2);
        let r2 = Residual::<u64>::new(3, 1);
        assert_eq!((r1 & r2).to_string(), "0@0");
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_residual_contains_a() {
        let r1 = Residual::<u64>::new(3, 0);
        assert_eq!(r1.contains(-3), true);
        assert_eq!(r1.contains(-2), false);
        assert_eq!(r1.contains(-1), false);
        assert_eq!(r1.contains(0), true);
        assert_eq!(r1.contains(1), false);
        assert_eq!(r1.contains(2), false);
        assert_eq!(r1.contains(3), true);
        assert_eq!(r1.contains(4), false);
        assert_eq!(r1.contains(5), false);
    }

    #[test]
    fn test_residual_contains_b() {
        let r1 = Residual::<u64>::new(0, 0);
        assert_eq!(r1.contains(-2), false);
        assert_eq!(r1.contains(-1), false);
        assert_eq!(r1.contains(0), false);
        assert_eq!(r1.contains(1), false);
        assert_eq!(r1.contains(2), false);
        assert_eq!(r1.contains(3), false);
    }

    #[test]
    fn test_residual_contains_c() {
        let r1 = Residual::<u64>::new(3, 1);
        assert_eq!(r1.contains(-3), false);
        assert_eq!(r1.contains(-2), true);
        assert_eq!(r1.contains(-1), false);
        assert_eq!(r1.contains(0), false);
        assert_eq!(r1.contains(1), true);
        assert_eq!(r1.contains(2), false);
        assert_eq!(r1.contains(3), false);
        assert_eq!(r1.contains(4), true);
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_sieve_new_a() {
        let s1 = Sieve::new("3@1");
        assert_eq!(s1.to_string(), "Sieve{3@1}");
    }

    #[test]
    fn test_sieve_new_b() {
        let s1 = Sieve::new("3@4");
        assert_eq!(s1.to_string(), "Sieve{3@1}");
    }

    #[test]
    fn test_sieve_new_c() {
        let s1 = Sieve::new("5@5");
        assert_eq!(s1.to_string(), "Sieve{5@0}");
    }

    #[test]
    fn test_sieve_new_d() {
        let s1 = Sieve::new("0@5");
        assert_eq!(s1.to_string(), "Sieve{0@0}");
    }

    #[test]
    fn test_sieve_contains_a() {
        let r1 = Residual::<u64>::new(3, 0);
        let s1 = SieveNode::Unit(r1);

        let pos = vec![-3, -2, -1, 0, 1];
        let val = vec![true, false, false, true, false];
        for (p, b) in pos.iter().zip(val.iter()) {
            assert_eq!(s1.contains(*p), *b);
        }
    }

    #[test]
    fn test_sieve_contains_b() {
        let r1 = Residual::<u64>::new(3, 0);
        let r2 = Residual::<u64>::new(3, 1);
        let s1 = SieveNode::Union(Rc::new(SieveNode::Unit(r1)), Rc::new(SieveNode::Unit(r2)));

        assert_eq!(s1.contains(-2), true);
        assert_eq!(s1.contains(-1), false);
        assert_eq!(s1.contains(0), true);
        assert_eq!(s1.contains(1), true);
        assert_eq!(s1.contains(2), false);
        assert_eq!(s1.contains(3), true);
        assert_eq!(s1.contains(4), true);
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_sieve_operators_a() {
        let s1 = Sieve::new("3@1");
        let s2 = Sieve::new("4@0");
        let s3 = s1 | s2;

        assert_eq!(s3.to_string(), "Sieve{3@1|4@0}");
    }

    #[test]
    fn test_sieve_operators_b() {
        let s1 = Sieve::new("3@1");
        let s2 = Sieve::new("4@0");
        let s3 = &s1 | &s2;

        assert_eq!(s3.to_string(), "Sieve{3@1|4@0}");

        if let SieveNode::Union(lhs, rhs) = s3.root.as_ref() {
            assert!(Rc::ptr_eq(lhs, &s1.root));
            assert!(Rc::ptr_eq(rhs, &s2.root));
        } else {
            panic!("Expected union node");
        }
    }

    #[test]
    fn test_sieve_operators_c() {
        let s1 = Sieve::new("3@1");
        let s2 = Sieve::new("4@0");
        let s3 = &s1 & &s2;

        assert_eq!(s3.to_string(), "Sieve{3@1&4@0}");
    }

    #[test]
    fn test_sieve_operators_d() {
        let s1 = Sieve::new("3@1");
        let s2 = Sieve::new("4@0");
        let s3 = &s1 ^ &s2;

        assert_eq!(s3.to_string(), "Sieve{3@1^4@0}");
    }

    #[test]
    fn test_sieve_operators_e() {
        let s1 = Sieve::new("3@1");
        let s3 = !&s1;
        assert_eq!(s3.to_string(), "Sieve{!(3@1)}");

        if let SieveNode::Inversion(node) = s3.root.as_ref() {
            assert!(Rc::ptr_eq(node, &s1.root));
        } else {
            panic!("Expected inversion node");
        }
    }
}
