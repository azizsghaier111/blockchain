import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();
import { contract_address, code_hash } from './contract_address.js'


const wallet = new Wallet(process.env.MNEMONIC);


const contract_wasm = fs.readFileSync("../contract.wasm.gz");
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com/",
  wallet: wallet,
  walletAddress: wallet.address,
});
const args = process.argv.slice(2);
console.log('Input arguments:', args);
let vector = args.map(Number);

console.log('Parsed vector:', vector);
let setX = async () => {
  const startTime = performance.now(); // Record start time
  
  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contract_address,
      code_hash: code_hash, // optional but way faster
      msg: {
        set_user_vector: { "vector": vector.filter(value => !isNaN(value)) },
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

setX();
