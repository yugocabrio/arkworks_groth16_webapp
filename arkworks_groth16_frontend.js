import { run, create_proof, verify_proof_js } from './pkg/your_project_name.js';

// 以下の関数を追加
export async function createProof() {
  try {
    const proofVec = await create_proof();
    console.log('Proof created:', proofVec);
    return proofVec;
  } catch (error) {
    console.error('Error creating proof:', error);
    return null;
  }
}

export async function verifyProof(proofVec, publicInput) {
  try {
    const result = await verify_proof_js(proofVec, publicInput);
    console.log('Proof verification result:', result);
    return result;
  } catch (error) {
    console.error('Error verifying proof:', error);
    return false;
  }
}

export async function runRust() {
  run();
}

