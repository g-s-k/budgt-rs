use std::{fmt, ops};

#[derive(Clone, Copy, Default, Debug)]
pub struct Money(pub i64);

impl ops::Deref for Money {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl ops::DerefMut for Money {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl<T> From<T> for Money
where
    i64: From<T>,
{
    fn from(val: T) -> Self {
        Money(i64::from(val))
    }
}

impl<T> ops::Add<T> for Money
where
    Money: From<T>,
{
    type Output = Money;

    fn add(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 + r.0)
    }
}

impl<T> ops::Sub<T> for Money
where
    Money: From<T>,
{
    type Output = Money;

    fn sub(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 - r.0)
    }
}

impl<T> ops::Mul<T> for Money
where
    Money: From<T>,
{
    type Output = Money;

    fn mul(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 * r.0)
    }
}

impl<T> ops::Div<T> for Money
where
    Money: From<T>,
{
    type Output = Money;

    fn div(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 / r.0)
    }
}

impl<T> ops::Rem<T> for Money
where
    Money: From<T>,
{
    type Output = Money;

    fn rem(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 % r.0)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}.{:02}", self.0 / 100, self.0 % 100)
    }
}
