// Loop through a u8 array and convert each element to a Field
fn u8_array_to_field_array<N>(u8_array: [u8; N]) -> [Field; N] {
    let mut field_array: [Field; N] = [0; N];
    for i in 0..N {
        field_array[i] = u8_array[i] as Field;
    }
    field_array
}

#[recursive]
fn main(x: pub Field, out: pub Field) {
    let digest = std::hash::keccak256([x as u8], 1);
    let poseidon_hash = std::hash::poseidon2::Poseidon2::hash(u8_array_to_field_array(digest), digest.len());
    assert(poseidon_hash == out);
}

#[test]
fn test_main() {
    main(1234, 0x12a62eebe17a2a4852ebcb89fbcba38e5bd980e95f579b68405090352babc73c);
}
