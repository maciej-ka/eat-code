// https://leetcode.com/problems/simple-bank-system/submissions/1812164878/

struct Bank {
    accounts: Vec<i64>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Bank { accounts: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = (account1 - 1) as usize;
        let account2 = (account2 - 1) as usize;
        if account1 >= self.accounts.len()
            || account2 >= self.accounts.len()
            || self.accounts[account1] < money
        { return false };

        self.accounts[account1] -= money;
        self.accounts[account2] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = (account - 1) as usize;
        if account >= self.accounts.len() { return false };
        self.accounts[account] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account= (account - 1) as usize;
        if account >= self.accounts.len() || self.accounts[account] < money
        { return false };
        self.accounts[account] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */

#[test]
fn test_1() {
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
    assert_eq!(bank.withdraw(3, 10), true);
    assert_eq!(bank.transfer(5, 1, 20), true);
    assert_eq!(bank.deposit(5, 20), true);
    assert_eq!(bank.transfer(3, 4, 15), false);
    assert_eq!(bank.withdraw(10, 50), false);
}
