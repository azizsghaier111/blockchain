import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
import fs from "fs/promises";

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
let vector = JSON.parse(args[0]);

console.log('Parsed vector:', vector);

const setAdminVector = async () => {
  try {
    const tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: "secret1x94sfpan58qep4key25c964eudj7v3cq0rllu6",
        code_hash: "ac2bee9a2493b1356ebaf1eaf0daf0a4246f5b25c9594633c086ea37adf5e56c",
        msg: {
          set_admin_vector: { admin_vector: vector.filter(value => value !== null) },
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
};

setAdminVector();
