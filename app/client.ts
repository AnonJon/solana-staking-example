import { Connection, Keypair } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { StakingPoolManager } from "../target/types/staking_pool_manager";
import {
  PublicKey,
  ConfirmOptions,
  Commitment,
  SystemProgram,
} from "@solana/web3.js";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const wallet = anchor.Wallet.local();
const POOL_ID = 1;

const program = anchor.workspace
  .StakingPoolManager as anchor.Program<StakingPoolManager>;

const createPool = async () => {
  console.log(`Finding pool...`);
  let [poolAccountAddress] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("pool", "utf8"),
      new anchor.BN(POOL_ID).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  try {
    // Get pool to check if it already exists
    const poolAccount = await program.account.pool.fetch(poolAccountAddress);
    console.log(
      `Pool already exists, id: ${poolAccount.id}, accountAddress: ${poolAccountAddress}`
    );
  } catch {
    let tx = await program.methods
      .createPool(
        new anchor.BN(POOL_ID),
        new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")
      )
      .accounts({
        authority: program.provider.publicKey,
        pool: poolAccountAddress,
        systemProgram: SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc();
    console.log(
      `Created new pool id: ${POOL_ID}, accountAddress: ${poolAccountAddress}`
    );
  }
  return poolAccountAddress;
};
const main = async () => {
  createPool();
};

main();
