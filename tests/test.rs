#![cfg(test)]

use soroban_sdk::{Env, BytesN};
use soroban_external_contract::{ExampleContract, ExampleContractClient};

fn register_example(e: &Env, contract_id: &[u8; 32]) {
    let contract_id = BytesN::from_array(e, contract_id);
    // This call fails for - 
    //     the trait `ContractFunctionSet` is not implemented for `ExampleContract`
    e.register_contract(&contract_id, ExampleContract);
}

#[test]
fn test_do_thing() {
    let test_env = Env::default();

    let ex_id: [u8; 32] = [1; 32];
    register_example(&test_env, &ex_id);

    let ex_client = ExampleContractClient::new(&test_env, &ex_id);
    let x: u64 = 123; 
    let result = ex_client.do_thing(&x);
    assert_eq!(result, 124);
}
