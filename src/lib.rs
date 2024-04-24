use scrypto::prelude::*;

#[derive(Debug, ScryptoSbor, NonFungibleData)]
struct ClaimNFT {
    id: u64,
    amount: Decimal,
    claim_date: Instant,
}

#[blueprint]
mod astrl_staking {

    // Setting the access rules 
    enable_method_auth! { 
        methods { 

            add_stake => PUBLIC; 
            remove_stake => PUBLIC; 
            claim_unstaked_coins => PUBLIC; 
            airdrop => restrict_to: [OWNER];
            deposit_rewards => restrict_to: [OWNER];
            airdrop_deposited_amount => restrict_to: [OWNER];
            
        }
    
    }

    struct AstrlStaking {

        sastrl: Global<OneResourcePool>,
        future_rewards: Vault,
        claim_nft_resource_manager: ResourceManager,
        unlock_period: i64,
        pending_unstake_vaults: KeyValueStore<u64, Vault>,
        next_id: u64,

    }

    impl AstrlStaking {
        pub fn new(owner_badge: ResourceAddress, ra: ResourceAddress, unlock_period: i64) -> Global<OneResourcePool> {

            assert!(unlock_period >= 0, "Invalid unlock_period");

            let (address_reservation, component_address) = Runtime::allocate_component_address(AstrlStaking::blueprint_id());
            let global_component_caller_badge = NonFungibleGlobalId::global_caller_badge(component_address);

            let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            let sastrl = Blueprint::<OneResourcePool>::instantiate(
                owner_role.clone(),
                rule!(require(global_component_caller_badge)), 
                ra,
                None
            );

            let claim_nft_resource_manager = ResourceBuilder::new_integer_non_fungible::<ClaimNFT>(
                OwnerRole::Updatable(rule!(require(owner_badge)))
            )
            .metadata(metadata!(
                roles {
                    metadata_setter => rule!(require(owner_badge));
                    metadata_setter_updater => rule!(require(owner_badge));
                    metadata_locker => rule!(require(owner_badge));
                    metadata_locker_updater => rule!(require(owner_badge));
                },
                init {
                    "name" => "Claim NFT", updatable;
                }
            ))
            .mint_roles(mint_roles!(
                minter => rule!(require(global_caller(component_address)));
                minter_updater => rule!(deny_all);
            ))
            .non_fungible_data_update_roles(non_fungible_data_update_roles!(
                non_fungible_data_updater => rule!(deny_all);
                non_fungible_data_updater_updater => rule!(deny_all);
            ))
            .burn_roles(burn_roles!(
                burner => rule!(require(global_caller(component_address)));
                burner_updater => rule!(deny_all);
            ))
            .create_with_no_initial_supply();

            Self {

                sastrl: sastrl,
                future_rewards: Vault::new(ra),
                claim_nft_resource_manager: claim_nft_resource_manager,
                unlock_period: unlock_period,
                pending_unstake_vaults: KeyValueStore::new(),
                next_id: 1,

            }

            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(
                rule!(require(owner_badge))
            ))
            .with_address(address_reservation)
            .globalize();

            return sastrl
        }

        pub fn remove_stake(&mut self, sastrl: Bucket) -> Bucket {

            let tokens = self.sastrl.redeem(sastrl);

            if self.unlock_period == 0 {

                return tokens;

            } else {

                let mut instant = Clock::current_time_rounded_to_minutes();
                instant.seconds_since_unix_epoch += self.unlock_period;

                let non_fungible_data = ClaimNFT {
                    id: self.next_id,
                    amount: tokens.amount(),
                    claim_date: instant,
                };

                let claim_nft = self.claim_nft_resource_manager.mint_non_fungible(
                    &NonFungibleLocalId::integer(self.next_id.into()),
                    non_fungible_data,
                );

                self.pending_unstake_vaults.insert(self.next_id, Vault::with_bucket(tokens));

                return claim_nft;

            }

        }

        pub fn add_stake(&mut self, astrl: Bucket) -> Bucket {

            let tokens = self.sastrl.contribute(astrl);
            return tokens

        }

        pub fn airdrop(&mut self, astrl: Bucket){

            self.sastrl.protected_deposit(astrl);
            return

        }

        pub fn deposit_rewards(&mut self, astrl: Bucket) {

            self.future_rewards.put(astrl);

        }

        pub fn airdrop_deposited_amount(&mut self, amount: Decimal){

            self.sastrl.protected_deposit(
                self.future_rewards.take(amount)
            );

        }

        pub fn claim_unstaked_coins(&mut self, claim_nft: Bucket) -> Bucket {

            assert!(
                claim_nft.resource_address() == self.claim_nft_resource_manager.address(),
                "Unknown token",
            );

            let non_fungible_data = claim_nft
            .as_non_fungible()
            .non_fungible::<ClaimNFT>()
            .data();

            assert!(
                non_fungible_data.claim_date.seconds_since_unix_epoch <= Clock::current_time_rounded_to_minutes().seconds_since_unix_epoch,
                "Not ready to be claimed",
            );

            claim_nft.burn();

            return self.pending_unstake_vaults.get_mut(&non_fungible_data.id)
            .expect("Vault not found")
            .take_all();
        }
    }
}
