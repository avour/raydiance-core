import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Raydiance } from "../target/types/raydiance";
// import * as spl from '@solana/spl-token';
import { createMint } from "../tests/utils"

import {
  createAccount,
  createAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  createInitializeAccountInstruction,
  createInitializeMintInstruction,
  createMintToInstruction,
  getAccount,
  getAssociatedTokenAddress,
  getMint,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

// import { expect } from "chai";
import assert from "assert";

const TEST_SWAP_MARKET = new anchor.web3.PublicKey("Af4W1HNpMqzVyFmTxa3aWUWQUyKzmDAGoU819Prbsapv")
const TEST_DEX_PROGRAM = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")

interface PDAParameters {
  // LP vault is the PDA where LP tokens are being stored
  lpVaultKey: anchor.web3.PublicKey,
  lendingPoolKey: anchor.web3.PublicKey,
}

describe("raydiance", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Raydiance as Program<Raydiance>;


  let mintAddress: anchor.web3.PublicKey;
  let mintAuthority: anchor.web3.Keypair;
  let user: anchor.web3.Keypair;
  let userWallet: anchor.web3.PublicKey;

  let pda: PDAParameters;

  const getPdaParams = async (connection: anchor.web3.Connection, user: anchor.web3.PublicKey, serum_market: anchor.web3.PublicKey, lp_mint: anchor.web3.PublicKey): Promise<PDAParameters> => {
    const uid = new anchor.BN(parseInt((Date.now() / 1000).toString()));
    const uidBuffer = uid.toBuffer('le', 8);

    let [lendingPoolPubKey,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lending_pool"), serum_market.toBuffer(), lp_mint.toBuffer()], program.programId,
    );
    let [lpVaultKey,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_vault"), serum_market.toBuffer(), lp_mint.toBuffer()], program.programId,
    );
    return {
      lpVaultKey: lpVaultKey,
      lendingPoolKey: lendingPoolPubKey
    }
  }


  const createUserAndAssociatedWallet = async (connection: anchor.web3.Connection, mint?: anchor.web3.PublicKey): Promise<[anchor.web3.Keypair, anchor.web3.PublicKey | undefined]> => {
    const user = new anchor.web3.Keypair();
    let userAssociatedTokenAccount: anchor.web3.PublicKey | undefined = undefined;


    // Fund user with some SOL
    let txFund = new anchor.web3.Transaction();
    txFund.add(anchor.web3.SystemProgram.transfer({
      fromPubkey: provider.wallet.publicKey,
      toPubkey: user.publicKey,
      lamports: 5 * anchor.web3.LAMPORTS_PER_SOL,
    }));
    const sigTxFund = await provider.sendAndConfirm(txFund);
    console.log(`[${user.publicKey.toBase58()}] Funded new account with 5 SOL: ${sigTxFund}`);

    if (mint) {
      // Create a token account for the user and mint some tokens
      userAssociatedTokenAccount = await getAssociatedTokenAddress(
        mint,
        user.publicKey
      )
      const txFundTokenAccount = new anchor.web3.Transaction();
      txFundTokenAccount.add(createAssociatedTokenAccountInstruction(
        provider.wallet.publicKey,
        userAssociatedTokenAccount,
        user.publicKey,
        mint,
      ))


      txFundTokenAccount.add(createMintToInstruction(
        mint,
        userAssociatedTokenAccount,
        mintAuthority.publicKey,
        1337000000,
        [mintAuthority]

      ));

      const txFundTokenSig = await provider.sendAndConfirm(txFundTokenAccount, [mintAuthority]);
      // console.log(`[${userAssociatedTokenAccount.toBase58()}] New associated account for mint ${mint.toBase58()}: ${txFundTokenSig}`);
    }
    return [user, userAssociatedTokenAccount];
  }


  beforeEach(async () => {
    mintAuthority = anchor.web3.Keypair.generate();

    mintAddress = await createMint(provider, 6, mintAuthority.publicKey);

    [user, userWallet] = await createUserAndAssociatedWallet(provider.connection, mintAddress);

    pda = await getPdaParams(provider.connection, user.publicKey, TEST_SWAP_MARKET, mintAddress);
  });



  it("Can successfuly create a lending pool!", async () => {

    const userAccount = await getAccount(provider.connection, userWallet, undefined, TOKEN_PROGRAM_ID);
    console.log(userAccount.amount.toString());
    assert(userAccount.amount.toString(), '1337000000');


    const amount = new anchor.BN(20000000);

    // Initialize mint account and fund the account
    const tx1 = await program.methods.createPool(

    ).accounts({
      lendingPool: pda.lendingPoolKey,
      lpVaultState: pda.lpVaultKey,
      lpMint: mintAddress,
      user: userWallet,
      serumMarket: TEST_SWAP_MARKET,
      // dexProgram: 

      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,

    });
    console.log(`Lending Pool created`);

    // Assert that 20 tokens were moved from user's account to the escrow.
    // const [, userBalancePost] = await readAccount(userWallet, provider);
    // assert.equal(userBalancePost, '1317000000');

    // const lpVaultAccount = await getAccount(provider.connection, pda.lpVaultKey, undefined, TOKEN_PROGRAM_ID);
    // assert.equal(lpVaultAccount.amount.toString(), '20000000');

    // const lendingPoolAccount = await getAccount(provider.connection, pda.lendingPoolKey, undefined, TOKEN_PROGRAM_ID);
    // assert.equal(lendingPoolAccount.amount.toString(), '20000000');

    // assert.equal(state.stage.toString(), '1');
  });
});
