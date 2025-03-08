## 🦀Understanding the issue of bump seed Canonicalization using `create_program_address` function in @solana

```Rust 
pub fn validate_pda_usage(
    provided_pda: &Pubkey,
    program_id: &Pubkey,
    seeds: &[&[u8]],
    bump: u8,  // Input why ?? 
) -> ProgramResult {
    // Build the seeds array by appending the bump seed.

    let bump_slice = &[bump];
    let mut seeds_with_bump: Vec<&[u8]> = seeds.to_vec();

    seeds_with_bump.push(bump_slice);

    // Compute the PDA using the given seeds and program_id.
    let computed_pda = Pubkey::create_program_address(seeds_with_bump.as_slice(), program_id)?;  // @audit here is the issue

    // compare teh computed PDA  with the provided PDA
    if computed_pda == *provided_pda {
        msg!("PDA validation successful.");
        Ok(())
    }

    else{
        msg!("PDA validation failed, Expected: {}, Provided: {}", computed_pda,provided_pda);
        Err(ProgramError::InvalidArgument);
        Ok(())
    }
```


A PDA (Program Derived Address) is an off-curve , deterministically generated address - created from the program ID combined with specified seeds and a bump seed - that has no private key and enables a program to securely sign.

PDAs are mainly used to store custom state in accordance with main program.

## Flow chart
I read  this from sea-level attack about PDA derived from the Bump seed + seeds Array + Program ID 

Program ID -> Seed Array -> Bump Seed -> Concentrate Inputs Program ID + Seeds + Bump -> Hash with SHA-512 --> Is hash off-curve 
--> Yes -> PDA Program Derived Address 
--> No -> Adjust Bump Seed and Repeat the process(i think i read somewhere to subtract - 1) highest se lowest ki taraf aate hain



## What is the Bump Seeds?

Bump seeds are numerical value adjusted during the PDA derivation until the resulting address is off the `ed25519` curve , ensuring it has no private key (uses a hit and trial method)

These value ranges from 0-255.

Where deriving a pda, we always choose the most canonical bump(the highest) value to derive the pda.


## Deriving the address of a PDA using two function;

1. `find_program_address`:
   - Take program ID and seeds to derive the PDA with most canonical bump, ensuring it's secure and safe to use.


2. `create_program_address`:
   - Take the Program ID , seeds and a custom bump to derive the PDA.If the PDA can reside off the `ed25519`
   curve , it also returns a valid PDA


## ⭕ Issue with usage of custom bump in create_program_address?

THe create_program_address function derives a PDA but does not search for the canonical bump. It allows multiple valid bumps, producing different address. This lacks determinism since multiple bumps can yield different address for the same seeds.

## 🔥How can an attacker exploit the given contract?

- A user can create a PDA with teh custom bump, making it valid . The User can then use `validate_pda_function` to ensure only their PDA is used in the contract logic.

- However the attacker can create another PDA using `create_pda_account` with a different bump. Since `validate_pda_usage` accepts a custom bump, the attacker can by pass security by providing their bump, validating their PDA, using it in various custom logic of the Program.

In short , the restricted provided by `validating_pda_usage` to only allow user-created PDA be easily bypassed.

## Remediation

- Use `find_program_address` fro validating and creating a program address ,as it uses the most canonical bump.
- If using `create_program_address` do not allow user-provided custom bumps; instead, save the bump in the PDA and use it to derive the PDA similarly as anchor does it.


