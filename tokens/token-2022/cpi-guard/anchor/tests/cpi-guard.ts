import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    ExtensionType,
    TOKEN_2022_PROGRAM_ID,
    createEnableCpiGuardInstruction,
    createInitializeAccountInstruction,
    createMint,
    disableCpiGuard,
    getAccountLen,
    mintTo,
} from "@solana/spl-token";
import {
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
} from "@solana/web3.js";
import { CpiGuard } from "../target/types/cpi_guard";

describe("cpi-guard", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;
    const wallet = provider.wallet as anchor.Wallet;

    const program = anchor.workspace.cpiGuard as Program<CpiGuard>;
    const mintKeypair = new anchor.web3.Keypair();
    const tokenKeypair = new anchor.web3.Keypair();

    it("Create Token Account with CpiGuard extension", async () => {
        await createMint(
            connection,
            wallet.payer,
            wallet.publicKey,
            null,
            2,
            mintKeypair,
            undefined,
            TOKEN_2022_PROGRAM_ID
        );
        // Size of Token Account with extension
        const accountLen = getAccountLen([ExtensionType.CpiGuard]);
        const lamports = await connection.getMinimumBalanceForRentExemption(
            accountLen
        );
        const createAccountInstruction = SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: tokenKeypair.publicKey,
            space: accountLen,
            lamports,
            programId: TOKEN_2022_PROGRAM_ID,
        });
        // Instruction to initialize Token Account data
        const initAccountInstruction = createInitializeAccountInstruction(
            tokenKeypair.publicKey, // Token Account Address
            mintKeypair.publicKey, // Mint Account
            wallet.publicKey, // Token Account Owner
            TOKEN_2022_PROGRAM_ID // Token Program ID
        );
        // Instruction to initialize the CpiGuard extension
        const enableCpiGuardInstruction = createEnableCpiGuardInstruction(
            tokenKeypair.publicKey, // Token Account Address
            wallet.publicKey, //
            [],
            TOKEN_2022_PROGRAM_ID
        ); // Token Program ID
        const transaction = new Transaction().add(
            createAccountInstruction,
            initAccountInstruction,
            enableCpiGuardInstruction
        );

        const transactionSignature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [wallet.payer, tokenKeypair]
        );
        await mintTo(
            connection,
            wallet.payer,
            mintKeypair.publicKey, // Mint Account
            tokenKeypair.publicKey, // Token Account Address
            wallet.publicKey, // Token Account Owner
            1, // Amount to mint
            [],
            null,
            TOKEN_2022_PROGRAM_ID // Token Program ID
        );
        console.log("Your transaction signature", transactionSignature);
    });

    it("Transfer expect fail", async () => {
        // transfer
        try {
            const sig = await program.methods
                .cpiTransfer()
                .accounts({
                    senderTokenAccount: tokenKeypair.publicKey,
                    mint: mintKeypair.publicKey,
                })
                .signers([wallet.payer])
                .rpc();
        } catch (error) {
            console.error(
                "Transfer failed as expected due to CpiGuard enabled"
            );
        }
    });
    it("Disable CpiGuard", async () => {
        const disableCpiGuardInstruction = await disableCpiGuard(
            connection,
            wallet.payer,
            tokenKeypair.publicKey, // Token Account Address
            wallet.publicKey, // Token Account Owner
            [],
            { commitment: "confirmed" }, // Confirm options
            TOKEN_2022_PROGRAM_ID // Token Program ID
        );
        console.log("Your transaction signature", disableCpiGuardInstruction);
    });
    it("Transfer after disabling CpiGuard", async () => {
        const sig = await program.methods
            .cpiTransfer()
            .accounts({
                senderTokenAccount: tokenKeypair.publicKey,
                mint: mintKeypair.publicKey,
            })
            .signers([wallet.payer])
            .rpc();
        console.log("Transfer successful, transaction signature:", sig);
    });
});
