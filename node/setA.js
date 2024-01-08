import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
import fs from "fs/promises";
import { contract_address, code_hash } from './contract_address.js';
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contractWasm = await fs.readFile("../contract.wasm.gz");

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

const setAdminVector = async () => {
  const startTime = performance.now(); // Record start time

  try {
    const tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: contract_address,
        code_hash: code_hash,
        msg: {
          set_admin_vector: { a_vector: vector.filter(value => !isNaN(value)) },
        },
        sentFunds: [],
      },
      {
        gasLimit: 200_000,
      }
    );

    console.log("Transaction:", tx);
  } catch (error) {
    console.error('Error setting admin vector:', error);
  }
  const endTime = performance.now(); // Record end time

  const executionTime = endTime - startTime
  console.log(`Transaction took ${executionTime} milliseconds to complete`);

};

setAdminVector() ;
