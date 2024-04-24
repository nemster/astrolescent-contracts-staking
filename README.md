# Astrolescent Staking contract (customised)

This fork adds two features to the Astrolescent Staking contract:  

- rug proof: the owner can deposit ahead of time all of the future staking rewards with no way to get them back (`deposit_rewards` method). The deposited staking rewards can then be distributed calling the `airdrop_deposited_amount` method.  

- optional `locking_period`: by specifying a non zero `<LOCKING_PERIOD>` during component instantiation (function `new`), the owner can make so that at the unstake a `ClaimNFT` is returned instead of the staked coins. The ClaimNFT clearly shows in the user's wallet the unstaked `amount` and the `claim_date` and can be redeemed via the `claim_unstaked_coins` method when the `locking_period` ends.  
 
Below are the transaction manifests needed to use the contract:  

## instantiate (stokenet)
```
CALL_FUNCTION
  Address("package_tdx_2_1pkvqy92x2wsp9rpdg5xtuqy9m8ux5vyrnhdlfsgjyylplqlywz08w0")
  "AstrlStaking"
  "new"
  Address("<OWNER_BADGE>")
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  <LOCKING_PERIOD>i64
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

## redeem previously unstaked coins
```

CALL_METHOD
    Address("<ACCOUNT>")
    "withdraw_non_fungibles"
    Address("<CLAIM_NFT_ADDRESS>")
    Array<NonFungibleLocalId>(NonFungibleLocalId("<CLAIM_NFT_ID>"))
;

TAKE_ALL_FROM_WORKTOP
  Address("<CLAIM_NFT_ADDRESS>")
  Bucket("claim_nft")
;

CALL_METHOD
  Address("<STAKE_COMPONENT_ADDRESS>")
  "claim_unstaked_coins"
  Bucket("claim_nft")
;

CALL_METHOD
    Address("<ACCOUNT>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```
