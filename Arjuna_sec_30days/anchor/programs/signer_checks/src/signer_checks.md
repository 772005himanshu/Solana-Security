## This Vulnerability is like Access Control in EVM

If signer checks were no applied properly any user can exploit the contract.

Hint Solana : There is a Vulnerability in this contract which allows any one to be admin of the pda


## Where checks are included to protect from this vulnerability.

- `AccountInfo` struct is low-level representation of an on-chain account. It encapsulates both the account's metadata (such as its public key, lamports balance, and ownership) and its raw data,
- Every wallet account on solana is represented by this `AccountInfo` struct 
- `AccountInfo` struct has various fields one of them is signer which responds if the account has signed the transaction.
- When using native solana,this check should be always validated!!

## Remediation

Use the `.is_signer` check in solana to ensure signer check is enforced

