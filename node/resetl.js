import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();
import { contract_address, code_hash } from './contract_address.js'
const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm.gz");
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

let try_reset_array = async () => {
  const startTime = performance.now(); // Record start time

  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contract_address,
      code_hash:
        code_hash,
      msg: {
        reset_legit: {},
      },
      sentFunds: [], // optional
    },
    {
      gasLimit: 100_000,
    }
  );
  console.log("incrementing...");
  console.log(tx);
  const endTime = performance.now(); // Record end time

  const executionTime = endTime - startTime
  console.log(`tx took ${executionTime} milliseconds to complete`);

};

try_reset_array();