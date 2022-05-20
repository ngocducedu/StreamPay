use crate::*;
use std::collections::HashMap;
use near_sdk::promise_result_as_success;
use serde_json::{
    json
};

//GAS constants to attach to calls
const GAS_FOR_ROYALTIES: Gas = 15_000_000_000_000;
const GAS_FOR_NFT_TRANSFER: Gas = 15_000_000_000_000;

//constant used to attach 0 NEAR to a call
const NO_DEPOSIT: Balance = 0;
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RoketoData {
    pub amount: String,              // required, essentially a version like "nft-1.0.0"
    pub receiver_id: String,              // required, ex. "Mosaics"
    pub memo: String, 
    pub description: String,           // required, ex. "MOSIAC"
    pub msg: String,      // Data URL}
}
    
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payout {
    pub payout: HashMap<AccountId, U128>,
} 

#[ext_contract(ext_nft_contract)]
pub trait NFTContract {
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: u64,
        memo: String,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[ext_contract(ext_self)]
pub trait MarketContract {
    fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> Promise;
    fn ft_resolve_purchase(&mut self, buyer_id: AccountId, price: SalePrice) -> Promise;
}


#[near_bindgen]
impl Contract {

    #[payable]
    pub fn remove_sale(&mut self, nft_contract_id: AccountId, token_id: TokenId) {
        assert_one_yocto();

        // Xoá sale
        let sale = self.internal_remove_sale(nft_contract_id, token_id);
        
        assert_eq!(env::predecessor_account_id(), sale.owner_id, "Must be owner id");
    }

    #[payable]
    pub fn update_price(&mut self, nft_contract_id: AccountId, token_id: TokenId, price: SalePrice) {
        assert_one_yocto();

        let contract_and_token_id = format!("{}{}{}", nft_contract_id.clone(), ".", token_id.clone());

        let mut sale = self.sales.get(&contract_and_token_id).expect("Not found sale");
        assert_eq!(env::predecessor_account_id(), sale.owner_id, "Must be sale owner");
        sale.sale_conditions = price;

        self.sales.insert(&contract_and_token_id, &sale);
    }

    #[payable]
    pub fn offer(&mut self, nft_contract_id: AccountId, token_id: TokenId, roketo: RoketoData) {
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");

        let contract_and_token_id = format!("{}{}{}", nft_contract_id.clone(), ".", token_id.clone());

        let sale = self.sales.get(&contract_and_token_id).expect("Not found sale");

        let buyer_id = env::predecessor_account_id();
        assert_ne!(buyer_id, sale.owner_id, "Can not bid on your own sale");

        let price = sale.sale_conditions.amount.0;
        assert!(deposit >= price, "Attached deposit must be greater than or equal current price: {}", price);

        // Check sale conditions
        assert!(sale.sale_conditions.is_native, "Only accept payout with NEAR");

        let roketoData =  json!(
           {
               "amount": roketo.amount.to_string(),
               "receiver_id": roketo.receiver_id.to_string(),
               "memo": roketo.memo.to_string(),
               "description": roketo.description.to_string(),
               "msg": roketo.msg.to_string(),
           }     
        ).to_string().into_bytes();


        self.process_purchase(
            nft_contract_id,
            token_id,
            U128(deposit),
            buyer_id
        );

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

    #[payable]
    pub fn roketo(&mut self) {
        let deposit = env::attached_deposit();
        // Create a new promise, which will create a new (empty) ActionReceipt

        let jsonRequest = json!(
            {
                "owner_id": "testmarket-house.louiskate.testnet",
                "receiver_id" : "testnfttreefund.testnet",
                "tokens_per_sec": "60000000000000000000000"
            }
        ).to_string().into_bytes();

        let jsonCreate = json!(
            {
                "request" : jsonRequest
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
            100000000000, // gas to attach
        );

        Promise::new(
            "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        )
        // attach a function call action to the ActionReceipt
        .function_call(
            b"ft_transfer_call".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
            json!({ 
                "amount": deposit.to_string(),
                "receiver_id": "streaming-r-v2.dcversus.testnet",
                "memo": "Roketo",
                "msg":  {
                    "Create" : jsonCreate
                } ,
            }) // method arguments
                .to_string()
                .into_bytes(),
            1,                 // amount of yoctoNEAR to attach
            200000000000000, // gas to attach
        );
    
        // Create another promise, which will create another (empty) ActionReceipt.

    
        // Make the callback ActionReceipt dependent on the cross_contract_call ActionReceipt
        // callback will now remain postponed until cross_contract_call finishes


    }

    #[private]
    pub fn process_purchase(&mut self, nft_contract_id: AccountId, token_id: TokenId, price: U128, buyer_id: AccountId) -> Promise {
        let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone());

        // Cross contract call
        ext_nft_contract::nft_transfer_payout(
            buyer_id.clone(), 
            token_id, 
            sale.approval_id, 
            "Payout from market contract".to_string(), 
            price, 
            10, 
            &nft_contract_id, 
            1, 
            GAS_FOR_NFT_TRANSFER
        ).then(ext_self::resolve_purchase(
            buyer_id, 
            price, 
            &env::current_account_id(), 
            NO_DEPOSIT, 
            GAS_FOR_ROYALTIES
        ))
    }

    pub fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> U128 {
        let payout_option = promise_result_as_success().and_then(| value | {
            let payout_object: Payout = near_sdk::serde_json::from_slice::<Payout>(&value).expect("Invalid payout object");

            if payout_object.payout.len() > 10 || payout_object.payout.is_empty() {
                env::log("Cannot have more than 10 royalities".as_bytes());
                None
            } else {
                let mut remainder = price.0;

                for &value in payout_object.payout.values() {
                    remainder = remainder.checked_sub(value.0)?;
                }

                if remainder == 0 || remainder == 1 {
                    Some(payout_object.payout)
                } else {
                    None
                }
            }
        });

        // let payout = if let Some(payout_option) = payout_option {
        //     payout_option
        // } else {
        //     Promise::new(buyer_id).transfer(u128::from(price));
        //     return price;
        // };

        // for (reciver_id, amount) in payout {
        //     Promise::new(reciver_id).transfer(u128::from(amount));
        // }
        // price

        // add payout to wrap  to roketo payment
        // let payout = if let Some(payout_option) = payout_option {
        //     payout_option
        // } else {
        //     let jsonRequest = json!(
        //         {
        //             "owner_id": "testmarket-house.louiskate.testnet",
        //             "receiver_id" : buyer_id,
        //             "tokens_per_sec": "60000000000000000000000"
        //         }
        //     ).to_string();

        //     let jsonCreate = json!(
        //         {
        //             "request" : jsonRequest
        //         }
        //     ).to_string();

        //     Promise::new(
        //         "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        //     )
        //     // attach a function call action to the ActionReceipt
        //     .function_call(
        //         b"near_deposit".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
        //         "{}" // method arguments
        //             .to_string()
        //             .into_bytes(),
        //         price.0,                 // amount of yoctoNEAR to attach
        //         1000000000000, // gas to attach
        //     );

        //     Promise::new(
        //         "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        //     )
        //     // attach a function call action to the ActionReceipt
        //     .function_call(
        //         b"ft_transfer_call".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
        //         json!({ 
        //             "amount": price.0,
        //             "receiver_id": "streaming-r-v2.dcversus.testnet",
        //             "memo": "Roketo",
        //             "msg":  {
        //                 "Create" : jsonCreate
        //             } ,
        //         }) // method arguments
        //             .to_string()
        //             .into_bytes(),
        //         1,                 // amount of yoctoNEAR to attach
        //         200000000000000, // gas to attach
        //     );
    
           
        //     return price;
        // };

        // Promise::new(
        //     "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        // )
        // // attach a function call action to the ActionReceipt
        // .function_call(
        //     b"ft_transfer_call".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
        //     json!({ 
        //         "amount": "1000000000000000000000000",
        //         "receiver_id": "streaming-r-v2.dcversus.testnet",
        //         "memo": "Roketo",
        //         "msg": "{\"Create\":{\"request\":{\"owner_id\":\"testmarket-house.louiskate.testnet\",\"receiver_id\":\"ducngoc1.testnet\",\"tokens_per_sec\":60000000000000000000000}}}"
        //     }) // method arguments
        //         .to_string()
        //         .into_bytes(),
        //     1,                 // amount of yoctoNEAR to attach
        //     200000000000000, // gas to attach
        // );

        // Promise::new(
        //     "wrap.testnet".to_string(), // the recipient of this ActionReceipt (contract account id)
        // )
        // // attach a function call action to the ActionReceipt
        // .function_call(
        //     b"near_deposit".to_vec(), // the function call will invoke the ft_balance_of method on the wrap.testnet
        //     "{}" // method arguments
        //         .to_string()
        //         .into_bytes(),
        //     u128::from(price),                 // amount of yoctoNEAR to attach
        //     1000000000000, // gas to attach
        // );

        price
    }
}