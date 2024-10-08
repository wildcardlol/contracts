# Wild Contracts Repository

To reproduce and test the Id registry contract:

1. Ensure Anchor version 0.30.1 is installed:
  ```
  anchor --version
  ```
  If not, install or update to this version.

2. Install dependencies:
  ```
  pnpm install
  ```

3. Update Admin:
  ```
  solana-keygen new -o ./admin.json
  ```
  Copy the generated address and update the admin id in `common/src/lib.rs`:
  ```rust
  pub mod admin {
      use anchor_lang::declare_id;
      declare_id!("<generates-address>");
  }
  ```
  Replace the existing id with your new address.
  This would let you intialize and manage gateway.

4. Sync program keys:
  ```
  anchor keys sync
  ```

5. Run the test
  ```
  anchor test --detach
  ```

These commands will set up the necessary environment and execute the test suite for the Id registry contract.
