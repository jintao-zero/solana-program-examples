import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CheckingAccounts } from "../target/types/checking_accounts";
import { assert } from "chai";
import {
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    Keypair,
} from "@solana/web3.js";

describe("checking-accounts", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    let wallet = anchor.getProvider().wallet;
    let accountToChange = anchor.web3.Keypair.generate();

    const accountToCreate = new Keypair();

    const program = anchor.workspace
        .checkingAccounts as Program<CheckingAccounts>;

    it("Create an account owned by the program", async () => {
        const instruction = SystemProgram.createAccount({
            fromPubkey: provider.wallet.publicKey,
            newAccountPubkey: accountToChange.publicKey,
            lamports:
                await provider.connection.getMinimumBalanceForRentExemption(0),
            space: 0,
            programId: program.programId, // Our program
        });

        const transaction = new Transaction().add(instruction);

        await sendAndConfirmTransaction(provider.connection, transaction, [
            wallet.payer,
            accountToChange,
        ]);
    });
    it("Check accounts", async () => {
        const tx = await program.methods
            .checkAccounts()
            .accounts({
                payer: wallet.publicKey,
                accountToCreate: accountToCreate.publicKey,
                accountToChange: accountToChange.publicKey,
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });
});
