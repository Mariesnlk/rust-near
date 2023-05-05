# Minting NFT

## Installed Rust before build
'''# Get Rust in linux and MacOS
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
'''

# Add the wasm toolchain
rustup target add wasm32-unknown-unknown

## Build contract
'''
yarn build
'''

## Connect to the testnet wallet
'''
near login
'''

'''
export NFT_CONTRACT_ID="YOUR_ACCOUNT_NAME"
'''

"YOUR_ACCOUNT_NAME" should be like '.testnet'

'''
echo $NFT_CONTRACT_ID
'''

## Deploy

'''
near deploy --wasmFile out/main.wasm --accountId $NFT_CONTRACT_ID
'''

## Initializing the contract
'''
near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID
'''

## Viewing the contract's metadata
'''
near view $NFT_CONTRACT_ID nft_metadata
'''