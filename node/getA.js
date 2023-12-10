import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

let getA = async () => {
  const my_query = await secretjs.query.compute.queryContract({
    contract_address: "secret1x94sfpan58qep4key25c964eudj7v3cq0rllu6",
    code_hash:
      "ac2bee9a2493b1356ebaf1eaf0daf0a4246f5b25c9594633c086ea37adf5e56c",
    query: { get_admin_vector: {} },
  });

  console.log(my_query);
};

getA();