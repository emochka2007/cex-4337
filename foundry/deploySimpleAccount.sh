forge create --rpc-url http://localhost:8545 \
             --private-key '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80' \
             contracts/accounts/SimpleAccount.sol:SimpleAccount \
             --broadcast \
             --constructor-args '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512'
