mod money;

pub use money::Money;

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
    /// let myacct = budgt::Account::new("cool account", budgt::Money(12345), budgt::Money(200), 0., false);
    /// ```
    pub fn new(name: &str, balance: Money, holds: Money, rate: f64, negative: bool) -> Account {
        let typ = if negative {
            AcctType::Debit
        } else {
            AcctType::Credit
        };

        Account {
            name: name.to_string(),
            balance,
            holds,
            rate,
            typ,
        }
    }

    /// Get the current balance of an account.
    ///
    /// # Examples
    ///
    /// ```
    /// let myacct = budgt::Account::new("cool account", budgt::Money(12345), budgt::Money(575), 0., false);
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
    /// let myacct = budgt::Account::new("cool account", budgt::Money(10000), budgt::Money(0), 0.02, true);
    /// let future_bal = myacct.future(3);
    /// ```
    pub fn future(&self, n_months: u64) -> Money {
        Money((self.current().0 as f64 * (1.0 + n_months as f64 * self.rate)) as i64)
    }

    /// Take a snapshot of an account's value.
    ///
    /// # Examples
    /// ```
    /// let acct = budgt::Account::new("name", budgt::Money(10), budgt::Money(5), 0.0, false);
    /// assert_eq!(acct.take_snapshot(), budgt::AccountSnapshot::new("name", 5));
    /// ```
    pub fn take_snapshot(&self) -> AccountSnapshot {
        AccountSnapshot::new(&self.name, self.current())
    }
}

/// A container for information about a given account at a given time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountSnapshot(String, Money);

impl AccountSnapshot {
    /// Create a new AccountSnapshot.
    ///
    /// # Examples
    ///
    ///     let snap = budgt::AccountSnapshot::new("abcdefg", 12345);
    pub fn new<T>(name: &str, balance: T) -> Self where Money: From<T> {
        AccountSnapshot(name.to_owned(), Money::from(balance))
    }
}

/// Represents one concrete instance of a transaction.
///
/// # Examples
/// ```
/// let ti = budgt::TransactionInstance::default()
///     .name("foo")
///     .amount(123456)
///     .source(budgt::AccountSnapshot::new("bar", 999))
///     .dest(budgt::AccountSnapshot::new("baz", -27));
/// ```
#[derive(Default, Clone, Debug)]
pub struct TransactionInstance {
    date: String,
    name: String,
    amount: Money,
    source: Option<AccountSnapshot>,
    dest: Option<AccountSnapshot>,
}

impl TransactionInstance {
    /// Create a new TransactionInstance.
    ///
    /// # Examples
    /// ```
    /// let ti = budgt::TransactionInstance::new("foo", budgt::Money(1000), Some(budgt::AccountSnapshot::new("bar", 12345)), Some(budgt::AccountSnapshot::new("baz", 20231)));
    /// ```
    ///
    /// ```
    /// let ti = budgt::TransactionInstance::new("foo", budgt::Money(1000), Some(budgt::AccountSnapshot::new("bar", 12345)), None);
    /// ```
    ///
    /// ```
    /// let ti = budgt::TransactionInstance::new("foo", budgt::Money(1000), None, Some(budgt::AccountSnapshot::new("baz", 3099)));
    /// ```
    pub fn new(
        name: &str,
        amount: Money,
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

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn amount<T>(mut self, amt: T) -> Self where Money: From<T> {
        self.amount = Money::from(amt);
        self
    }

    pub fn source(mut self, acct: AccountSnapshot) -> Self {
        self.source = Some(acct);
        self
    }

    pub fn dest(mut self, acct: AccountSnapshot) -> Self {
        self.dest = Some(acct);
        self
    }

    /// Format a transaction instance as a series of strings.
    pub fn fmt_table(&self) -> Vec<String> {
        let mut v = Vec::with_capacity(7);

        v.push(self.date.clone());
        v.push(self.name.clone());
        v.push(self.amount.to_string());

        if let Some(ref acct) = self.source {
            v.push(acct.0.clone());
            v.push(acct.1.to_string())
        } else {
            v.push("".to_owned());
            v.push("".to_owned());
        };

        if let Some(ref acct) = self.dest {
            v.push(acct.0.clone());
            v.push(acct.1.to_string())
        } else {
            v.push("".to_owned());
            v.push("".to_owned());
        };

        v
    }
}
