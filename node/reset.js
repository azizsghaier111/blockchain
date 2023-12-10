import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync("../contract.wasm.gz");
const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

let try_reset_array = async () => {
  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: "secret1x94sfpan58qep4key25c964eudj7v3cq0rllu6",
      code_hash:
        "ac2bee9a2493b1356ebaf1eaf0daf0a4246f5b25c9594633c086ea37adf5e56c",
      msg: {
        reset_array: {},
      },
      sentFunds: [], // optional
    },
    {
      gasLimit: 100_000,
    }
  );
  console.log("incrementing...");
  console.log(tx);
};

try_reset_array();