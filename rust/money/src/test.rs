use std::rc::Rc;

struct Company {
    asset: i32,
    income: i32,
    emp_cnt: u32,
    debt: Rc<Dept>,
}

impl Company {
    fn need_audit(&self) -> bool {
        let mut count = 0;

        if self.asset > 120 {
            count += 1;
        }
        if self.income > 100 {
            count += 1;
        }
        if self.emp_cnt > 100 {
            count += 1;
        }
        if self.debt.budget > 70 {
            count += 1;
        }

        if count >= 2 {
            println!("You need to clean up your financial statements.");
            return true;
        } else {
            return false;
        }
    }
}

struct Dept {
    owner: String,
    borrower: Company,
    budget: i32,
}

impl Dept {
    fn can_maturity_extension(&self) -> bool {
        self.borrower.need_audit()
    }
}

pub fn main() {
    let company = Company {
        asset: 100,
        income: 100,
        emp_cnt: 100,
        debt: Dept::new("Bob", company, 70),
    };

    if company.need_audit() {}
}
