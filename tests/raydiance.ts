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
  collateralVault: anchor.web3.PublicKey,
  lendingPoolKey: anchor.web3.PublicKey,
  userCollateralConfigKey: anchor.web3.PublicKey,
}

describe("raydiance", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Raydiance as Program<Raydiance>;


  let mintAddress: anchor.web3.Keypair;
  let mintAuthority: anchor.web3.Keypair;
  // let user: anchor.web3.Keypair;
  let userLpTokenAccount: anchor.web3.PublicKey;
  const radianceMint = anchor.web3.Keypair.generate()
  let userRadianceTokenAccount: anchor.web3.PublicKey;
  let pda: PDAParameters;

  const getPdaParams = async (connection: anchor.web3.Connection, user: anchor.web3.PublicKey, serum_market: anchor.web3.PublicKey, lp_mint: anchor.web3.PublicKey): Promise<PDAParameters> => {
    const uid = new anchor.BN(parseInt((Date.now() / 1000).toString()));
    const uidBuffer = uid.toBuffer('le', 8);

    let [lendingPoolPubKey,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("lending_pool"), serum_market.toBuffer(), lp_mint.toBuffer()], program.programId,
    );

    let [collateralVault,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("collateral_vault"), serum_market.toBuffer(), lp_mint.toBuffer()], program.programId,
    );

    let [userCollateralConfigKey,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("user_collateral_config"), provider.wallet.publicKey.toBuffer(), serum_market.toBuffer(), lp_mint.toBuffer()], program.programId,
    );

    return {
      collateralVault: collateralVault,
      lendingPoolKey: lendingPoolPubKey,
      userCollateralConfigKey: userCollateralConfigKey
    }
  }


  const createUserAndAssociatedWallet = async (connection: anchor.web3.Connection, mint?: anchor.web3.Keypair): Promise<[anchor.web3.Keypair, anchor.web3.PublicKey | undefined]> => {
    // const user = new anchor.web3.Keypair();
    let userAssociatedTokenAccount: anchor.web3.PublicKey | undefined = undefined;


    // Fund user with some SOL
    // let txFund = new anchor.web3.Transaction();
    // txFund.add(anchor.web3.SystemProgram.transfer({
    //   fromPubkey: provider.wallet.publicKey,
    //   toPubkey: user.publicKey,
    //   lamports: 5 * anchor.web3.LAMPORTS_PER_SOL,
    // }));
    // const sigTxFund = await provider.sendAndConfirm(txFund);
    // console.log(`[${user.publicKey.toBase58()}] Funded new account with 5 SOL: ${sigTxFund}`);

    if (mint) {
      // Create a token account for the user and mint some tokens
      userAssociatedTokenAccount = await getAssociatedTokenAddress(
        mint.publicKey,
        provider.wallet.publicKey
      )
      const txFundTokenAccount = new anchor.web3.Transaction();
      txFundTokenAccount.add(createAssociatedTokenAccountInstruction(
        provider.wallet.publicKey,
        userAssociatedTokenAccount,
        provider.wallet.publicKey,
        mint.publicKey,
      ))


      txFundTokenAccount.add(createMintToInstruction(
        mint.publicKey,
        userAssociatedTokenAccount,
        mintAuthority.publicKey,
        1337000000,
        [mintAuthority]

      ));

      const txFundTokenSig = await provider.sendAndConfirm(txFundTokenAccount, [mintAuthority]);
      console.log(`[${userAssociatedTokenAccount.toBase58()}] New associated account for mint ${mint.publicKey.toBase58()}: ${txFundTokenSig}`);
    }
    return [, userAssociatedTokenAccount];
  }


  beforeEach(async () => {
    mintAuthority = anchor.web3.Keypair.generate();

    mintAddress = await createMint(provider, 9, mintAuthority.publicKey);

    [, userLpTokenAccount] = await createUserAndAssociatedWallet(provider.connection, mintAddress);

    pda = await getPdaParams(provider.connection, provider.wallet.publicKey, TEST_SWAP_MARKET, mintAddress.publicKey);

    userRadianceTokenAccount = await getAssociatedTokenAddress(
      radianceMint.publicKey,
      provider.wallet.publicKey,
    );

  });



  it("Can successfuly create a lending pool!", async () => {

    const userAccount = await getAccount(provider.connection, userLpTokenAccount, undefined, TOKEN_PROGRAM_ID);
    console.log(userAccount.amount.toString());
    assert(userAccount.amount.toString(), '1337000000');


    // Initialize mint account and fund the account
    const tx1 = await program.methods.createPool().accounts({
      lendingPool: pda.lendingPoolKey,
      collateralVault: pda.collateralVault,
      lpMint: mintAddress.publicKey,
      user: provider.wallet.publicKey,
      serumMarket: TEST_SWAP_MARKET,
      radianceMint: radianceMint.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .signers([radianceMint])
      .rpc();

    console.log(`Lending Pool created`);

    const lendingPool = await program.account.lendingPool.fetch(pda.lendingPoolKey);
    assert.equal(lendingPool.interestRate, 0);
    assert.equal(lendingPool.collateralVault.toBase58(), pda.collateralVault.toBase58());
    assert.equal(lendingPool.lpMint.toBase58(), mintAddress.publicKey.toBase58());

    const amount = new anchor.BN(20000000);

    // Initialize mint account and fund the account
    const tx2 = await program.methods.depositCollateral({
      amount: amount
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      collateralVault: pda.collateralVault,
      userCollecteralConfig: pda.userCollateralConfigKey,

      userLpTokenAccount: userLpTokenAccount,
      lpMint: mintAddress.publicKey,
      user: provider.wallet.publicKey,

      serumMarket: TEST_SWAP_MARKET,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();

    console.log(`Deposit Made`);
    const collateralVaultAccount = await getAccount(provider.connection, pda.collateralVault, undefined, TOKEN_PROGRAM_ID);
    assert.equal(collateralVaultAccount.amount.toString(), '20000000');

    /// TODO: that lp token has been deducted

    let userCollateralConfig = await program.account.userCollateralConfig.fetch(pda.userCollateralConfigKey);
    assert.equal(userCollateralConfig.user.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(userCollateralConfig.amount.toString(), amount.toString());

    console.log("Withdrawing Collateral");
    const tx3 = await program.methods.withdrawCollateral(amount).accounts({
      lendingPool: pda.lendingPoolKey,
      collateralVault: pda.collateralVault,
      userCollecteralConfig: pda.userCollateralConfigKey,

      userLpTokenAccount: userLpTokenAccount,
      lpMint: mintAddress.publicKey,
      user: provider.wallet.publicKey,
      serumMarket: TEST_SWAP_MARKET,

      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();

    userCollateralConfig = await program.account.userCollateralConfig.fetch(pda.userCollateralConfigKey);
    assert.equal(userCollateralConfig.user.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(userCollateralConfig.amount.toString(), '0');
    /// TODO: that lp token has been refunded and vault is deducted

  });


});
