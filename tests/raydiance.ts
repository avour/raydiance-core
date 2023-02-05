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

const swapMarket = new anchor.web3.PublicKey("Af4W1HNpMqzVyFmTxa3aWUWQUyKzmDAGoU819Prbsapv")
const dexProgram = new anchor.web3.PublicKey("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX")
// const borrowableQuoteMint = new anchor.web3.PublicKey("6x5cVSbkYFeQ78WCCLBtrTKaGNMhLkVujJQPBE1VNkRv") //
// const borrowableBaseMint = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112") // SOL
const accountBalance = 133700000000000;

interface PDAParameters {
  // LP vault is the PDA where LP tokens are being stored
  poolID: anchor.BN,
  collateralVault: anchor.web3.PublicKey,
  lendingPoolKey: anchor.web3.PublicKey,
  userCollateralConfigKey: anchor.web3.PublicKey,
  borrowableBaseVault: anchor.web3.PublicKey,
  borrowableQuoteVault: anchor.web3.PublicKey,
}

describe("raydiance", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Raydiance as Program<Raydiance>;

  let lpMint: anchor.web3.Keypair;
  let baseRadianceMint = anchor.web3.Keypair.generate();
  let quoteRadianceMint = anchor.web3.Keypair.generate();

  let borrowableBaseMint: anchor.web3.Keypair;
  let borrowableQuoteMint: anchor.web3.Keypair;


  let mintAuthority: anchor.web3.Keypair;
  let userLpTokenAccount: anchor.web3.PublicKey;

  let userRadianceTokenAccount: anchor.web3.PublicKey;
  let userBorrowableTokenAccount: anchor.web3.PublicKey;
  let pda: PDAParameters;


  const getPdaParams = async (connection: anchor.web3.Connection, user: anchor.web3.PublicKey, serum_market: anchor.web3.PublicKey, lp_mint: anchor.web3.PublicKey): Promise<PDAParameters> => {
    const id = parseInt((Date.now() / 1000).toString());
    const uid = new anchor.BN(id);
    const uidBuffer = uid.toBuffer('le', 8);
    console.log("POOL ID: ", id)
    

    let [lendingPoolPubKey,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("lending_pool"), serum_market.toBuffer(), uidBuffer], program.programId,
    );


    let [collateralVault,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("collateral_vault"), serum_market.toBuffer(), uidBuffer], program.programId,
    );

    let [userCollateralConfigKey,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("user_collateral_config"), provider.wallet.publicKey.toBuffer(), serum_market.toBuffer(), uidBuffer], program.programId,
    );

    let [borrowableBaseVault,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("borrowable_vault"), serum_market.toBuffer(), borrowableBaseMint.publicKey.toBuffer(), uidBuffer], program.programId,
    );

    let [borrowableQuoteVault,] = await anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("borrowable_vault"), serum_market.toBuffer(), borrowableQuoteMint.publicKey.toBuffer(), uidBuffer], program.programId,
    );


    return {
      poolID: uid,
      collateralVault: collateralVault,
      lendingPoolKey: lendingPoolPubKey,
      userCollateralConfigKey: userCollateralConfigKey,
      borrowableBaseVault, borrowableQuoteVault,
    }
  }

  // Fund user with some SOL
  // let txFund = new anchor.web3.Transaction();
  // txFund.add(anchor.web3.SystemProgram.transfer({
  //   fromPubkey: provider.wallet.publicKey,
  //   toPubkey: user.publicKey,
  //   lamports: 5 * anchor.web3.LAMPORTS_PER_SOL,
  // }));
  // const sigTxFund = await provider.sendAndConfirm(txFund);
  // console.log(`[${user.publicKey.toBase58()}] Funded new account with 5 SOL: ${sigTxFund}`);

  const createAssociatedWallet = async (connection: anchor.web3.Connection, mint: anchor.web3.Keypair, authority?: anchor.web3.Keypair): Promise<anchor.web3.PublicKey | undefined> => {
    let userAssociatedTokenAccount: anchor.web3.PublicKey | undefined = undefined;

    if (mint) {
      // Create a token account for the user and mint some tokens
      userAssociatedTokenAccount = await getAssociatedTokenAddress(
        mint.publicKey,
        provider.wallet.publicKey
      )
      const tx1 = new anchor.web3.Transaction();
      tx1.add(createAssociatedTokenAccountInstruction(
        provider.wallet.publicKey,
        userAssociatedTokenAccount,
        provider.wallet.publicKey,
        mint.publicKey,
      ))

      await provider.sendAndConfirm(tx1);

      if (authority) {
        console.log("oops")
        const tx2 = new anchor.web3.Transaction();
        tx2.add(createMintToInstruction(
          mint.publicKey,
          userAssociatedTokenAccount,
          authority.publicKey,
          accountBalance,
          [authority]

        ));
        await provider.sendAndConfirm(tx2, [authority]);

      }


      console.log(`[${userAssociatedTokenAccount.toBase58()}] New associated account for mint ${mint.publicKey.toBase58()}`);
    }
    return userAssociatedTokenAccount;
  }


  beforeEach(async () => {
    mintAuthority = anchor.web3.Keypair.generate();
    let borrowableMintAuthority = anchor.web3.Keypair.generate();

    lpMint = await createMint(provider, 9, mintAuthority.publicKey);

    borrowableBaseMint = await createMint(provider, 9, borrowableMintAuthority.publicKey);
    borrowableQuoteMint = await createMint(provider, 9, borrowableMintAuthority.publicKey);

    pda = await getPdaParams(provider.connection, provider.wallet.publicKey, swapMarket, lpMint.publicKey);

    userLpTokenAccount = await createAssociatedWallet(provider.connection, lpMint, mintAuthority);

    userBorrowableTokenAccount = await createAssociatedWallet(provider.connection, borrowableBaseMint, borrowableMintAuthority);
    console.log("Are we here yet");

  });



  it("Can successfuly create a lending pool!", async () => {
    const userAccount = await getAccount(provider.connection, userLpTokenAccount, undefined, TOKEN_PROGRAM_ID);
    console.log(userAccount.amount.toString());
    assert(userAccount.amount.toString(), accountBalance.toString());


    // Initialize mint account and fund the account
    await program.methods.createPool({
      poolId: pda.poolID,
      safetyMargin: new anchor.BN("250"),
      liquidationIncentive: new anchor.BN("104"),
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      collateralVault: pda.collateralVault,
      lpMint: lpMint.publicKey,
      borrowableBaseVault: pda.borrowableBaseVault,
      borrowableQuoteVault: pda.borrowableQuoteVault,

      baseRadianceMint: baseRadianceMint.publicKey,
      quoteRadianceMint: quoteRadianceMint.publicKey,

      borrowableBaseMint: borrowableBaseMint.publicKey,
      borrowableQuoteMint: borrowableQuoteMint.publicKey,

      user: provider.wallet.publicKey,
      serumMarket: swapMarket,
      // dexProgram: dexProgram,

      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .signers([baseRadianceMint, quoteRadianceMint])
      .rpc();

    console.log(`Lending Pool created`);

    const lendingPool = await program.account.lendingPool.fetch(pda.lendingPoolKey);
    assert.equal(lendingPool.quoteInterestRate, 0);
    assert.equal(lendingPool.collateralVault.toBase58(), pda.collateralVault.toBase58());
    assert.equal(lendingPool.lpMint.toBase58(), lpMint.publicKey.toBase58());

    const amount = new anchor.BN(10000000);

    console.log(`Deposit Collateral`);
    // Initialize mint account and fund the account
    await program.methods.depositCollateral({
      poolId: pda.poolID,
      amount,
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      collateralVault: pda.collateralVault,
      userCollecteralConfig: pda.userCollateralConfigKey,

      userLpTokenAccount: userLpTokenAccount,
      lpMint: lpMint.publicKey,
      user: provider.wallet.publicKey,

      serumMarket: swapMarket,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();

    console.log(`Deposit Made`);
    const collateralVaultAccount = await getAccount(provider.connection, pda.collateralVault, undefined, TOKEN_PROGRAM_ID);
    assert.equal(collateralVaultAccount.amount.toString(), amount.toString());

    /// TODO: that lp token has been deducted

    let userCollateralConfig = await program.account.userCollateralConfig.fetch(pda.userCollateralConfigKey);
    assert.equal(userCollateralConfig.user.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(userCollateralConfig.collateralDeposited.toString(), amount.toString());


    console.log("Supplying Borrowable")
    userRadianceTokenAccount = await createAssociatedWallet(provider.connection, baseRadianceMint);
    await program.methods.supplyBorrowable({
      mintType: { base: {} },
      poolId: pda.poolID,
      amount,
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      borrowableVault: pda.borrowableBaseVault,
      borrowableMint: borrowableBaseMint.publicKey,
      radianceMint: baseRadianceMint.publicKey,
      userBorrowableTokenAccount: userBorrowableTokenAccount,
      userRadianceTokenAccount: userRadianceTokenAccount,

      user: provider.wallet.publicKey,

      serumMarket: swapMarket,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();
    const tokenAccount4 = await getAccount(provider.connection, userBorrowableTokenAccount, undefined, TOKEN_PROGRAM_ID);
    assert.equal(tokenAccount4.amount.toString(), (new anchor.BN(accountBalance).sub(amount)).toString());

    console.log("Taking out a loan")
    await program.methods.borrow({
      poolId: pda.poolID,
      mintType: { base: {} },
      amount,
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      userCollecteralConfig: pda.userCollateralConfigKey,
      borrowableVault: pda.borrowableBaseVault,
 
      borrowableMint: borrowableBaseMint.publicKey,
      userBorrowableTokenAccount: userBorrowableTokenAccount,
      user: provider.wallet.publicKey,

      serumMarket: swapMarket,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();

    console.log("Loan Application successful")
    const tokenAccount = await getAccount(provider.connection, userBorrowableTokenAccount, undefined, TOKEN_PROGRAM_ID);
    assert.equal(tokenAccount.amount.toString(), accountBalance.toString());

    console.log("Repaying Loan")
    await program.methods.repayLoan({
      poolId: pda.poolID,
      mintType: { base: {} },
      amount,
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      userCollecteralConfig: pda.userCollateralConfigKey,
      borrowableVault: pda.borrowableBaseVault,

      borrowableMint: borrowableBaseMint.publicKey,
      userBorrowableTokenAccount: userBorrowableTokenAccount,
      user: provider.wallet.publicKey,

      serumMarket: swapMarket,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();
      console.log("Loan Repayment successful")
      const tokenAccount2 = await getAccount(provider.connection, userBorrowableTokenAccount, undefined, TOKEN_PROGRAM_ID);
      assert.equal(tokenAccount2.amount.toString(), (new anchor.BN(accountBalance).sub(amount)).toString());
  

    console.log("Lender Withdrawing Borrowable")
    await program.methods.withdrawBorrowable({
      mintType: { base: {} },
      poolId: pda.poolID,
      amount,
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      borrowableVault: pda.borrowableBaseVault,

      borrowableMint: borrowableBaseMint.publicKey,
      radianceMint: baseRadianceMint.publicKey,
      userBorrowableTokenAccount: userBorrowableTokenAccount,
      userRadianceTokenAccount: userRadianceTokenAccount,

      user: provider.wallet.publicKey,

      serumMarket: swapMarket,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();

    const tokenAccount3 = await getAccount(provider.connection, userBorrowableTokenAccount, undefined, TOKEN_PROGRAM_ID);
    assert(tokenAccount3.amount.toString(), accountBalance.toString());
  

    console.log("Withdrawing Collateral");
    await program.methods.withdrawCollateral({
      poolId: pda.poolID,
      amount,
    }).accounts({
      lendingPool: pda.lendingPoolKey,
      collateralVault: pda.collateralVault,
      userCollecteralConfig: pda.userCollateralConfigKey,

      userLpTokenAccount: userLpTokenAccount,
      lpMint: lpMint.publicKey,
      user: provider.wallet.publicKey,
      serumMarket: swapMarket,

      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .rpc();
    console.log("Collateral Withdrawn");
    const userAccount2 = await getAccount(provider.connection, userLpTokenAccount, undefined, TOKEN_PROGRAM_ID);
    assert(userAccount2.amount.toString(), accountBalance.toString());

    userCollateralConfig = await program.account.userCollateralConfig.fetch(pda.userCollateralConfigKey);
    assert.equal(userCollateralConfig.user.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(userCollateralConfig.collateralDeposited.toString(), '0');
    /// TODO: that lp token has been refunded and vault is deducted

  });
});
