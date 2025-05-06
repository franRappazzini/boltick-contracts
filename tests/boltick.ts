import * as anchor from "@coral-xyz/anchor";

import { Boltick } from "../target/types/boltick";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { expect } from "chai";

const SEED_CONFIG = "config";
const SEED_TREASURY = "treasury";
const SEED_EVENT = "event";
const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" // devnet and mainnet
);

// PARA EL metadata_program_account ES NECESARIO DESCARGAR EL PROGRAMA DE MAINNET Y EJECUTARLO LOCAL
// https://solana.com/es/developers/cookbook/development/using-mainnet-accounts-programs
/*
  #   solana program dump -u <source cluster> <address of account to fetch> <destination file name/path>
  solana program dump -u m PROGRAM_ID NAME.so

  # solana-test-validator --bpf-program <address to load the program to> <path to program file> --reset
  solana-test-validator --bpf-program PROGRAM_ID NAME.so --reset
*/

describe("boltick", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.Boltick as Program<Boltick>;

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  );
  console.log("Config PDA address:", configPda.toBase58());

  it("Should initialize Config Account!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeConfig().rpc();
    console.log("Initialize Config account tx signature:", tx);

    const configAccount = await program.account.config.fetch(configPda);

    expect(configAccount.eventCount.toNumber()).to.equal(0);
  });

  it("Should initialize Event Account!", async () => {
    const name = "Test Event";
    const symbol = "TE";
    const uri = "Test Event";
    const eventName = "Test Event";
    const eventId = 0;

    const tx = await program.methods
      .initializeEvent(name, symbol, uri, eventName)
      .accounts({ tokenProgram: TOKEN_PROGRAM_ID })
      .rpc({ skipPreflight: true });
    console.log("Initialize Event account tx signature:", tx);

    const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_EVENT), bn(eventId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    console.log("Event PDA address:", eventPda.toBase58());

    const eventAccount = await program.account.event.fetch(eventPda);
    const configAccount = await program.account.config.fetch(configPda);

    expect(eventAccount.name).to.equal(name);
    expect(configAccount.eventCount.toNumber()).to.equal(eventId + 1);
  });
});

function bn(n: number) {
  return new anchor.BN(n);
}
