use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::from_value;
use console_error_panic_hook;
use serde_json::{json, Value};

use ark_bn254::{Bn254, Fr};
use ark_groth16::{generate_random_parameters, prepare_verifying_key, create_random_proof, verify_proof, Proof};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_serialize::CanonicalSerialize;
use ark_serialize::CanonicalDeserialize;
use ark_ff::fields::PrimeField;

// 追加
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

pub struct Circuit {
    pub a: Option<Fr>,
    pub b: Option<Fr>,
    pub c: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for Circuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| {
            self.a
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        let b = cs.new_witness_variable(|| {
            self.b
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        let c = cs.new_input_variable(|| {
            self.c
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
        a: Some(Fr::from(4)),
        b: Some(Fr::from(2)),
        c: Some(Fr::from(6)),
    };

    let public_input = assignment.c.clone().ok_or_else(|| JsValue::from_str("Failed to get public input"))?; // 変更
    
    let public_inputs = &[public_input]; // 変更
    web_sys::console::log_1(&JsValue::from_str(&format!("Public inputs: {:?}", public_inputs)));

    // let proof = create_random_proof(assignment, &pk, rng).unwrap();
    let proof = create_random_proof(assignment, &pk, rng)
    .map_err(|e| JsValue::from_str(&format!("Failed to create random proof: {:?}", e)))?; // 変更

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();

    Ok(proof_vec)
}

#[wasm_bindgen]
pub fn create_json_proof(proof_vec: Vec<u8>) -> Result<String, JsValue> {
    let proof = Proof::<Bn254>::deserialize(&proof_vec[..])
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize proof: {:?}", e)))?;

    let a: Vec<String> = vec![
        format!("{:?}", proof.a.x.into_repr()),
        format!("{:?}", proof.a.y.into_repr()),
    ];
    let b: Vec<Vec<String>> = vec![
        vec![
            format!("{:?}", proof.b.x.c0.into_repr()),
            format!("{:?}", proof.b.x.c1.into_repr()),
        ],
        vec![
            format!("{:?}", proof.b.y.c0.into_repr()),
            format!("{:?}", proof.b.y.c1.into_repr()),
        ],
    ];
    let c: Vec<String> = vec![
        format!("{:?}", proof.c.x.into_repr()),
        format!("{:?}", proof.c.y.into_repr()),
    ];

    let proof_json = json!({
        "curve": "bn254",
        "protocol": "groth16",
        "a": {
            "infinity": proof.a.infinity,
            "x": a[0],
            "y": a[1]
        },
        "b": {
            "infinity": proof.b.infinity,
            "x": {
                "c0": b[0][0],
                "c1": b[0][1]
            },
            "y": {
                "c0": b[1][0],
                "c1": b[1][1]
            }
        },
        "c": {
            "infinity": proof.c.infinity,
            "x": c[0],
            "y": c[1]
        }
    });

    Ok(serde_json::to_string(&proof_json).unwrap())
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
