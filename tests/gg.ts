import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Gg } from "../target/types/gg";

describe("gg", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Gg as Program<Gg>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
