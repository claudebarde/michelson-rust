![Krusty Krab](https://upload.wikimedia.org/wikipedia/en/3/33/Krusty_Krab_230b.png)

# A Michelson parser and runtime implemented in Rust

I am writing this Michelson engine as I am learning Rust :)

Available instructions (implemented and tested):

- [x] ABS
- [x] ADD
- [x] AMOUNT
- [x] COMPARE
- [x] DIG
- [x] DROP
- [x] DUG
- [x] DUP
- [x] EQ
- [x] FAILWITH
- [x] GE
- [x] GT
- [x] IF
- [x] IF_LEFT
- [x] LE
- [x] LT
- [x] MUL
- [x] NEQ
- [x] NIL
- [x] PAIR
- [x] PUSH
- [x] SENDER
- [x] SOME
- [x] SOURCE
- [x] SUB
- [x] SWAP
- [x] UNPAIR

- [ ] ADDRESS
- [ ] AND
- [ ] APPLY
- [ ] BALANCE
- [ ] BLAKE2B
- [ ] CAST
- [ ] CHAIN_ID
- [ ] CHECK_SIGNATURE
- [ ] CONCAT
- [ ] CONS
- [ ] CONTRACT
- [ ] CREATE_CONTRACT
- [ ] DIP
- [ ] EDIV
- [ ] EMPTY_BIG_MAP
- [ ] EMPTY_MAP
- [ ] EMPTY_SET
- [ ] EXEC
- [ ] GET
- [ ] HASH_KEY
- [ ] IF_CONS
- [ ] IF_NONE
- [ ] IFCMP\*
- [ ] IMPLICIT_ACCOUNT
- [ ] INT
- [ ] ISNAT
- [ ] ITER
- [ ] JOIN_TICKETS
- [ ] KECCAK
- [ ] LAMBDA
- [ ] LEFT
- [ ] LEVEL
- [ ] LOOP
- [ ] LSL
- [ ] LSR
- [ ] MAP
- [ ] MEM
- [ ] NEG
- [ ] NEVER
- [ ] NONE
- [ ] NOT
- [ ] NOW
- [ ] OPEN_CHEST
- [ ] OR
- [ ] PACK
- [ ] PAIRING_CHECK
- [ ] READ_TICKET
- [ ] RENAME
- [ ] RIGHT
- [ ] SAPLING_EMPTY_STATE
- [ ] SAPLING_VERIFY_UPDATE
- [ ] SELF
- [ ] SELF_ADDRESS
- [ ] SET_DELEGATE
- [ ] SHA3
- [ ] SHA256
- [ ] SHA512
- [ ] SIZE
- [ ] SLICE
- [ ] SPLIT_TICKET
- [ ] TICKET
- [ ] TOTAL_VOTING_POWER
- [ ] TRANSFER_TOKENS
- [ ] UNIT
- [ ] UNPACK
- [ ] UPDATE
- [ ] VOTING_POWER
- [ ] XOR

(26 instructions / 94)
