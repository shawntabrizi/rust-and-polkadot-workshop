# Enable Balance Transfers

Now that we have initialized and started to use our balances module, let's add probably the most important API: `transfer`.

## Learn

Before we write our function, it is important that we review some of the principles of blockchain and Rust.

### Bad Actors

### Safe Math

### Error Handling



## Create Transfer

So let's create our `transfer` function:

1. In your

```rust
impl BalancesModule {
    pub fn transfer(&mut self, from: &'static str, to: &'static str, amount: u128) -> Result<(), &'static str> {
        let from_balance = self.balance(&from);
        let to_balance = self.balance(&to);

        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}
```
