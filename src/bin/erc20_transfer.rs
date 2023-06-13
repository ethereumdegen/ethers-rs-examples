use ethers::prelude::*;  
use ethers::types::transaction::eip2930::AccessList;
 
 
use std::sync::Arc; 
 

use eyre::Result;


use ethers_rs_examples::util::wallet_client::{WalletClient,WalletClientError};


use ethers::types::{Address};

 


//https://www.gakonst.com/ethers-rs/

#[tokio::main]
async fn main()   -> Result<(),WalletClientError> {
        
        
    abigen!(
        ERC20,
        "./src/abi/erc20.abi.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );
     
     
    let wallet_client = match WalletClient::from_env() {
        Ok(wc) => wc,
        Err( e ) =>  return Err( e )
        
    };
     
    let wallet = wallet_client.wallet;
    let signer_middleware = Arc::clone(&wallet_client.signer_middleware);
   
       
    
    // specify the contract address 
    let contract_address =   "0x82DC1036bF9E1078D9B38b2C7e2ebFBaEd0e7eD5".parse::<Address>() ?;

    // Initialize contract 
    let token_contract = ERC20::new(contract_address, signer_middleware.clone());
  
    let recipient = "0x82DC1036bF9E1078D9B38b2C7e2ebFBaEd0e7eD5".parse::<Address>() ?;    
    let transfer_call = token_contract.transfer( recipient, (100).into()   );
    
    
    let swap_data_bytes = transfer_call.calldata().unwrap();
    let data = Some( swap_data_bytes );
    
    println!("broadcasting from {}", wallet.address().clone() );
    
    // You could call this fn directly like this !! 
    // transfer_call.send( ).await;
    
    
    //Or you can do it manually ! 
    let transaction:Eip1559TransactionRequest = Eip1559TransactionRequest {
        to: Some(NameOrAddress::Address(contract_address)),
        data, 
        from: Some(wallet.address()),
        gas: Some(220000.into()),
        value: None,
        nonce: None,
        access_list: AccessList::default(),    
        max_priority_fee_per_gas: Some(2.into()),
        max_fee_per_gas: None,
        chain_id: Some(Chain::Goerli.into()),        
    };
    
    let pending_tx = signer_middleware.send_transaction(transaction, None).await;
       match pending_tx {
        Ok( tx ) =>  {
            println!("tx broadcasted: {} " , tx.tx_hash())
             } ,
        Err(e) => {
            println!("{}", e.to_string());
            }
    }
 
    
       
    
    Ok(())
}


 