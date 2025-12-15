#[derive(Debug)]
struct Account {
    name: String,
    balance: f64,
}

impl Account {
    fn new(name: &str, balance: f64) -> Self {
        Self { name: name.to_string(), balance }
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

    fn display(&self) {
        println!("Account Name: {}, Balance: {}", self.name, self.balance);
    }
}

fn main() {
    let mut jsmith = Account::new("John Smith", 250.00);
    dbg!(&jsmith);

    jsmith.deposit(50.00);
    dbg!(&jsmith);

    jsmith.withdraw(325.50);

    jsmith.display();
}
