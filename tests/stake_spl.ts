import * as anchor from "@coral-xyz/anchor";

import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

import { Program } from "@coral-xyz/anchor";
import { SEED_CONFIG } from "./utils/constants";
import { StakeSpl } from "../target/types/stake_spl";
import { parseConfigAccount } from "./utils/parsers";

describe("stake program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet;
  const randomKeypair = anchor.web3.Keypair.generate();
  let boltMint: PublicKey;

  anchor.setProvider(provider);

  const program = anchor.workspace.StakeSpl as Program<StakeSpl>;

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  );
  console.log("Config PDA address:", configPda.toBase58());

  before(async () => {
    // boltMint = await createMint(connection, wallet.payer, wallet.publicKey, null, 9);
    boltMint = new PublicKey("72iLTJ7PMemAmF3m4kuSrVd3oDKgkxeY9NvRL2AU6yYN");
    console.log("BOLT Mint Address:", boltMint.toBase58());

    // airdrop to the random keypair
    await connection.requestAirdrop(
      randomKeypair.publicKey,
      0.1 * LAMPORTS_PER_SOL // 0.1 SOL
    );

    const walletBoltAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      boltMint,
      wallet.publicKey
    );

    const randomBoltAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      boltMint,
      randomKeypair.publicKey
    );

    // mint to wallet
    const tx = await mintTo(
      connection,
      wallet.payer,
      boltMint,
      walletBoltAta.address,
      wallet.publicKey,
      1000 * 1e9 // Minting 1 billion tokens (9 decimals)
    );

    // mint to random keypair
    const tx2 = await mintTo(
      connection,
      wallet.payer,
      boltMint,
      randomBoltAta.address,
      wallet.publicKey,
      800 * 1e9 // Minting 800 million tokens (9 decimals)
    );

    console.log("Minted 1000 tokens to wallet ATA:", tx);
    console.log("Minted 800 tokens to random ATA:", tx2);
  });

  it("Should initialize the stake program Config account!", async () => {
    const tx = await program.methods
      .initializeConfig()
      .accounts({
        boltMint: boltMint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Initialize Config tx signature:", tx);
  });

  it("Should deposit BOLT to stake!", async () => {
    const amount = bn(100 * 1e9);

    const tx = await program.methods
      .depositStake(amount)
      .accounts({
        boltMint,
        depositor: randomKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([randomKeypair])
      .rpc({ skipPreflight: true });

    console.log("Deposit Stake tx signature:", tx);

    const configAccount = await program.account.config.fetch(configPda);
    console.log("Config account after deposit:", parseConfigAccount(configAccount));
  });
});

function bn(n: number) {
  return new anchor.BN(n);
}
