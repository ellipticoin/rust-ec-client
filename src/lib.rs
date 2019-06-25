extern crate crypto;
extern crate reqwest;
extern crate serde_cbor;

use serde::{Deserialize, Serialize};
pub use serde_cbor::Value;
use serde_cbor::{from_slice, to_vec};

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    #[serde(with = "serde_bytes")]
    pub sender: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub contract_address: Vec<u8>,
    pub contract_name: String,
    pub nonce: u64,
    pub function: String,
    pub arguments: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignedTransaction {
    #[serde(with = "serde_bytes")]
    pub sender: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub contract_address: Vec<u8>,
    pub contract_name: String,
    pub nonce: u64,
    pub function: String,
    pub arguments: Vec<Value>,
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
}

pub fn create_contract(
    name: &str,
    code: &[u8],
    constructor_arguments: Vec<Value>,
    private_key: &[u8],
) {
    post(
        Transaction {
            sender: private_key[32..].to_vec(),
            contract_address: [0; 32].to_vec(),
            contract_name: "system".to_string(),
            nonce: 0,
            function: "create_contract".to_string(),
            arguments: vec![
                name.to_string().into(),
                Value::Bytes(code.to_vec()),
                Value::Array(constructor_arguments),
            ],
        },
        private_key,
    );
}

pub fn post(transaction: Transaction, private_key: &[u8]) {
    let transaction_bytes = to_vec(&transaction).unwrap();
    let signature = crypto::ed25519::signature(&transaction_bytes, private_key).to_vec();
    let signed_transaction = SignedTransaction {
        sender: transaction.sender,
        contract_address: transaction.contract_address,
        contract_name: transaction.contract_name,
        nonce: transaction.nonce,
        function: transaction.function,
        arguments: transaction.arguments,
        signature: signature,
    };
    let signed_transaction_bytes = to_vec(&signed_transaction).unwrap();
    let client = reqwest::Client::new();
    let resp = client
        .post("http://davenport.ellipticoin.org:4460/transactions")
        .header("Content-Type", "application/cbor")
        .body(signed_transaction_bytes)
        .send();
}
