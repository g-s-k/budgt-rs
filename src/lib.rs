use std::{fmt, ops};

#[derive(Clone, Copy)]
pub struct Money(i64);

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

impl<T> From<T> for Money where i64: From<T> {
    fn from(val: T) -> Self {
        Money(i64::from(val))
    }
}

impl<T> ops::Add<T> for Money where Money: From<T> {
    type Output = Money;

    fn add(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 + r.0)
    }
}

impl<T> ops::Sub<T> for Money
where T: From<i64> + ops::Sub,
      i64: From<<T as ops::Sub>::Output>
{
    type Output = Money;

    fn sub(self, rhs: T) -> Money {
        Money(i64::from(T::from(self.0) - rhs))
    }
}

impl<T> ops::Mul<T> for Money where Money: From<T> {
    type Output = Money;

    fn mul(self, rhs: T) -> Money {
        let r = Money::from(rhs);
        Money(self.0 * r.0)
    }
}

impl<T> ops::Div<T> for Money
where T: From<i64> + ops::Div,
      i64: From<<T as ops::Div>::Output>
{
    type Output = Money;

    fn div(self, rhs: T) -> Money {
        Money(i64::from(T::from(self.0) / rhs))
    }
}

impl<T> ops::Rem<T> for Money
where T: From<i64> + ops::Rem,
      i64: From<<T as ops::Rem>::Output>
{
    type Output = Money;

    fn rem(self, rhs: T) -> Money {
        Money(i64::from(T::from(self.0) % rhs))
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}.{:02}", *self / 100i64, *self % 100i64)
    }
}

enum AcctType {
    Credit,
    Debit,
}

/// Represents an account - holding either money or debt.
pub struct Account {
    pub name: String,
    balance: Money,
    holds: Money,
    rate: f64,
    typ: AcctType,
}

impl Account {
    /// Create a new Account.
    ///
    /// Each field must be initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// let myacct = budgt::Account::new("cool account", 12345, 200, 0., false);
    /// ```
    pub fn new(name: &str, balance: i64, holds: i64, rate: f64, negative: bool) -> Account {
        let typ = if negative {
            AcctType::Debit
        } else {
            AcctType::Credit
        };

        Account {
            name: name.to_string(),
            balance: Money(balance),
            holds: Money(holds),
            rate,
            typ,
        }
    }

    /// Get the current balance of an account.
    ///
    /// # Examples
    ///
    /// ```
    /// let myacct = budgt::Account::new("cool account", 12345, 575, 0., false);
    /// let real_bal = myacct.current();
    /// ```
    pub fn current(&self) -> Money {
        self.balance + self.holds * match self.typ {
            AcctType::Credit => -1,
            AcctType::Debit => 1,
        }
    }

    /// Get the balance of an account with n months of interest applied.
    ///
    /// # Examples
    ///
    /// ```
    /// let myacct = budgt::Account::new("cool account", 10000, 0, 0.02, true);
    /// let future_bal = myacct.future(3);
    /// ```
    pub fn future(&self, n_months: u64) -> Money {
        Money((self.current().0 as f64 * (1.0 + n_months as f64 * self.rate)) as i64)
    }
}

/// A container for information about a given account at a given time.
pub struct AccountSnapshot(pub String, pub i64);

/// Represents one concrete instance of a transaction.
pub struct TransactionInstance {
    date: String,
    name: String,
    amount: i64,
    source: Option<AccountSnapshot>,
    dest: Option<AccountSnapshot>,
}

impl TransactionInstance {
    /// Create a new TransactionInstance.
    ///
    /// # Examples
    /// ```
    /// let ti = budgt::TransactionInstance::new("foo", 1000, Some(budgt::AccountSnapshot("bar".to_string(), 12345)), Some(budgt::AccountSnapshot("baz".to_string(), 20231)));
    /// ```
    ///
    /// ```
    /// let ti = budgt::TransactionInstance::new("foo", 1000, Some(budgt::AccountSnapshot("bar".to_string(), 12345)), None);
    /// ```
    ///
    /// ```
    /// let ti = budgt::TransactionInstance::new("foo", 1000, None, Some(budgt::AccountSnapshot("baz".to_string(), 3099)));
    /// ```
    pub fn new(
        name: &str,
        amount: i64,
        source: Option<AccountSnapshot>,
        dest: Option<AccountSnapshot>,
    ) -> TransactionInstance {
        TransactionInstance {
            date: "".to_string(),
            name: name.to_string(),
            amount,
            source,
            dest,
        }
    }

    /// Format a transaction instance as a series of strings.
    pub fn fmt_table(&self) -> Vec<String> {
        vec![
            self.date.clone(),
            self.name.clone(),
            self.amount.to_string(),
            match self.source {
                Some(ref acct) => acct.0.clone(),
                None => "".to_string(),
            },
            if let Some(ref acct) = self.source {
                acct.1.to_string()
            } else {
                "".to_string()
            },
            match self.dest {
                Some(ref acct) => acct.0.clone(),
                None => "".to_string(),
            },
            if let Some(ref acct) = self.dest {
                acct.1.to_string()
            } else {
                "".to_string()
            },
        ]
    }
}
