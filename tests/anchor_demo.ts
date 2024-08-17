import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { AnchorDemo } from "../target/types/anchor_demo";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("anchor_demo", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorDemo as Program<AnchorDemo>;
  const wallet = provider.wallet as anchor.Wallet;

  it("Is initialized!", async () => {
    const data = new BN(42);
    const newAccountKp = new Keypair();
    const tx = await program.methods
      .initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: wallet.publicKey,
      })
      .signers([newAccountKp])
      .rpc();
    const currentNewAccount = await program.account.newAccount.fetch(
      newAccountKp.publicKey
    );
    assert(currentNewAccount.data.eq(data), "Expected data to be 42");
    console.log("Your transaction signature", tx);
  });
});
