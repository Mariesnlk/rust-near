use crate::*;
use near_sdk::{ ext_contract };

pub trait NonFungibleTokenCore {
    //approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);

    //check if the passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool;

    //revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId);

    //revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: TokenId);
}

#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    //cross contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String
    );
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    //allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        assert_at_least_one_yocto();

        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        assert_eq!(
            &env::predecessor_account_id(),
            &token.owner_id,
            "Predecessor must be the token owner."
        );

        //get the next approval ID if we need a new approval
        let approval_id: u64 = token.next_approval_id;

        //check if the account has been approved already for this token
        let is_new_approval = token.approved_account_ids
            //insert returns none if the key was not present
            .insert(account_id.clone(), approval_id)
            //if the key was not present, .is_none() will return true so it is a new approval
            .is_none();

        //if it was a new approval - need to calculate how much storage is being used to add the account
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        token.next_approval_id += 1;
        //insert the token back into the tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);
        //refund any excess storage attached by the user. If the user didn't attach enough, panic
        refund_deposit(storage_used);

        if let Some(msg) = msg {
            // Defaulting GAS weight to 1, no attached deposit, and no static GAS to attach
            ext_non_fungible_approval_receiver
                ::ext(account_id)
                .nft_on_approve(token_id, token.owner_id, approval_id, msg)
                .as_return();
        }
    }

    //check if the passed in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool {
        let token = self.tokens_by_id.get(&token_id).expect("No token");

        //get the approval number for the passed in account ID
        let approval = token.approved_account_ids.get(&approved_account_id);

        if let Some(approval) = approval {
            if let Some(approval_id) = approval_id { approval_id == *approval } else { true }
        } else {
            true
        }
    }

    //revoke a specific account from transferring the token on your behalf
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        assert_one_yocto();

        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        //if the account ID was in the token's approval - remove it and the if statement logic executes
        if token.approved_account_ids.remove(&account_id).is_some() {
            refund_approved_account_ids_iter(predecessor_account_id, [account_id].iter());

            self.tokens_by_id.insert(&token_id, &token);
        }
    }

    //revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        assert_one_yocto();

        let mut token = self.tokens_by_id.get(&token_id).expect("No token");

        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &token.owner_id);

        if !token.approved_account_ids.is_empty() {
            refund_approved_account_ids(predecessor_account_id, &token.approved_account_ids);

            token.approved_account_ids.clear();

            self.tokens_by_id.insert(&token_id, &token);
        }
    }
}