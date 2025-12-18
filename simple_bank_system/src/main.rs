#[derive(Debug)]
enum AccountType {
    Current,
    Savings,
}

impl AccountType {
    fn interest_rate(&self) -> f64 {
        match self {
            AccountType::Current => 0.00,
            AccountType::Savings => 0.04,
        }
    }
}

#[derive(Debug)]
struct BankAccount {
    name: String,
    balance: f64,
    account_type: AccountType,
}

impl BankAccount {
    fn new(name: &str, balance: f64, account_type: AccountType) -> Self {
        Self { name: name.to_string(), balance, account_type }
    }

    fn deposit(&mut self, amount: f64) {
        if amount < 0.0 {
            println!("Invalid Deposit Amount, Must be Positive");
            return;
        }
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance < amount {
            println!("Insufficient Funds");
            return;
        } 
        self.balance -= amount;
    }

    fn apply_interest(&mut self) {
        let rate = self.account_type.interest_rate();
        self.balance *= 1.0 + rate;
    }

    fn display(&self) {
        println!("Bank Account Name: {}, Balance: {}", self.name, self.balance);
    }
}

fn main() {
    let mut jsmith = BankAccount::new("John Smith", 250.00, AccountType::Current);
    dbg!(&jsmith);

    jsmith.deposit(50.00);
    dbg!(&jsmith);

    jsmith.withdraw(325.50);

    jsmith.display();

    let mut jdoe = BankAccount::new("Jane Doe", 5000.00, AccountType::Savings);
    jdoe.apply_interest();
    jdoe.display();
    jdoe.deposit(500.00);
    jdoe.apply_interest();
    jdoe.display();
}
