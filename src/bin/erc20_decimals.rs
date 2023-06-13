use ethers::prelude::*;  
 
 
use std::sync::Arc;  


use ethers_rs_examples::util::wallet_client::{WalletClient,WalletClientError};

//more docs
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
    
    //let provider = wallet_client.provider;
    //let wallet = wallet_client.wallet;
    let signer_middleware = Arc::clone(&wallet_client.signer_middleware);
    
    
    //specify the contract address
    //0xbtc on goerli 
    let contract_address =   "0xab89a7742cb10e7bce98540fd05c7d731839cf9f".parse::<Address>() ?;

    //Initialize contract 
    let token_contract = ERC20::new(contract_address, signer_middleware.clone());
    
    
    let signed_call = match token_contract.decimals(  ).call().await {
        Ok(call) => call,
        Err( .. ) => return Err( WalletClientError::ContractCallError )        
    }; 
     
    println!("decimal result {} " , signed_call);
      
    
    
    Ok(())
}


 