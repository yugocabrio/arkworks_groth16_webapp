use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::from_value;

use ark_bn254::{Bn254, Fr};
use ark_groth16::{generate_random_parameters, prepare_verifying_key, create_random_proof, verify_proof, Proof};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_serialize::CanonicalSerialize;
use ark_serialize::CanonicalDeserialize;

pub struct Circuit {
    pub a: Option<String>,
    pub b: Option<String>,
    pub c: Option<String>,
}

impl ConstraintSynthesizer<Fr> for Circuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| {
            self.a
                .as_ref()
                .and_then(|s| s.parse::<Fr>().ok())
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        let b = cs.new_witness_variable(|| {
            self.b
                .as_ref()
                .and_then(|s| s.parse::<Fr>().ok())
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        let c = cs.new_input_variable(|| {
            self.c
                .as_ref()
                .and_then(|s| s.parse::<Fr>().ok())
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        cs.enforce_constraint(lc!() + a + b, lc!() + Variable::One, lc!() + c)?;

        Ok(())
    }
}

#[wasm_bindgen]
pub fn create_proof() -> Result<Vec<u8>, JsValue> {
    let rng = &mut StdRng::seed_from_u64(0u64);

    let pk = {
        let c = Circuit {
            a: None,
            b: None,
            c: None,
        };
        generate_random_parameters::<Bn254, _, _>(c, rng).unwrap()
    };

    let assignment = Circuit {
        a: Some(Fr::from(4).to_string()),
        b: Some(Fr::from(2).to_string()),
        c: Some(Fr::from(6).to_string()),
    };

    let public_input = &[assignment.c.as_ref().and_then(|s| s.parse::<Fr>().ok()).unwrap()];

    let proof = create_random_proof(assignment, &pk, rng).unwrap();

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();

    Ok(proof_vec)
}

#[wasm_bindgen]
pub fn verify_zk_proof(proof_vec: &[u8], public_input_js: JsValue) -> Result<bool, JsValue> {
    let rng = &mut StdRng::seed_from_u64(0u64);

    let c = Circuit {
        a: None,
        b: None,
        c: None,
    };
    
    let params = generate_random_parameters::<Bn254, _, _>(c, rng).unwrap();
    let vk = prepare_verifying_key(&params.vk);
    
    let proof = match Proof::deserialize(proof_vec) {
        Ok(p) => p,
        Err(_) => return Err(JsValue::from_str("Failed to deserialize proof")),
    };
    
    let public_input: Vec<String> = from_value(public_input_js).map_err(|_| JsValue::from_str("Failed to parse public input"))?;
    let public_input: Vec<Fr> = public_input.into_iter().filter_map(|s| s.parse::<Fr>().ok()).collect();
    
    let result = verify_proof(&vk, &proof, &public_input).unwrap();
    Ok(result)
}
