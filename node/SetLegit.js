import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();
import { contract_address, code_hash } from './contract_address.js';

const args = process.argv.slice(2);
const wallet = new Wallet(process.env.MNEMONIC);

if (args.length === 0) {
  console.error("Please provide an argument for the value to add.");
  process.exit(1); // Exit the process with an error code
}

const valueToAdd = args[0];
const contract_wasm = fs.readFileSync("../contract.wasm.gz");
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com/",
  wallet: wallet,
  walletAddress: wallet.address,
});

let setlegit = async () => {
  const startTime = performance.now(); // Record start time

  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contract_address,
      code_hash: code_hash,
      msg: {
        set_legitim_users: { "address": valueToAdd },
      },

      sentFunds: [], // optional
    },
    {
      gasLimit: 200_000,
    }
  );
  console.log(tx);

  const endTime = performance.now(); // Record end time
  const executionTime = endTime - startTime;
  console.log(`Transaction took ${executionTime} milliseconds to complete`);
};

setlegit();
