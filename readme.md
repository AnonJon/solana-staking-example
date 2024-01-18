# Solana Staking Example

## Setup

Generate new wallet to test with

```bash
solana-keygen new -o ./key.json
```

Run a test validator in a new terminal

```bash
solana-test-validator
```

Airdrop tokens to your new keypair

```bash
solana airdrop 10 --keypair ./key.json --url http://localhost:8899
```

Install Anchor

## Deploy on Localnet

```bash
 anchor deploy --provider.cluster localnet
```

Run a demo

```bash
anchor run client
```

Possible Gotchas

```bash
Error: AnchorError occurred. Error Code: DeclaredProgramIdMismatch. Error Number: 4100. Error Message: The declared program id does not match the actual program id.
```

Make sure you take the deployed programId from the terminal and update the `declare_id` in the program, then build then redeploy.
