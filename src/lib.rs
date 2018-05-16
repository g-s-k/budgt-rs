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

/// A container for information about a given account at a given time.
struct AccountSnapshot(String, f64);

/// Represents one concrete instance of a transaction.
pub struct TransactionInstance {
    name: String,
    date: String,
    amount: f64,
    source: Option<AccountSnapshot>,
    dest: Option<AccountSnapshot>,
}

impl TransactionInstance {

    /// Create a new TransactionInstance.
    pub fn new(
        name: &str,
        amount: f64,
        source: &str,
        s_balance: f64,
        dest: &str,
        d_balance: f64,
    ) -> TransactionInstance {
        let source = match source {
            "" => None,
            name => Some(AccountSnapshot(name.to_string(), s_balance)),
        };
        let dest = match dest {
            "" => None,
            name => Some(AccountSnapshot(name.to_string(), d_balance)),
        };

        TransactionInstance {
            name: name.to_string(),
            date: "".to_string(),
            amount,
            source,
            dest,
        }
    }

    /// Format a transaction instance as a series of strings.
    pub fn fmt_table(&self) -> Vec<String> {
    vec![
        self.name.clone(),
        format!("{:8}", self.amount),

        match self.source {
            Some(ref acct) => acct.0.clone(),
            None => "".to_string()
        },

        if let Some(ref acct) = self.source {
            format!("{:8}", acct.1)
        } else {
            "".to_string()
        },

        match self.dest {
            Some(ref acct) => acct.0.clone(),
            None => "".to_string()
        },

        if let Some(ref acct) = self.dest {
            format!("{:8}", acct.1)
        } else {
            "".to_string()
        }
    ]

    }
}
