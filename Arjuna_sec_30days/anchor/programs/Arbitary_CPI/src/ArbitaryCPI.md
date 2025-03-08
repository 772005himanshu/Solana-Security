## What is CPI (Cross Program Innovation):

- CPI is the mechanism that allows one one-chain program(smart contract) to call the functionality of another program
- When making a CPI, program use teh runtime functions invoke for standard calls and invoked_signed when a program needs to `sign` for a Program Derived Address (PDA)
- By Leveraging CPIs , developers can build more complex applications by reusing ell-trusted and audited code from the other programs (such as SPL Token program)
  
Because of CPI in solana Reentrancy is not possible In Solana Max depth of the contract a function is `MAx = 4` times re-enter

## 🦀How does CPI work?
1. The calling program build an instruction with the target program ID, account metedata and encoded parameter

2. It choose between `invoke` for standard calls or `invoked_signed` when a PDA must sign using The seed data

3. The program gathers and passes all necessary AccountInfo object
4. The solana runtime validates account permission and verifies PDA seeds if needed
5. The target instruction executes sync, applying state changes atomically and returning a result.

## Vulnerability of Arbitary CPI calls happens when a program invokes another program without verifying that the target program is the one it intends to call

- ⭕ Invoking a CPI with your signer seeds or privilages is like executing every statement in that instruction. If the protocolis malicious and mimics the context and function names, you're essentially giving your signer privileges to do anything , leading to catastrophic effects.

- ⭕ For instance , instead of invoking the SPL Token Program Transfer instruction , your program might inadvertently invoke a custom, attacker-controlled program , resulting in unexpected state changes, draining funds or compromising the logic of your contract


## How to identify issue releated to Arbitary CPI:

1. If your program does not explicitly checks the public key of teh program being invoked via CPI
2. Manually invoking CPI via `solana_program::program::invoke` or `invoke_signed` without leveraging the Anchor built-in CPI modules increase the risk of missing program checks
3. While using ANchor: Using unchecked `Account` and `AccountInfo` for the program ID.
