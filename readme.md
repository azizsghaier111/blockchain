```bash
#Add new wallet:
secretcli keys add myWallet
#Consult the balance 
secretcli query bank balances "secret16fn60y0seepp2dsyzks3e4qm2jvnq3lef09tvd"
#Fund The Wallet:
curl http://localhost:5000/faucet?address=secret16fn60y0seepp2dsyzks3e4qm2jvnq3lef09tvd
#convert .rs to .wasm
make build
#compress the contract
docker run --rm -v "$(pwd)":/contract \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  enigmampc/secret-contract-optimizer  
#Upload a contract
secretcli tx compute store contract.wasm.gz --gas 5000000 --from myWallet --chain-id secretdev-1
#List of contract
secretcli query compute list-code
#Instantiate the contract:
secretcli tx compute instantiate 1 '{"count": 1}' --from myWallet --label Reg -y
#get information about the instantiated contreact
secretcli query compute list-contract-by-code 1
#Queries and Tx: 

    #append to the X 
    secretcli tx compute execute secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0 '{"set_user_vector": {"value": 42}}' --from myWallet --gas 1000000 --gas-prices=1uscrt
    #see what is in X
    secretcli query compute query secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0  '{"get_user_vector": {}}'
    #see what is in A
    secretcli query compute query secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0  '{"get_A_vector": {}}'
    #Reset the Average variable:
    secretcli tx compute execute secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0 '{"reset": {}}' --from FakeWallet --gas 1000000 --gas-prices=1uscrt

    #Get The average value:
    secretcli query compute query secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0 '{"get_average": {}}'
```
secret16fn60y0seepp2dsyzks3e4qm2jvnq3lef09tvd
emerge palace like crunch doctor between tornado logic tell casual feature minimum creek liquid tonight hard feel raise wrong scrub feed ahead protect flag



contract address
secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0