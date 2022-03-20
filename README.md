# Ejemplo de Atomic Composability

Crea el archivo transaccion.rtm con el siguiente código cambiando las direcciones.

y ejecuta con:
``` 
resim run transaccion.rtm
```
## Código del manifiesto de transacción:
```rust
CALL_METHOD Address("02cf99c283616b96df11c8c6fbc19b20feb14cf1e696978ce00930") "withdraw" Decimal("101") Address("030000000000000000000000000000000000000000000000000004") BucketRef(1u32);

TAKE_FROM_WORKTOP Decimal("100") Address("030000000000000000000000000000000000000000000000000004") Bucket("XRD");
TAKE_FROM_WORKTOP Decimal("0.5") Address("030000000000000000000000000000000000000000000000000004") Bucket("fee1");
TAKE_FROM_WORKTOP Decimal("0.5") Address("030000000000000000000000000000000000000000000000000004") Bucket("fee2");
CALL_METHOD Address("02cc9c4e9aa78fb116732acf4efeafb33a62b5d60edeb32ee7dc36") "swap" Bucket("XRD") Bucket("fee1") ;

ASSERT_WORKTOP_CONTAINS Decimal("100") Address("03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542");

TAKE_ALL_FROM_WORKTOP Address("03d527faee6d0b91e7c1bab500c6a986e5777a25d704acc288d542") Bucket("USD");
CALL_METHOD Address("02907db7d030b81b2a537fb1efb11b28a2f04d1cf111cfbcb1bdaf") "swap" Bucket("USD") Bucket("fee2");

ASSERT_WORKTOP_CONTAINS Decimal("100") Address("0347dfe3a58e8a630305f2f3df82949cd70ce49e2cde097b259f8d");

CALL_METHOD_WITH_ALL_RESOURCES Address("02cf99c283616b96df11c8c6fbc19b20feb14cf1e696978ce00930") "deposit_batch";
```