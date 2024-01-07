import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();
import { contract_address, code_hash } from './contract_address.js'
const args = process.argv.slice(2);

const wallet = new Wallet(process.env.MNEMONIC);
const valueToAdd = parseInt(args[0], 10);

const contract_wasm = fs.readFileSync("../contract.wasm.gz");
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com/",
  wallet: wallet,
  walletAddress: wallet.address,
});

let appendX = async () => {
  const startTime = performance.now(); // Record start time
  
  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contract_address,
      code_hash: code_hash, // optional but way faster
      msg: {
        set_user_vector: { "value": valueToAdd },
      },
      sentFunds: [], // optional
    },
    {
      gasLimit: 200_000,
    }
  );

  const endTime = performance.now(); // Record end time

  const executionTime = endTime - startTime; // Calculate execution time
  console.log(`Transaction took ${executionTime} milliseconds to complete`);
};

appendX();
