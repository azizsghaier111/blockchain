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
    secretcli tx compute execute secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"set_user_vector": {"value": 42}}' --from myWallet --gas 1000000 --gas-prices=1uscrt
    #see what is in X
    secretcli query compute query secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve  '{"get_user_vector": {}}'
    #set A only by admin
    secretcli tx compute execute secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"set_admin_vector": {"admin_vector": [1, 2, 3, 4]}}' --from myWallet --gas 200000 --gas-prices=0.25uscrt --broadcast-mode block
    #see what is in A
    secretcli query compute query secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve  '{"get_admin_vector": {}}'
    #get the dot
    secretcli query compute query secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"get_dot_product":{}}' --from myWallet
    #add legitimated user 
    secretcli tx compute execute secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"set_legitim_users": {"address": "secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4sdsdsd"}}' --from myWallet --gas 1000000 --gas-prices=1uscrt
    #see what is in the legit vector 
    secretcli query compute query secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"get_legit_vector":{}}' 

    #Get The average value:
    secretcli query compute query secret1adrmc3yxe0c2rwgw7w07v27nz6ad6wdr3amez0 '{"get_average": {}}'


        secretcli tx compute execute secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"reset_x": {}}' --from myWallet --gas 1000000 --gas-prices=1uscrt

        secretcli tx compute execute secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"reset_admin": {}}' --from myWallet --gas 1000000 --gas-prices=1uscrt

        secretcli tx compute execute secret1zc4rka7r8y3mpan5p80a6zuxy9s4x4g0jrp5ve '{"reset_legit": {}}' --from myWallet --gas 1000000 --gas-prices=1uscrt