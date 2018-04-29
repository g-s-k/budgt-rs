enum AcctType {
    Credit,
    Debit,
}

/// Represents an account - holding either money or debt.
pub struct Account {
    pub name: String,
    balance: f64,
    holds: f64,
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
    /// let myacct = budgt::Account::new("cool account", 123.45, 2.0, 0.0, false);
    /// ```
    pub fn new(name: &str, balance: f64, holds: f64, rate: f64, negative: bool) -> Account {
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
    /// let myacct = budgt::Account::new("cool account", 123.45, 5.75, 0.0, false);
    /// let real_bal = myacct.current();
    /// ```
    pub fn current(&self) -> f64 {
        self.balance + self.holds * match self.typ {
            AcctType::Credit => -1.0,
            AcctType::Debit => 1.0,
        }
    }

    /// Get the balance of an account with n months of interest applied.
    ///
    /// # Examples
    ///
    /// ```
    /// let myacct = budgt::Account::new("cool account", 100.0, 0.0, 0.02, true);
    /// let future_bal = myacct.future(3);
    /// ```
    pub fn future(&self, n_months: u64) -> f64 {
        self.current() * (1.0 + n_months as f64 * self.rate)
    }
}
