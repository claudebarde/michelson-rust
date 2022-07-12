![Krusty Krab](https://upload.wikimedia.org/wikipedia/en/3/33/Krusty_Krab_230b.png)

# A Michelson parser and runtime implemented in Rust

I am writing this Michelson engine as I am learning Rust :)
As it is a work in process, it is subject to constant breaking changes. Wait until the stable version to use.

Available instructions (implemented and tested):

- [x] ABS
- [x] ADD
- [x] AND
- [x] AMOUNT
- [x] BALANCE
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
- [x] INT
- [x] LE
- [x] LT
- [x] MUL
- [x] NEQ
- [x] NIL
- [x] NONE
- [x] NOT
- [x] OR
- [x] PAIR
- [x] PUSH
- [x] SELF_ADDRESS
- [x] SENDER
- [x] SIZE
- [x] SOME
- [x] SOURCE
- [x] SUB
- [x] SWAP
- [x] UNIT
- [x] UNPAIR
- [x] XOR

- [ ] ADDRESS
- [ ] APPLY
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
- [ ] NOW
- [ ] OPEN_CHEST
- [ ] PACK
- [ ] PAIRING_CHECK
- [ ] READ_TICKET
- [ ] RENAME
- [ ] RIGHT
- [ ] SAPLING_EMPTY_STATE
- [ ] SAPLING_VERIFY_UPDATE
- [ ] SELF
- [ ] SET_DELEGATE
- [ ] SHA3
- [ ] SHA256
- [ ] SHA512
- [ ] SLICE
- [ ] SPLIT_TICKET
- [ ] TICKET
- [ ] TOTAL_VOTING_POWER
- [ ] TRANSFER_TOKENS
- [ ] UNPACK
- [ ] UPDATE
- [ ] VOTING_POWER

(36 instructions / 94)
