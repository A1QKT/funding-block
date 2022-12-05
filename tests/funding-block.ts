import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FundingBlock } from "../target/types/funding_block";

describe("funding-block", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FundingBlock as Program<FundingBlock>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
