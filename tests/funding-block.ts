import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FundingBlock } from "../target/types/funding_block";
import { Keypair, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("funding-block", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FundingBlock as Program<FundingBlock>;

  it("Created Quest", async () => {
    const name = "Quest 1";

    const questKeyPair = anchor.web3.Keypair.generate();
    const [investorPDA, investorBump] = await PublicKey.findProgramAddress(
      [
        Buffer.from("investor_quest"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from("id_2"),
      ],
      program.programId
    );
    console.log("con cho rach {}", questKeyPair.publicKey);
    const balance = await provider.connection.getBalance(
      provider.wallet.publicKey
    );
    console.log("dmmm {}", balance / LAMPORTS_PER_SOL);

    const tx = await program.methods
      .createQuest(name, "id_2")
      .accounts({
        questAccount: questKeyPair.publicKey,
        investorQuest: investorPDA,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([questKeyPair])
      .rpc();

    const questAccount = await program.account.quest.fetch(
      questKeyPair.publicKey
    );
    console.log(questAccount.title);
    console.log(questAccount.timeStart.toNumber());
    console.log(questAccount.numInvestor.toNumber());
    console.log("Your transaction signature", tx);
  });
});
