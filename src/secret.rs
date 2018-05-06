use std::ops::{Neg, Not};
use std::cmp::{PartialOrd, PartialEq};

/// Stores a secret.
///
/// A secret is a data structure that might have some evidence.
/// The evidence tells why the value has the value it has.
///
/// For example, the maximum value of a list could be stored as a secret
/// which has as evidence the index where the maximum value can be found.
///
/// The meaning of secrets is interpreted depending on context.
pub struct Secret<E, T> {
    /// The evidence for the value.
    pub evidence: Option<E>,
    /// The value.
    pub value: T,
}

impl<E, T> Neg for Secret<E, T>
    where T: Neg
{
    type Output = Secret<E, T::Output>;
    fn neg(self) -> Secret<E, T::Output> {
        Secret {value: -self.value, evidence: self.evidence}
    }
}

impl<E> Not for Secret<E, bool> {
    type Output = Secret<E, bool>;
    fn not(self) -> Secret<E, bool> {
        Secret {value: !self.value, evidence: self.evidence}
    }
}

impl<E, T> Secret<E, T> {
    /// Checks if secret value is less than value.
    ///
    /// This is a method because Rust does not allow overriding
    /// the output type for comparison operators.
    pub fn lt<U>(self, other: &U) -> Secret<E, bool>
        where T: PartialOrd<U>
    {
        Secret {
            value: self.value.lt(other),
            evidence: self.evidence
        }
    }

    /// Checks if secret value is less or equal than value.
    ///
    /// This is a method because Rust does not allow overriding
    /// the output type for comparison operators.
    pub fn le<U>(self, other: &U) -> Secret<E, bool>
        where T: PartialOrd<U>
    {
        Secret {
            value: self.value.le(other),
            evidence: self.evidence
        }
    }

    /// Checks if secret value is greater than value.
    ///
    /// This is a method because Rust does not allow overriding
    /// the output type for comparison operators.
    pub fn gt<U>(self, other: &U) -> Secret<E, bool>
        where T: PartialOrd<U>
    {
        Secret {
            value: self.value.gt(other),
            evidence: self.evidence
        }
    }

    /// Checks if secret value is greater or equal than value.
    ///
    /// This is a method because Rust does not allow overriding
    /// the output type for comparison operators.
    pub fn ge<U>(self, other: &U) -> Secret<E, bool>
        where T: PartialOrd<U>
    {
        Secret {
            value: self.value.ge(other),
            evidence: self.evidence
        }
    }

    /// Checks if secret value is equal to value.
    ///
    /// This is a method because Rust does not allow overriding
    /// the output type for comparison operators.
    pub fn eq<U>(self, other: &U) -> Secret<E, bool>
        where T: PartialEq<U>
    {
        Secret {
            value: self.value.eq(other),
            evidence: self.evidence
        }
    }

    /// Checks if secret value is not equal to value.
    ///
    /// This is a method because Rust does not allow overriding
    /// the output type for comparison operators.
    pub fn ne<U>(self, other: &U) -> Secret<E, bool>
        where T: PartialEq<U>
    {
        Secret {
            value: self.value.ne(other),
            evidence: self.evidence
        }
    }
}
