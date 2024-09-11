use std::{fs};
use noir_rs::recursion;
use noir_rs::{prove::prove_honk, srs::setup_srs, utils::decode_circuit, verify::verify_honk};
use acir::FieldElement;
use acir::native_types::{Witness, WitnessMap};

const LOCAL_SRS: &str = "transcript00.dat";

const INNER_X_ARG: u128 = 1234;
const INNER_1_OUT_ARG: &str = "0x12a62eebe17a2a4852ebcb89fbcba38e5bd980e95f579b68405090352babc73c";
const INNER_2_OUT_ARG: &str = "0x15278acd02283843129846f018a13892b03b4c5290f1dfe1c40b738c654f7766";

const INNER_1_VKEY_HASH: &str = "0x1a11f0369e221c27f75c5b95164093a3d21447223ec78a859c263b25fb7a12fd";
const INNER_2_VKEY_HASH: &str = "0x2608f0cf8afd53837eb9ccea0a89241c3f97ed4f837de807052b9e57e592e06e";


#[test]
fn test_honk_inner1() {
    // Read the JSON manifest of the circuit
    let recursed_circuit_txt = fs::read_to_string("./target/inner1.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let recursed_circuit: serde_json::Value = serde_json::from_str(&recursed_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursed_circuit_bytecode = recursed_circuit["bytecode"].as_str().unwrap();

    setup_srs(String::from(recursed_circuit_bytecode), Some(LOCAL_SRS)).unwrap();

    // Citcuit inputs
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::from(INNER_X_ARG));
    initial_witness.insert(Witness(1), FieldElement::from(FieldElement::try_from_str(INNER_1_OUT_ARG).unwrap()));

    let (recursed_proof, recursed_vk) = prove_honk(String::from(recursed_circuit_bytecode), initial_witness).unwrap();
    let (proof_as_fields, vk_as_fields, key_hash) = recursion::generate_recursive_honk_proof_artifacts(recursed_proof, recursed_vk).unwrap();

    // println!("proof: {:?}", proof);
    // println!("vk: {:?}", vk);
    // println!("key_hash: {:?}", key_hash);

    assert_eq!(proof_as_fields.len(), 409);
    assert_eq!(vk_as_fields.len(), 103);
    assert_eq!(key_hash, INNER_1_VKEY_HASH);
}


#[test]
fn test_honk_inner2() {
    // Read the JSON manifest of the circuit
    let recursed_circuit_txt = fs::read_to_string("./target/inner2.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let recursed_circuit: serde_json::Value = serde_json::from_str(&recursed_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursed_circuit_bytecode = recursed_circuit["bytecode"].as_str().unwrap();

    setup_srs(String::from(recursed_circuit_bytecode), Some(LOCAL_SRS)).unwrap();

    // Citcuit inputs
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::from(INNER_X_ARG));
    initial_witness.insert(Witness(1), FieldElement::from(FieldElement::try_from_str(INNER_2_OUT_ARG).unwrap()));

    let (recursed_proof, recursed_vk) = prove_honk(String::from(recursed_circuit_bytecode), initial_witness).unwrap();
    let (proof_as_fields, vk_as_fields, key_hash) = recursion::generate_recursive_honk_proof_artifacts(recursed_proof, recursed_vk).unwrap();

    // println!("proof: {:?}", proof);
    // println!("vk: {:?}", vk);
    // println!("key_hash: {:?}", key_hash);

    assert_eq!(proof_as_fields.len(), 409);
    assert_eq!(vk_as_fields.len(), 103);
    assert_eq!(key_hash, INNER_2_VKEY_HASH);
}


#[test]
fn test_honk_outer() {
    // Read the JSON manifest of the circuit
    let recursed_circuit_txt = fs::read_to_string("./target/inner2.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let recursed_circuit: serde_json::Value = serde_json::from_str(&recursed_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursed_circuit_bytecode = recursed_circuit["bytecode"].as_str().unwrap();

    setup_srs(String::from(recursed_circuit_bytecode), Some(LOCAL_SRS)).unwrap();

    // Inner proof circuit inputs
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::from(INNER_X_ARG));
    initial_witness.insert(Witness(1), FieldElement::from(FieldElement::try_from_str(INNER_2_OUT_ARG).unwrap()));
    // Prove inner proof
    let (recursed_proof, recursed_vk) = prove_honk(String::from(recursed_circuit_bytecode), initial_witness).unwrap();
    let (proof_as_fields, vk_as_fields, key_hash) = recursion::generate_recursive_honk_proof_artifacts(recursed_proof, recursed_vk).unwrap();

    // println!("proof: {:?}", proof);
    // println!("vk: {:?}", vk);
    // println!("proof_as_fields: {:?}", proof_as_fields);
    // println!("key_hash: {:?}", key_hash);

    assert_eq!(proof_as_fields.len(), 409);
    assert_eq!(vk_as_fields.len(), 103);

    // Read the JSON manifest of the circuit
    let recursive_circuit_txt = fs::read_to_string("./target/outer.json").unwrap();
    // Parse the JSON manifest into a dictionary
    let recursive_circuit: serde_json::Value = serde_json::from_str(&recursive_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursive_circuit_bytecode = recursive_circuit["bytecode"].as_str().unwrap();

    setup_srs(String::from(recursive_circuit_bytecode), Some(LOCAL_SRS)).unwrap();

    // Outer proof circuit inputs
    let mut initial_witness_recursive = WitnessMap::new();
    let mut index = 0;
    // Verification key
    vk_as_fields.iter().for_each(|v| {
        initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(v).unwrap());
        index += 1;
    });
    // Proof
    proof_as_fields.iter().for_each(|v| {
        initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(v).unwrap());
        index += 1;
    });

    // Public inputs
    initial_witness_recursive.insert(Witness(index), FieldElement::from(INNER_X_ARG));
    index += 1;
    initial_witness_recursive.insert(Witness(index), FieldElement::from(FieldElement::try_from_str(INNER_2_OUT_ARG).unwrap()));
    index += 1;
    initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(&key_hash).unwrap());

    // Prove final outer proof
    let (proof, vk) = prove_honk(String::from(recursive_circuit_bytecode), initial_witness_recursive).unwrap();

    // Verify outer proof
    let verdict = verify_honk(proof, vk).unwrap();
    assert!(verdict);
}
