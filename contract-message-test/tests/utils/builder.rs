use fuel_crypto::Hasher;

use fuels::prelude::*;
use fuels::tx::{Address, AssetId, Bytes32, Contract as tx_contract, Input, Output, Transaction};

const CONTRACT_MESSAGE_MIN_GAS: u64 = 1_200_000;
const CONTRACT_MESSAGE_SCRIPT_BINARY: &str =
    "../contract-message-script/out/debug/contract_message_script.bin";
const CONTRACT_MESSAGE_PREDICATE_BINARY: &str =
    "../contract-message-predicate/out/debug/contract_message_predicate.bin";

/// Gets the message to contract script
pub async fn get_contract_message_script() -> (Vec<u8>, Bytes32) {
    let script_bytecode = std::fs::read(CONTRACT_MESSAGE_SCRIPT_BINARY).unwrap();
    let script_hash = Hasher::hash(&script_bytecode.clone());
    (script_bytecode, script_hash)
}

/// Gets the message to contract predicate
pub async fn get_contract_message_predicate() -> (Vec<u8>, Address) {
    let predicate_bytecode = std::fs::read(CONTRACT_MESSAGE_PREDICATE_BINARY).unwrap();
    let predicate_root = Address::from(*tx_contract::root_from_code(&predicate_bytecode));
    (predicate_bytecode, predicate_root)
}

/// Build a message-to-contract transaction with the given input coins and outputs
/// note: unspent gas is returned to the owner of the first given gas input
pub async fn build_contract_message_tx(
    message: Input,
    contract: Input,
    gas_coins: &[Input],
    optional_inputs: &[Input],
    optional_outputs: &[Output],
    params: TxParameters,
) -> Transaction {
    // Get the script and predicate for contract messages
    let (script_bytecode, _) = get_contract_message_script().await;

    // Start building tx list of inputs
    let mut tx_inputs: Vec<Input> = Vec::new();
    tx_inputs.push(contract);
    tx_inputs.push(message);

    // Start building tx list of outputs
    let mut tx_outputs: Vec<Output> = Vec::new();
    tx_outputs.push(Output::Contract {
        input_index: 0u8,
        balance_root: Bytes32::zeroed(),
        state_root: Bytes32::zeroed(),
    });

    // Build a change output for the owner of the first provided coin input
    if !gas_coins.is_empty() {
        let coin: &Input = &gas_coins[0];
        match coin {
            Input::CoinSigned { owner, .. } | Input::CoinPredicate { owner, .. } => {
                // Add change output
                tx_outputs.push(Output::Change {
                    to: owner.clone(),
                    amount: 0,
                    asset_id: AssetId::default(),
                });
            }
            _ => {
                // do nothing
            }
        }
    }

    // Append provided inputs and outputs
    tx_inputs.append(&mut gas_coins.to_vec());
    tx_inputs.append(&mut optional_inputs.to_vec());
    tx_outputs.append(&mut optional_outputs.to_vec());

    // Create the trnsaction
    Transaction::Script {
        gas_price: params.gas_price,
        gas_limit: CONTRACT_MESSAGE_MIN_GAS * 10,
        maturity: params.maturity,
        receipts_root: Default::default(),
        script: script_bytecode,
        script_data: vec![],
        inputs: tx_inputs,
        outputs: tx_outputs,
        witnesses: vec![],
        metadata: None,
    }
}