use crate::*;

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
       let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

       let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set{
        tokens_for_owner_set
       } else {
        // if there is no set of tokens - simply return an empty vector. 
        return vec![];
       };

       // pagination
       let start = u128::from(from_index.unwrap_or(U128(0)));

       tokens.iter()
        .skip(start as usize)
        .take(limit.unwrap_or(50) as usize)
        .map(|token_id| self.nft_token(token_id.clone()).unwrap())
        .collect()

    }
}