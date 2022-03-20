use scrypto::prelude::*;

blueprint! {
    struct Swap {
       a_pool: Vault,
       b_pool: Vault,
       ratio: Decimal,
       fee_pool: Vault,
       fee: Decimal
    }

    impl Swap {
        
        pub fn new(a_token: Bucket, b_token: Bucket, fee: Decimal) -> Component {
           
            Self {
                a_pool: Vault::with_bucket(a_token),
                b_pool: Vault::with_bucket(b_token),
                ratio: Decimal::one(),
                fee: fee,
                fee_pool: Vault::new(RADIX_TOKEN)
            }
            .instantiate()
        }

        pub fn swap(&mut self, token_b: Bucket, mut fee: Bucket) -> (Bucket, Bucket) {
            assert!(fee.amount() >= self.fee, "No me has pasado fee suficiente");
            let amount: Decimal = token_b.amount() * self.ratio;
            self.b_pool.put(token_b);
            self.fee_pool.put(fee.take(self.fee));
            (self.a_pool.take(amount), fee) 
        }
    }
}
