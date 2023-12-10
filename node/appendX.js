import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

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
  let tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: "secret1x94sfpan58qep4key25c964eudj7v3cq0rllu6",
      code_hash:
        "ac2bee9a2493b1356ebaf1eaf0daf0a4246f5b25c9594633c086ea37adf5e56c", // optional but way faster
      msg: {
        set_user_vector: {"value":valueToAdd },
      },
      sentFunds: [], // optional
    },
    {
      gasLimit: 200_000,
    }
  );
};

appendX();