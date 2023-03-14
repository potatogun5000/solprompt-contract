import * as anchor from "@project-serum/anchor";
import * as SplToken from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { Prompt3 } from "../target/types/prompt3";
import { getAirdrop } from "./util";
import bs58 from "bs58";
import assert from "assert";
import idl from '../target/idl/prompt3.json';

const { web3 } = anchor;
const {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  mintTo,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
} = SplToken;
import {
  getPdaWithBump,
  getPda,
  getAirdrop,
  accountIndexFind,
  getDynamicPda,
  sleep,
} from "./util";
import { PublicKey } from "@solana/web3.js";

const PROGRAM_ID = anchor.web3.SystemProgram.programId;

describe("prompt3", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const admin = provider.wallet.payer;
  const program = anchor.workspace.Prompt3 as Program<Prompt3>;
  const sellerKP = anchor.web3.Keypair.generate();
  const buyerKP = anchor.web3.Keypair.generate();
  let sellerPda;
  let statePda;
  let listingPda;
  let buyerPda;
  let receiptPda;
  let sellerBalance;
  let adminBalance;
  let buyerBalance;

  it("set stuff up", async () => {
    await getAirdrop(provider, sellerKP.publicKey);
    await getAirdrop(provider, admin.publicKey);
    await getAirdrop(provider, buyerKP.publicKey);

    sellerPda = await getPda(program, "seller", [sellerKP.publicKey]);
    listingPda = await getDynamicPda(program, "listing", sellerKP.publicKey, 0);
    buyerPda = await getPda(program, "buyer", [buyerKP.publicKey]);
    receiptPda = await getDynamicPda(program, "receipt", buyerKP.publicKey, 0);

    statePda = await getPda(program, "state", []);

    console.log("listing", listingPda.toBase58());
    console.log("seller", sellerPda.toBase58());
    console.log("buyerPda", buyerPda.toBase58());
    console.log("receiptPda", receiptPda.toBase58());
    console.log("statePda", statePda.toBase58());
  });*

  /*it('init mainnet', async () => {
    const CONTRACT_ID = "F6X97TzFoSyRAsMCVEjRncxAqNNmDqGoaq5ESFKvCzWZ";


    const programId = new anchor.web3.PublicKey(CONTRACT_ID);
    const program = new anchor.Program(idl as any, programId, provider);

    const statePda = await getPda(program, "state", []);

     const sig = await program.methods
      .initialize()
      .accounts({
        signer: admin.publicKey,
        state: statePda,
        systemProgram: PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
    console.log(sig);
    await provider.connection.confirmTransaction(sig);
  })*/

  it("init contract", async () => {
    const sig = await program.methods
      .initialize()
      .accounts({
        signer: admin.publicKey,
        state: statePda,
        systemProgram: PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
    console.log(sig);
    await provider.connection.confirmTransaction(sig);
  });
  it("validate accounts", async () => {
    const data = await program.account.state.fetch(statePda);

    assert.equal(data.listings, 0);
    assert.equal(data.sales, 0);
    assert.equal(data.sellers, 0);
    assert.equal(data.buyers, 0);
    assert.equal(data.balance, 0);
  });

  it("init seller", async () => {
    const accounts = {
      signer: sellerKP.publicKey,
      seller: sellerPda,
      state: statePda,
      systemProgram: PROGRAM_ID,
    };

    const sig = await program.methods
      .initSeller()
      .accounts(accounts)
      .signers([sellerKP])
      .rpc();
    await provider.connection.confirmTransaction(sig);
  });

  it("validate accounts", async () => {
    const seller = await program.account.seller.fetch(sellerPda);
    const state = await program.account.state.fetch(statePda);

    assert.equal(seller.seller.toBase58(), sellerKP.publicKey);
    assert.equal(seller.sales, 0);
    assert.equal(seller.listings, 0);
    assert.equal(seller.balance, 0);
    assert.equal(state.sellers, 1);
  });

  it("create listing", async () => {
    const accounts = {
      signer: sellerKP.publicKey,
      seller: sellerPda,
      listing: listingPda,
      state: statePda,
      systemProgram: PROGRAM_ID,
    };

    const sig = await program.methods
      .createListing("dog", sellerKP.publicKey, new anchor.BN(1000000))
      .accounts(accounts)
      .signers([sellerKP])
      .rpc();

    await provider.connection.confirmTransaction(sig);
  });
  it("validate accounts", async () => {
    const seller = await program.account.seller.fetch(sellerPda);
    const listing = await program.account.listing.fetch(listingPda);
    const state = await program.account.state.fetch(statePda);

    assert.equal(seller.listings, 1);
    assert.equal(state.listings, 1);
    assert.equal(listing.engine, "dog");
    assert.equal(listing.sales, 0);
    assert.equal(listing.seller.toBase58(), sellerKP.publicKey.toBase58());
    assert.equal(listing.token.toBase58(), sellerKP.publicKey.toBase58());
    assert.equal(listing.price, 1000000);
    assert.equal(listing.id, 1);
    assert.equal(listing.approved, false);
  });

  it("init buyer", async () => {
    const accounts = {
      signer: buyerKP.publicKey,
      buyer: buyerPda,
      state: statePda,
      systemProgram: PROGRAM_ID,
    };

    const sig = await program.methods
      .initBuyer()
      .accounts(accounts)
      .signers([buyerKP])
      .rpc();
    await provider.connection.confirmTransaction(sig);
  });
  it("validate accounts", async () => {
    const state = await program.account.state.fetch(statePda);
    const buyer = await program.account.buyer.fetch(buyerPda);

    assert.equal(state.buyers, 1);
    assert.equal(buyer.buyer.toBase58(), buyerKP.publicKey.toBase58());
  });
  it("purchase unapproved listing", async () => {
    let err;
    try {
      sellerBalance = await provider.connection.getBalance(sellerPda);

      const accounts = {
        signer: buyerKP.publicKey,
        sellerAccount: sellerKP.publicKey,
        seller: sellerPda,
        listing: listingPda,
        buyer: buyerPda,
        receipt: receiptPda,
        state: statePda,
        systemProgram: PROGRAM_ID,
      };

      const sig = await program.methods
        .purchaseListing(new anchor.BN(0))
        .accounts(accounts)
        .signers([buyerKP])
        .rpc();

      await provider.connection.confirmTransaction(sig);
    } catch (error) {
      err = error.message;
    }

    assert.equal(
      err,
      "AnchorError caused by account: listing. Error Code: ConstraintRaw. Error Number: 2003. Error Message: A raw constraint was violated."
    );
  });
  it("approve listing", async () => {
    const accounts = {
      admin: admin.publicKey,
      sellerAccount: sellerKP.publicKey,
      seller: sellerPda,
      listing: listingPda,
      systemProgram: PROGRAM_ID,
    };

    const sig = await program.methods
      .approveListing(new anchor.BN(0))
      .accounts(accounts)
      .signers([admin])
      .rpc();

    await provider.connection.confirmTransaction(sig);
  });
  it("purchase approved listing", async () => {
    sellerBalance = await provider.connection.getBalance(sellerPda);
    adminBalance = await provider.connection.getBalance(statePda);

    const accounts = {
      signer: buyerKP.publicKey,
      sellerAccount: sellerKP.publicKey,
      seller: sellerPda,
      listing: listingPda,
      buyer: buyerPda,
      receipt: receiptPda,
      state: statePda,
      systemProgram: PROGRAM_ID,
    };

    const sig = await program.methods
      .purchaseListing(new anchor.BN(0))
      .accounts(accounts)
      .signers([buyerKP])
      .rpc();

    await provider.connection.confirmTransaction(sig);
  });
  it("validate accounts", async () => {
    const seller = await program.account.seller.fetch(sellerPda);
    const listing = await program.account.listing.fetch(listingPda);
    const state = await program.account.state.fetch(statePda);
    const buyer = await program.account.buyer.fetch(buyerPda);
    const receipt = await program.account.receipt.fetch(receiptPda);

    const afterSellerBalance = await provider.connection.getBalance(sellerPda);
    const afterAdminBalance = await provider.connection.getBalance(statePda);

    assert.equal(
      afterAdminBalance - adminBalance,
      Number(listing.price) * 0.2
    );
    assert.equal(
      Number(listing.price) * 0.8,
      afterSellerBalance - sellerBalance,
      "seller invalid balance"
    );
    assert.equal(
      Number(listing.price) * 0.2,
      state.balance,
      "admin invalid balance"
    );

    assert.equal(seller.sales, 1, "seller sales not incre");
    assert.equal(
      Number(seller.balance),
      Number(listing.price) * 0.8,
      "seller balance not deducted by price"
    );
    assert.equal(buyer.purchases, 1, "buyer purchases not inc");
    assert.equal(state.sales, 1, "state sales not inc");
    assert.equal(listing.sales, 1, "listing sales not inc");
    assert.equal(listingPda.toBase58(), receipt.listing.toBase58());
  });
  it("view owned listings", async () => {
    const getListing = await getDynamicPda(
      program,
      "listing",
      sellerKP.publicKey,
      0
    );
    const listing = await program.account.listing.fetch(getListing);

    assert.equal(listing.engine, "dog");
  });
  it("view bought listings", async () => {
    const getReceipt = await getDynamicPda(
      program,
      "receipt",
      buyerKP.publicKey,
      0
    );
    const receipt = await program.account.receipt.fetch(getReceipt);
    const listing = await program.account.listing.fetch(receipt.listing);

    assert.equal(listing.engine, "dog");
  });*/
});
