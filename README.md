# Astrolescent Staking contract ("rug proof" version)
 
Below are the transaction manifests needed to use the contract:

## instantiate (stokenet)
```
CALL_FUNCTION
  Address("package_tdx_2_1p50vvget87alvsjwqhpv8m0exwq6zu32ppnuys5zjx9epjyuuuu7ay")
  "ASTRLSTAKING"
  "new"
  Address("<OWNER_BADGE>")
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
;
```

## instantiate (mainnet)
```
CALL_FUNCTION
  Address("package_rdx1p5zklqgyeaje7zm6tx3v9zmcqszecqdw0y9za9tw8pehkv87wa4ykm")
  "ASTRLSTAKING"
  "new"
  Address("<OWNER_BADGE>")
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
;
```

## add stake
```
CALL_METHOD
  Address("<ACCOUNT>")
  "withdraw"
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Decimal("100")
;

TAKE_ALL_FROM_WORKTOP
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Bucket("tokens")
;

CALL_METHOD
	Address("<STAKE_COMPONENT_ADDRESS>")
	"add_stake"
	Bucket("tokens")
;

CALL_METHOD
    Address("<ACCOUNT>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

## remove stake
```
CALL_METHOD
  Address("<ACCOUNT>>")
  "withdraw"
  Address("<POOL_UNIT_RESOURCE_ADDRESS>")
  Decimal("10")
;

TAKE_ALL_FROM_WORKTOP
  Address("<POOL_UNIT_RESOURCE_ADDRESS>")
  Bucket("tokens")
;

CALL_METHOD
	Address("<STAKE_COMPONENT_ADDRESS>")
	"remove_stake"
	Bucket("tokens")
;

CALL_METHOD
    Address("<ACCOUNT>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

## airdrop

To reward users for staking it is possible to deposit `RESOURCE_ADDRESS_TO_STAKE` without receiving new Pool Units in return, making the existing Pool Units hold more `RESOURCE_ADDRESS_TO_STAKE` and increasing their value.

```

CALL_METHOD
  Address("<ACCOUNT_HOLDING_OWNER_BADGE>")
    "create_proof_of_non_fungibles"
    Address("<OWNER_BADGE>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("#1#")
    )
;

CALL_METHOD
  Address("<ACCOUNT>")
  "withdraw"
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Decimal("<AMOUNT>")
;

TAKE_ALL_FROM_WORKTOP
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Bucket("tokens")
;

CALL_METHOD
	Address("<STAKE_COMPONENT_ADDRESS>")
	"airdrop"
	Bucket("tokens")
;
```

## deposit rewards

The owner can use this method to lock all of the future staking rewards, there will be no way to get them back.
```

CALL_METHOD
  Address("<ACCOUNT_HOLDING_OWNER_BADGE>")
    "create_proof_of_non_fungibles"
    Address("<OWNER_BADGE>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("#1#")
    )
;

CALL_METHOD
  Address("<ACCOUNT>")
  "withdraw"
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Decimal("<AMOUNT>")
;

TAKE_ALL_FROM_WORKTOP
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Bucket("tokens")
;

CALL_METHOD
  Address("<STAKE_COMPONENT_ADDRESS>")
  "deposit_rewards"
  Bucket("tokens")
;
```

## airdrop part of the previosly deposited coins
```

CALL_METHOD
  Address("<ACCOUNT_HOLDING_OWNER_BADGE>")
    "create_proof_of_non_fungibles"
    Address("<OWNER_BADGE>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("#1#")
    )
;

CALL_METHOD
  Address("<STAKE_COMPONENT_ADDRESS>")
  "airdrop_deposited_amount"
  Decimal("AMOUNT TO DISTRIBUTE")
;
```
