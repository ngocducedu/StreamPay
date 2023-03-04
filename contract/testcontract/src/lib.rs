use near_sdk::{AccountId, near_bindgen, PanicOnDefault, Balance, env, Promise, CryptoHash, ext_contract, Gas, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap, UnorderedSet, UnorderedMap};
use serde_json::{
    json
};





pub type TokenId = String;
pub type NFTContractId = String;
pub type ContractAndTokenId = String; //nft-tutorial.vbidev.testnet.VBI_NFT#01

#[derive(Deserialize, Serialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SalePrice {
    is_native: bool,
    contract_id: AccountId,
    decimals: U64,
    amount: U128
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Owner of contract
    pub owner_id: AccountId,

   
    
}



#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    SaleKey,
    ByOwnerIdKey,
    InnterByOwnerIdKey {
        account_id_hash: CryptoHash
    },
    InnterAuctionsByOwnerKey {
        account_id_hash: CryptoHash
    },
    ByContractIdKey,
    InnerByContractIdKey {
        account_id_hash: CryptoHash
    },
    InnerAuctionByContractIdKey {
        account_id_hash: CryptoHash
    },
    StorageDepositKey,
    AuctionsKey,
    AuctionsByOwnerKey,
    AuctionByContractIdKey,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
        
        }
    }

    #[payable]
    pub fn offer(&mut self) {
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");


        let roketoData =  json!(
           {
               "amount": "1000000000000000000000000",
               "receiver_id": "streaming-r-v2.dcversus.testnet",
               "memo": "test",
               "description": "Okla1",
               "msg": "{\"Create\":{\"request\":{\"owner_id\":\"market1.streampay.testnet\",\"receiver_id\":\"ericng.testnet\",\"tokens_per_sec\":\"38580246913580250\"}}}",
           }     
        ).to_string().into_bytes();


        Promise::new(
            "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        )
        // attach a function call action to the ActionReceipt
        .function_call(
            b"near_deposit".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
            "{}" // method arguments
                .to_string()
                .into_bytes(),
            deposit,                 // amount of yoctoNEAR to attach
            10000000000, // gas to attach
        );

        Promise::new(
            "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        )
        // attach a function call action to the ActionReceipt
        .function_call(
            b"ft_transfer_call".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
            roketoData,
            1,                 // amount of yoctoNEAR to attach
            200000000000000, // gas to attach
        );
    }

}