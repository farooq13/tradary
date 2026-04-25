import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TradaryProgram } from "../target/types/tradary_program";

describe("tradary-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tradaryProgram as Program<TradaryProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
