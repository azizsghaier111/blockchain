import { SecretNetworkClient, Wallet } from "secretjs";
import fs from "fs/promises";
import dotenv from "dotenv";

dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);
const contractWasm = await fs.readFile("../contract.wasm.gz");
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});
const customFees = {
  upload: {
    amount: [{ amount: "2000000", denom: "uscrt" }],
    gas: "2000000",
  },
  init: {
    amount: [{ amount: "500000", denom: "uscrt" }],
    gas: "500000",
  },
};

async function uploadContract() {
  try {
    let tx = await secretjs.tx.compute.storeCode(
      {
        sender: wallet.address,
        wasm_byte_code: contractWasm,
        source: "",
        builder: "",
      },
      {
        gasLimit: 4_000_000,
      }
    );

    const codeId = Number(
      tx.arrayLog.find(
        (log) => log.type === "message" && log.key === "code_id"
      ).value
    );

    console.log("codeId: ", codeId);

    const contractCodeHash = (
      await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
    ).code_hash;
    console.log(`Contract hash: ${contractCodeHash}`);

    console.log("Instantiating contract...");
    // Create an instance of the Counter contract, providing a starting count
    const initMsg = { count: 0 };
    let ty = await secretjs.tx.compute.instantiateContract(
      {
        code_id: codeId,
        sender: wallet.address,
        code_hash:
          contractCodeHash,
        init_msg: initMsg,
        label: "Linear_regression" + Math.ceil(Math.random() * 10000),
      },
      {
        gasLimit: 400_000,
      }
    );
    console.log("tx", ty);
  
    //Find the contract_address in the logs
    const contractAddress = ty.arrayLog.find(
      (log) => log.type === "message" && log.key === "contract_address"
    ).value;
  
    console.log(contractAddress);
  } catch (error) {
    console.error('Error in uploadContract:', error);
  }
}

uploadContract();

