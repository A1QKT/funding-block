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

describe("funding-block", () => {
  const provider = AnchorProvider.env();
  setProvider(provider);
  const program = workspace.FundingBlock as Program<FundingBlock>;
  const spl = Spl.token();
  let accountAddress;
  let solutionAddress;

  const programId = new web3.PublicKey(
    "9RgWo49pJ9pD24QkBMFTJ1J6RQdHbLoTa4J65V3n8eKm"
  );

  it("Created Quest", async () => {
    //Token Mint address
    // const mint = new web3.PublicKey(
    //   "44T2mgExB8FcfQFQx2DuGYhGrmA7gJxQCHvbMm5C98bC"
    // ); // USDC devnet
    // "DFgPm8yvJuKmnXqpSRqVXKiwbSnZ5vTtgPipgJXofXyu"

    //Huynh token
    const mint = new web3.PublicKey(
      "prjctRCEC7Pu5ryxvpW7Z5EfdbJN7puFfMcXdgheRGc"
    );

    // Phantom wallet
    // let secretKey: Uint8Array = bs58.decode(
    //   "4UqTC1SL7SQW4PqCysm6BBPZwLrZZ3hbx1LQu4krCxwuJ4YUfaMtz7yKofZarEc6GN6KmrZWJmZ8NZwShixBZH88"
    // );
    // const user = web3.Keypair.fromSecretKey(secretKey);

    // Program wallet
    // const wallet = [
    //   149, 58, 9, 194, 124, 21, 185, 118, 215, 236, 225, 201, 251, 4, 64, 129,
    //   206, 100, 145, 48, 252, 63, 19, 152, 234, 83, 43, 107, 35, 145, 53, 89,
    //   233, 99, 208, 95, 206, 82, 133, 245, 216, 136, 137, 176, 166, 154, 212,
    //   40, 109, 92, 41, 158, 161, 247, 114, 35, 192, 145, 27, 205, 243, 211, 235,
    //   243,
    // ];
    // const programWallet = web3.Keypair.fromSecretKey(Uint8Array.from(wallet));

    // const programWalletToken = await utils.token.associatedAddress({
    //   mint: mint,
    //   owner: programWallet.publicKey,
    // });

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

    // await initializeAccount(
    //   userToken,
    //   mint,
    //   provider.wallet.publicKey,
    //   provider
    // );
    console.log(`client wallet: ${userToken}`);
    console.log(`progam wallet: ${programWallet}`);

    const questAccount = web3.Keypair.generate();

    const [funderState, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("funder_state"),
        provider.wallet.publicKey.toBuffer(),
        questAccount.publicKey.toBuffer(),
      ],
      programId
    );

    const tx = await program.methods
      .createQuest("con cac", new BN("1"), new BN("1671123600"))
      .accounts({
        questAccount: questAccount.publicKey,
        funderState: funderState,
        programWallet: programWalletToken,
        user: provider.wallet.publicKey,
        userToken,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([questAccount])
      .rpc();

    console.log(`tx: ${tx}, quest: ${questAccount.publicKey}`);
    accountAddress = questAccount.publicKey;

    const questAccountData = await program.account.quest.fetch(
      questAccount.publicKey
    );
    console.log(`con cac: ${questAccountData.title}`);
  });

  it("Create Solution", async () => {
    const questAccount = new web3.PublicKey(accountAddress);

    const [solutionAccount, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("solution_account"),
        provider.wallet.publicKey.toBuffer(),
        questAccount.toBuffer(),
      ],
      programId
    );

    const tx = await program.methods
      .createSolution("cmm beo", "link ne")
      .accounts({
        questAccount: questAccount,
        solutionAccount: solutionAccount,
        user: provider.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    console.log(`tx: ${tx}, quest: ${solutionAccount}`);
    solutionAddress = solutionAccount;

    const solutionAccountData = await program.account.solution.fetch(
      solutionAccount
    );
    console.log(`con cac: ${solutionAccountData.content}`);
  });

  it("Fund Quest", async () => {});

  it("Join Quest", async () => {});

  it("Vote", async () => {});

  it("UnVote", async () => {});

  it("Transfer Rewarding", async () => {});

  it("Close Quest", async () => {});

  it("Send Fund Back", async () => {});
});
