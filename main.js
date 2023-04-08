import init, {
  create_proof,
  verify_zk_proof,
} from "./pkg/arkworks_groth16_frontend.js";

// グローバル変数を追加
let currentProofVec = null;

async function runCreateProof() {
  let proofVec; // proofVec を関数スコープの先頭で宣言
  try {
    console.log("runCreateProof started");
    // proofVec = create_proof();
    proofVec = create_proof();

    console.log("Proof: ", proofVec);

    // 生成された proofVec をグローバル変数に格納
    currentProofVec = proofVec;
    console.log("runCreateProof finished");

    // proof の結果を HTML に表示
    document.getElementById("proofResult").textContent = `Proof: ${proofVec}`;
  } catch (error) {
    console.error("Error creating proof: ", error);
  }
}

async function runVerifyProof() {
  try {
    console.log("runVerifyProof started");
    // 以前に生成された proofVec（currentProofVec）を使用
    if (currentProofVec === null) {
      console.log("No proof to verify. Please create a proof first.");
      return;
    }

    // 公開入力を設定
    const publicInput = ["6"]; // この例では、6は証明された和です

    // verify_zk_proofを呼び出す
    const result = verify_zk_proof(new Uint8Array(currentProofVec), publicInput);
    console.log("Verification result: ", result);
    console.log("runVerifyProof finished");
  } catch (error) {
    // 検証結果を HTML に表示
    document.getElementById("verificationResult").textContent = `Verification Result: ${result}`;
  }
}

// wasmモジュールの初期化とボタンイベントリスナーの追加
async function main() {
  await init();

  document.getElementById("createProofButton").addEventListener("click", runCreateProof);
  document.getElementById("verifyProofButton").addEventListener("click", runVerifyProof);
}

main();