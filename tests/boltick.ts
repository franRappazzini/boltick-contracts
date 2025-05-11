import * as anchor from "@coral-xyz/anchor";

import { ComputeBudgetProgram, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";

import { Boltick } from "../target/types/boltick";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { expect } from "chai";

const SEED_CONFIG = "config";
const SEED_TREASURY = "treasury";
const SEED_EVENT = "event";
const SEED_COLLECTION_MINT = "collection_mint";
const SEED_TOKEN_MINT = "token_mint";
const SEED_DIGITAL_ACCESS = "digital_access";
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
  const randomKeypair = anchor.web3.Keypair.generate();
  let firstCollectionAddress: string;

  anchor.setProvider(provider);

  const program = anchor.workspace.Boltick as Program<Boltick>;

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(SEED_CONFIG)],
    program.programId
  );
  console.log("Config PDA address:", configPda.toBase58());

  before(async () => {
    // Airdrop SOL to the random keypair
    const signature = await connection.requestAirdrop(
      randomKeypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    console.log("Airdrop signature:", signature);
  });

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
    const uri =
      "https://raw.githubusercontent.com/franRappazzini/boltick-contracts/main/tests/utils/uri-test.json";
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

    const [collectionPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_COLLECTION_MINT), bn(eventId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    firstCollectionAddress = collectionPda.toBase58();
    console.log("Collection PDA address:", firstCollectionAddress);

    const eventAccount = await program.account.event.fetch(eventPda);
    const configAccount = await program.account.config.fetch(configPda);

    expect(eventAccount.name).to.equal(name);
    expect(configAccount.eventCount.toNumber()).to.equal(eventId + 1);
  });

  it("Should create Digital Access types!", async () => {
    const eventId = 0;
    const price = 0.2 * LAMPORTS_PER_SOL;
    const max_supply = 1;
    const name = "VIP Access";
    const symbol = "VIP";
    const description = "VIP Access to the event";
    const uri =
      "https://raw.githubusercontent.com/franRappazzini/boltick-contracts/main/tests/utils/uri-test.json";

    const tx = await program.methods
      .addDigitalAccess(bn(eventId), bn(price), bn(max_supply), name, symbol, description, uri)
      .rpc();
    console.log("Add VIP Digital Access tx signature:", tx);

    const price2 = 0.1 * LAMPORTS_PER_SOL;
    const max_supply2 = 3;
    const name2 = "General Access";
    const symbol2 = "GA";
    const description2 = "General Access to the event";

    const tx2 = await program.methods
      .addDigitalAccess(bn(eventId), bn(price2), bn(max_supply2), name2, symbol2, description2, uri)
      .rpc();
    console.log("Add GA Digital Access tx signature:", tx2);

    const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_EVENT), bn(eventId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const eventAccount = await program.account.event.fetch(eventPda);

    const digitalAccessAccounts = await program.account.digitalAccess.all([
      {
        memcmp: {
          offset: 8,
          bytes: eventPda.toBase58(),
        },
      },
    ]);
    console.log({ eventAccount });
    digitalAccessAccounts.forEach((da) => console.log(da.account));

    expect(eventAccount.currentDigitalAccessCount).to.equal(2);
  });

  it("Should mint new token with the authority of Config account!", async () => {
    const eventId = 0;
    const digitalAccessId = 0;

    // create instruction to set compute unit limit
    const computeIx = ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 });

    const ix = await program.methods
      .mintToken(bn(eventId), digitalAccessId)
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        destination: randomKeypair.publicKey,
      })
      .instruction();

    const tx = new Transaction().add(computeIx, ix);

    tx.feePayer = wallet.publicKey;
    tx.recentBlockhash = (await provider.connection.getLatestBlockhash()).blockhash;

    const signature = await provider.sendAndConfirm(tx, [wallet.payer]);
    console.log("Mint token tx signature:", signature);

    // fetch the event account to check the currentNftCount and to fetch nft address
    const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_EVENT), bn(eventId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const eventAccount = await program.account.event.fetch(eventPda);

    // fetch the new nft address
    const nftId = eventAccount.currentNftCount.toNumber() - 1;

    const [nftPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_TOKEN_MINT),
        eventAccount.collectionMintAccount.toBuffer(),
        bn(nftId).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    console.log("NFT PDA address:", nftPda.toBase58());

    expect(eventAccount.currentNftCount.toNumber()).to.equal(nftId + 1);
    expect(eventAccount.collectionMintAccount.toBase58()).to.equal(firstCollectionAddress);
  });

  it("Should buy a token from randomKeypair wallet!", async () => {
    const eventId = 0;
    const digitalAccessId = 1;

    // get creator balance to then compare it
    const creatorPrevBalance = await provider.connection.getBalance(wallet.publicKey);

    const computeIx = ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 });

    const ix = await program.methods
      .buyToken(bn(eventId), digitalAccessId)
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        eventCreator: wallet.publicKey,
        buyer: randomKeypair.publicKey,
      })
      .signers([randomKeypair])
      .instruction();

    const tx = new Transaction().add(computeIx, ix);

    tx.feePayer = randomKeypair.publicKey;
    tx.recentBlockhash = (await provider.connection.getLatestBlockhash()).blockhash;

    const signature = await anchor.web3.sendAndConfirmTransaction(connection, tx, [randomKeypair], {
      skipPreflight: true,
    });
    // const signature = await provider.sendAndConfirm(tx, [randomKeypair], { skipPreflight: true });
    console.log("Buy token tx signature:", signature);

    // fetch the event account to check the currentNftCount and to fetch nft address
    const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_EVENT), bn(eventId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const eventAccount = await program.account.event.fetch(eventPda);

    // fetch the new nft address
    const nftId = eventAccount.currentNftCount.toNumber() - 1;

    const [nftPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(SEED_TOKEN_MINT),
        eventAccount.collectionMintAccount.toBuffer(),
        bn(nftId).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    console.log("NFT PDA address:", nftPda.toBase58());

    // check the creator balance after the transaction
    const creatorPostBalance = await provider.connection.getBalance(wallet.publicKey);

    expect(eventAccount.currentNftCount.toNumber()).to.equal(nftId + 1);
    expect(eventAccount.collectionMintAccount.toBase58()).to.equal(firstCollectionAddress);
    expect(creatorPrevBalance).to.be.lessThan(creatorPostBalance);
  });

  it("Should fail buy a token due to a wrong event_creator!", async () => {
    const eventId = 0;
    const digitalAccessId = 1;

    const computeIx = ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 });

    try {
      const ix = await program.methods
        .buyToken(bn(eventId), digitalAccessId)
        .accounts({
          tokenProgram: TOKEN_PROGRAM_ID,
          eventCreator: randomKeypair.publicKey,
          buyer: randomKeypair.publicKey,
        })
        .signers([randomKeypair])
        .instruction();

      const tx = new Transaction().add(computeIx, ix);

      tx.feePayer = randomKeypair.publicKey;
      tx.recentBlockhash = (await provider.connection.getLatestBlockhash()).blockhash;

      const signature = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [randomKeypair],
        { skipPreflight: true }
      );
      console.error("Unexpected buy token tx signature:", signature);
      return expect.fail("Unexpected buy token tx signature:", signature);
    } catch (err) {
      console.log("Expected error buying token tx signature:", err?.signature);
      return expect(err).to.be.instanceOf(Error);
    }
  });

  it("Should fail mint a token due to limit exceeded!", async () => {
    const eventId = 0;
    const digitalAccessId = 0; // VIP Access = only 1 token (already minted in previous test)

    try {
      const ix = await program.methods
        .mintToken(bn(eventId), digitalAccessId)
        .accounts({
          tokenProgram: TOKEN_PROGRAM_ID,
          destination: randomKeypair.publicKey,
        })
        .instruction();

      const tx = new Transaction().add(ix);

      tx.feePayer = wallet.publicKey;
      tx.recentBlockhash = (await provider.connection.getLatestBlockhash()).blockhash;

      const signature = await provider.sendAndConfirm(tx, [wallet.payer], { skipPreflight: true });
      console.error("Unexpected mint token tx signature:", signature);
      return expect.fail("Unexpected mint token tx signature:", signature);
    } catch (err) {
      console.log("Expected error minting token:", err);
      return expect(err).to.be.instanceOf(Error);
    }
  });

  it("Should update token metadata!", async () => {
    const eventId = 0;
    const nftId = 0;

    const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_EVENT), bn(eventId).toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const [digitalAccessPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SEED_DIGITAL_ACCESS), eventPda.toBuffer(), Uint8Array.from([nftId])],
      program.programId
    );

    const digitalAccessAccount = await program.account.digitalAccess.fetch(digitalAccessPda);

    const uri =
      "https://raw.githubusercontent.com/franRappazzini/boltick-contracts/main/tests/utils/uri-test-update.json";

    // 10' to wait for the transaction to check old metadata in explorer
    setTimeout(async () => {
      const tx = await program.methods
        .updateTokenMetadata(
          bn(eventId),
          bn(nftId),
          digitalAccessAccount.name,
          digitalAccessAccount.symbol,
          uri
        )
        // .accounts({})
        .rpc({ skipPreflight: true });

      console.log("Update token metadata tx signature:", tx);
    }, 10000);
  });
});

function bn(n: number) {
  return new anchor.BN(n);
}
