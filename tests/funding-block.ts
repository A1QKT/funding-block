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
import * as bs58 from "bs58";

describe("funding-block", () => {
  const provider = AnchorProvider.env();
  setProvider(provider);
  const program = workspace.FundingBlock as Program<FundingBlock>;
  const spl = Spl.token();

  it("Created Quest", async () => {
    //Token Mint address
    const mint = new web3.PublicKey(
      "44T2mgExB8FcfQFQx2DuGYhGrmA7gJxQCHvbMm5C98bC"
    ); // USDC devnet

    const programId = new web3.PublicKey(
      "Gfaf2UDB7aYzbBcZGHtAzyR3BtMubkm5EBhxGdzgwcs"
    );

    //Phantom wallet
    let secretKey: Uint8Array = bs58.decode(
      "4UqTC1SL7SQW4PqCysm6BBPZwLrZZ3hbx1LQu4krCxwuJ4YUfaMtz7yKofZarEc6GN6KmrZWJmZ8NZwShixBZH88"
    );
    const user = web3.Keypair.fromSecretKey(secretKey);

    // Program wallet
    const wallet = [
      149, 58, 9, 194, 124, 21, 185, 118, 215, 236, 225, 201, 251, 4, 64, 129,
      206, 100, 145, 48, 252, 63, 19, 152, 234, 83, 43, 107, 35, 145, 53, 89,
      233, 99, 208, 95, 206, 82, 133, 245, 216, 136, 137, 176, 166, 154, 212,
      40, 109, 92, 41, 158, 161, 247, 114, 35, 192, 145, 27, 205, 243, 211, 235,
      243,
    ];
    const programWallet = web3.Keypair.fromSecretKey(Uint8Array.from(wallet));

    const programWalletToken = await utils.token.associatedAddress({
      mint: mint,
      owner: programWallet.publicKey,
    });

    const userToken = await utils.token.associatedAddress({
      mint: mint,
      owner: user.publicKey,
    });

    const questAccount = web3.Keypair.generate();

    console.log(questAccount.publicKey);

    const [funderState, bump] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("funder_state"),
        user.publicKey.toBuffer(),
        questAccount.publicKey.toBuffer(),
      ],
      programId
    );

    const tx = await program.methods
      .createQuest("con cac", new BN("10"), new BN("1671123600"))
      .accounts({
        questAccount: questAccount.publicKey,
        funderState: funderState,
        programWallet: programWalletToken,
        user: user.publicKey,
        userToken,
        mintAccount: mint,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: utils.token.ASSOCIATED_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([user])
      .rpc();

    const questAccountData = await program.account.quest.fetch(
      questAccount.publicKey
    );
    console.log(questAccountData.title);
  });
});
