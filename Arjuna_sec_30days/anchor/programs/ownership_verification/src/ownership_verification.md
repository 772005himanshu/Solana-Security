## Understanding Ownership Verification in Native Solana and how Anchor abstracts the process

Hint : There eis missing verification check , allowing to withdraw from the vault account.

Every account in solana can be represented by an AccountInfo struct 

Metadata for an account/pubkey in an instruction includes:
- key : the pubkey of the account
- is_signer : whether the account is a signer of the transaction
- is_writable : whether the account can be modified by the instruction
- lamports: available lamports
- data: data stored on chain
- executable: if account holds executable code

