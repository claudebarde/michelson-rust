![Krusty Krab](https://upload.wikimedia.org/wikipedia/en/3/33/Krusty_Krab_230b.png)

# A Michelson parser and runtime implemented in Rust

I am writing this Michelson engine as I am learning Rust :)
As it is a work in process, it is subject to constant breaking changes. Wait until the stable version to use.

Available instructions (implemented and tested):

- [x] ABS
- [x] ADD
- [x] ADDRESS
- [x] AND
- [x] AMOUNT
- [x] BALANCE
- [x] CAR
- [x] CDR
- [x] CHAIN_ID
- [x] COMPARE
- [x] CONCAT
- [x] CONS
- [x] DIG
- [x] DROP
- [x] DUG
- [x] DUP
- [x] EDIV
- [x] EMPTY_BIG_MAP
- [x] EMPTY_MAP
- [x] EMPTY_SET
- [x] EQ
- [x] FAILWITH
- [x] GE
- [x] GET
- [x] GT
- [x] IF
- [x] IF_LEFT
- [x] INT
- [x] ISNAT
- [x] KECCAK
- [x] LE
- [x] LEFT
- [x] LEVEL
- [x] LT
- [x] MAP
- [x] MEM
- [x] MUL
- [x] NEG
- [x] NEQ
- [x] NEVER
- [x] NIL
- [x] NONE
- [x] NOW
- [x] NOT
- [x] OR
- [x] PAIR
- [x] PUSH
- [x] RIGHT
- [x] SELF_ADDRESS
- [x] SENDER
- [x] SIZE
- [x] SLICE
- [x] SOME
- [x] SOURCE
- [x] SUB
- [x] SWAP
- [x] UNIT
- [x] UNPAIR
- [x] XOR

- [ ] APPLY
- [ ] BLAKE2B
- [ ] CAST
- [ ] CHECK_SIGNATURE
- [ ] CONTRACT
- [ ] CREATE_CONTRACT
- [ ] DIP
- [ ] EXEC
- [ ] HASH_KEY
- [ ] IF_CONS
- [ ] IF_NONE
- [ ] IFCMP\*
- [ ] IMPLICIT_ACCOUNT
- [ ] ITER
- [ ] JOIN_TICKETS
- [ ] LAMBDA
- [ ] LOOP
- [ ] LSL
- [ ] LSR
- [ ] OPEN_CHEST
- [ ] PACK
- [ ] PAIRING_CHECK
- [ ] READ_TICKET
- [ ] RENAME
- [ ] SAPLING_EMPTY_STATE
- [ ] SAPLING_VERIFY_UPDATE
- [ ] SELF
- [ ] SET_DELEGATE
- [ ] SHA3
- [ ] SHA256
- [ ] SHA512
- [ ] SPLIT_TICKET
- [ ] TICKET
- [ ] TOTAL_VOTING_POWER
- [ ] TRANSFER_TOKENS
- [ ] UNPACK
- [ ] UPDATE
- [ ] VOTING_POWER

(57 instructions / 95)

## How to run the tests for `utils`?

```
cargo test utils_test_micheline_to_json -- --nocapture
```

## How to run the tests in watch mode for `utils`?

```
cargo watch -x 'test utils_test_micheline_to_json -- --nocapture'
```
