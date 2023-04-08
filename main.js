import init, {
  create_proof,
  verify_zk_proof,
  create_json_proof,
} from "./pkg/arkworks_groth16_frontend.js";

// グローバル変数を追加
let currentProofVec = null;

async function runCreateProof() {
  document.getElementById("proofResult").innerText = "";
  document.getElementById("verificationResult").innerText = "";
  const a = parseInt(document.getElementById("inputA").value);
  const b = parseInt(document.getElementById("inputB").value);
  let proofVec;
  try {
    console.log("runCreateProof started");
    proofVec = await create_proof(a, b);

    console.log("Proof: ", proofVec);

    // 生成された proofVec をグローバル変数に格納
    currentProofVec = proofVec;
    console.log("runCreateProof finished");

    // Get proof JSON
    const proofJsonString = await create_json_proof(new Uint8Array(proofVec));
    console.log("Proof JSON string: ", proofJsonString);

    // Parse the JSON string to a JavaScript object
    const proofJson = JSON.parse(proofJsonString);
    console.log("Proof JSON: ", proofJson);

    // proof の結果を HTML に表示
    document.getElementById("proofResult").innerText = `${JSON.stringify(proofJson, null, 2)}`;

    // 背景色を変更
    document.getElementById("proofResult").style.backgroundColor = "white"; // 追加

  } catch (error) {
    console.error("Error creating proof: ", error);
  }
}

// Proof をコピーする機能を追加
function copyProofToClipboard() {
  const proofText = document.getElementById("proofResult").innerText;

  if (proofText === "") {
    return;
  }

  const textArea = document.createElement("textarea");
  textArea.value = proofText;
  document.body.appendChild(textArea);
  textArea.select();
  document.execCommand("copy");
  document.body.removeChild(textArea);
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
    // const publicInput = ["24"]; // この例では、6は証明された和です

    // verify_zk_proofを呼び出す
    const result = verify_zk_proof(new Uint8Array(currentProofVec));
    console.log("Verification result: ", result);
    console.log("runVerifyProof finished");
    // 検証結果を HTML に表示
    document.getElementById("verificationResult").textContent = `${result}`;
  } catch (error) {
    console.error("Error verifying proof: ", error);
  }
}

// Reset 機能を追加
function resetResults() {
  document.getElementById("proofResult").innerText = "";
  document.getElementById("proofResult").style.backgroundColor = "transparent"; // 追加
  document.getElementById("verificationResult").innerText = "";
  currentProofVec = null;
}


// wasmモジュールの初期化とボタンイベントリスナーの追加
async function main() {
  await init();

  document.getElementById("createProofButton").addEventListener("click", runCreateProof);
  document.getElementById("verifyProofButton").addEventListener("click", runVerifyProof);
  document.getElementById("resetButton").addEventListener("click", resetResults); // Reset ボタンのイベントリスナーを追加
  document.getElementById("proofResult").addEventListener("click", copyProofToClipboard); // proofResult のイベントリスナーを追加
}

window.addEventListener("load", main);