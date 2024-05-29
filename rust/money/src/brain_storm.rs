#[allow(dead_code)]
#[allow(unused_variables)]

struct Sales {
    expense: Vec<Expense>,
    income: Income,
}

impl Sales {
    fn new() -> Self {
        Self {
            expense: vec![],
            income: Income::new(),
        }
    }

    fn from(expense: Vec<Expense>, income: Income) -> Self {
        Self { expense, income }
    }

    fn get_total_expense(&self) -> i32 {
        use Expense::*;
        let result: i32 = self
            .expense
            .iter()
            .map(|e| match e {
                Personal(e) => e,
                Cost(e) => e,
                Tax(e) => e,
                Allowance(e) => e,
                Salary(e) => e,
                Wage(e) => e,
            })
            .sum();
        result
    }

    fn get_total_income(&self) -> i32 {
        // self.income.client * self.income.cost
        0
    }
}

enum Expense {
    Personal(i32),
    Cost(i32),
    Tax(i32),
    Allowance(i32),
    Salary(i32),
    Wage(i32),
}

struct Income {
    client: Vec<Client>,
    cost: i32,
}

#[derive(Debug, PartialEq)]
enum Client {
    Prospect(i32),
    New(i32),
    Re(i32),
    Leave(i32),
}

impl Client {
    fn increase(&self, count: i32) -> Self {
        use Client::*;
        match self {
            Prospect(e) => Client::Prospect(e + count),
            Re(e) => Client::Re(e + count),
            New(e) => Client::New(e + count),
            Leave(e) => Client::Leave(e + count),
        }
    }

    fn decrease(&self, count: i32) -> Self {
        use Client::*;
        match self {
            Prospect(e) => Client::Prospect(e - count),
            Re(e) => Client::Re(e - count),
            New(e) => Client::New(e - count),
            Leave(e) => Client::Leave(e - count),
        }
    }
}

impl Income {
    fn new() -> Self {
        Self {
            client: vec![],
            cost: 0,
        }
    }

    fn from(client: Vec<Client>, cost: i32) -> Self {
        Self { client, cost }
    }

    fn client(&self) -> i32 {
        use Client::*;
        self.client
            .iter()
            .map(|c| match c {
                Prospect(i) => i,
                Re(i) => i,
                New(i) => i,
                Leave(i) => i,
            })
            .sum()
    }
}

enum Sns {
    YouTube,
    Instagram,
    Thread,
}

enum Sellable {
    Product(Expense),
    Service(Expense),
}

enum Business {
    Individual,
    Corporation,
}

struct Insurance {
    expense: Expense,
}

struct FinantialHealth {
    insurance: Insurance,
    expense: Expense,
    income: Income,
}

enum Opratable {
    Op,
    NoOp,
}

#[cfg(test)]
mod tests {
    use super::*; // Used for testing parent's module

    #[test]
    fn initial_from_zero() {
        let income = Income::new();
        let salse = Sales::new();
        assert_eq!(income.client(), 0);
        assert_eq!(income.cost, 0);
        assert_eq!(salse.get_total_income(), 0);
        assert_eq!(salse.get_total_expense(), 0);
    }

    #[test]
    fn enum_is_equal() {
        let client1 = Client::Re(0);
        let client2 = Client::Re(0);
        assert_eq!(client1, client2);
    }
}
