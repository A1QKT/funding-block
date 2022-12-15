import {
  AnchorProvider,
  setProvider,
  workspace,
  Spl,
  web3,
  Program,
  BN,
  utils,
} from "@project-serum/anchor";
import { FundingBlock } from "../target/types/funding_block";
import { initializeAccount } from "./utils";
import * as bs58 from "bs58";

describe("funding-block", async () => {
  const provider = AnchorProvider.env();
  setProvider(provider);
  const program = workspace.FundingBlock as Program<FundingBlock>;
  const spl = Spl.token();
  let accountAddress;
  let solutionAddress;
  const programId = new web3.PublicKey(
    "9RgWo49pJ9pD24QkBMFTJ1J6RQdHbLoTa4J65V3n8eKm"
  );
  const mint = new web3.PublicKey(
    "prjctRCEC7Pu5ryxvpW7Z5EfdbJN7puFfMcXdgheRGc"
  );
  //Huynh wallet
  const programWallet = new web3.PublicKey(
    "DTijKdweRy1GNVAGp8ZFvt15qqytd29wDPNndPQ1yJiY"
  );

  const programWalletToken = await utils.token.associatedAddress({
    mint: mint,
    owner: programWallet,
  });

  const userToken = await utils.token.associatedAddress({
    mint: mint,
    owner: provider.wallet.publicKey,
  });

  // it("Created Quest", async () => {
  //   //Huynh token

  //   // await initializeAccount(
  //   //   userToken,
  //   //   mint,
  //   //   provider.wallet.publicKey,
  //   //   provider
  //   // );
  //   console.log(`client wallet: ${userToken}`);
  //   console.log(`progam wallet: ${programWallet}`);

  //   const questAccount = web3.Keypair.generate();

  //   const [funderState, bump] = await web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("funder_state"),
  //       provider.wallet.publicKey.toBuffer(),
  //       questAccount.publicKey.toBuffer(),
  //     ],
  //     programId
  //   );

  //   const tx = await program.methods
  //     .createQuest("con cac", new BN("1"), new BN("1671123600"))
  //     .accounts({
  //       questAccount: questAccount.publicKey,
  //       funderState: funderState,
  //       programWallet: programWalletToken,
  //       user: provider.wallet.publicKey,
  //       userToken,
  //       systemProgram: web3.SystemProgram.programId,
  //       tokenProgram: utils.token.TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
  //       rent: web3.SYSVAR_RENT_PUBKEY,
  //     })
  //     .signers([questAccount])
  //     .rpc();

  //   console.log(`tx: ${tx}, quest: ${questAccount.publicKey}`);
  //   accountAddress = questAccount.publicKey;

  //   const questAccountData = await program.account.quest.fetch(
  //     questAccount.publicKey
  //   );
  //   console.log(`con cac: ${questAccountData.title}`);
  // });

  // it("Create Solution", async () => {
  //   const questAccount = new web3.PublicKey(accountAddress);

  //   const [solutionAccount, bump] = await web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("solution_account"),
  //       provider.wallet.publicKey.toBuffer(),
  //       questAccount.toBuffer(),
  //     ],
  //     programId
  //   );

  //   const tx = await program.methods
  //     .createSolution("cmm beo", "link ne")
  //     .accounts({
  //       questAccount: questAccount,
  //       solutionAccount: solutionAccount,
  //       user: provider.wallet.publicKey,
  //       systemProgram: web3.SystemProgram.programId,
  //     })
  //     .signers([])
  //     .rpc();

  //   console.log(`tx: ${tx}, solution: ${solutionAccount}`);
  //   solutionAddress = solutionAccount;

  //   const solutionAccountData = await program.account.solution.fetch(
  //     solutionAccount
  //   );
  //   console.log(`content solution: ${solutionAccountData.content}`);
  // });

  // it("Test", async () => {
  //   const questAccount = new web3.PublicKey(
  //     "AqBHXiKHaqgjZGTBJpoiXen77vWmEFEpy2wzxiYbkZtX"
  //   );

  //   const [solutionAccount, bump] = await web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("solution_account"),
  //       provider.wallet.publicKey.toBuffer(),
  //       questAccount.toBuffer(),
  //     ],
  //     programId
  //   );

  //   const tx = await program.methods
  //     .updateSolution("cmm beo", "link ne")
  //     .accounts({
  //       questAccount: questAccount,
  //       solutionAccount: solutionAccount,
  //       user: provider.wallet.publicKey,
  //     })
  //     .signers([])
  //     .rpc();

  //   const solutionAccountData = await program.account.solution.fetch(
  //     "DTEJ29yuqvQhHiafdguouM7dc8nC39nPzaRFZBT6iidV"
  //   );
  //   console.log(`content solution: ${solutionAccountData.content}`);
  // });

  it("Fund Quest", async () => {
    const questAccount = new web3.PublicKey(
      "AqBHXiKHaqgjZGTBJpoiXen77vWmEFEpy2wzxiYbkZtX"
    );
    const questAccountData = await program.account.quest.fetch(questAccount);
    console.log(`Fund Before: ${questAccountData.fund}`);

    const [funderState, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("funder_state"),
        provider.wallet.publicKey.toBuffer(),
        questAccount.toBuffer(),
      ],
      programId
    );

    const tx = await program.methods
      .fundQuest(new BN("1"))
      .accounts({
        questAccount: questAccount,
        programWallet: programWalletToken,
        user: provider.wallet.publicKey,
        userToken,
        funderState,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([])
      .rpc();

    const questAccountDataAfter = await program.account.quest.fetch(
      "AqBHXiKHaqgjZGTBJpoiXen77vWmEFEpy2wzxiYbkZtX"
    );
    console.log(`Fund After: ${questAccountDataAfter.fund}`);
  });

  it("Join Fund", async () => {
    const questAccount = new web3.PublicKey(
      "AqBHXiKHaqgjZGTBJpoiXen77vWmEFEpy2wzxiYbkZtX"
    );
  });

  it("Vote", async () => {});

  it("UnVote", async () => {});

  it("Transfer Rewarding", async () => {});

  it("Close Quest", async () => {});

  it("Send Fund Back", async () => {});
});
