https://www.erc4337.io/docs/understanding-ERC-4337/architecture
- Cover the fees for the user
- Contract per user
- Social Recovery Wallets 
- Users can interact with blockchain without holding ETH first.
- Institutions can create wallets with governance rules (multisig, threshold signatures, permissions).
- Funds are transparently segregated and managed securely via smart contracts.


## Installation libs
- forge install foundry-rs/forge-std --no-git
- forge install OpenZeppelin/openzeppelin-contracts-upgradeable --no-git
- forge install OpenZeppelin/openzeppelin-foundry-upgrades --no-git

### Summary
#### Improved UX: 
- gas sponsorship, fee abstraction, seamless onboarding.

#### Better Security: 
- social recovery, multifactor auth, spending limits.

#### Convenience: 
- batch transactions, recurring payments, token fee payments.

#### Institutional Integration: 
- governance, multisig, permissioned usage.