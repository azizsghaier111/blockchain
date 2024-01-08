import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();
import { contract_address, code_hash } from './contract_address.js'
const wallet = new Wallet(process.env.MNEMONIC);

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});
let dotAX = async () => {
  const startTime = performance.now(); // Record start time

  const my_query = await secretjs.query.compute.queryContract({
    contract_address: contract_address,
    code_hash:
      code_hash,
    query: { get_dot_product: {} },
  });

  console.log(atob(my_query)) ; 
  const endTime = performance.now(); // Record end time

  const executionTime = endTime - startTime
  console.log(`query took ${executionTime} milliseconds to complete`);

};

dotAX();