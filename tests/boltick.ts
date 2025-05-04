import * as anchor from "@coral-xyz/anchor";

import { Boltick } from "../target/types/boltick";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

const SEED_CONFIG = "config";
const SEED_TREASURY = "treasury";

describe("boltick", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.Boltick as Program<Boltick>;

  it("Should initialize Config Account!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeConfig().rpc();
    console.log("Initialize Config account tx signature:", tx);

    const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_CONFIG)],
      program.programId
    );
    console.log("Config PDA address:", configPda.toBase58());

    const configAccount = await program.account.config.fetch(configPda);
    // console.log("Config Account:", configAccount);

    expect(configAccount.eventCount.toNumber()).to.equal(0);
  });
});
