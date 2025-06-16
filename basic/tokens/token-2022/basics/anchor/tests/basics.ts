import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, sendAndConfirmTransaction } from "@solana/web3.js";
import { Basics } from "../target/types/basics";
import {
    TOKEN_2022_PROGRAM_ADDRESS,
    ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token-2022";

describe("basics", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.basics as Program<Basics>;
    const connection = provider.connection;
    const wallet = provider.wallet;
    const token2022PublicKey = new PublicKey(TOKEN_2022_PROGRAM_ADDRESS);
    console.log("Wallet Public Key:", wallet.publicKey.toBase58());
    console.log("Wallet Payer Public Key:", wallet.payer.publicKey.toBase58());
    console.log("Connection RPC Endpoint:", connection.rpcEndpoint);
    console.log("Connection Commitment:", connection.commitment);
    const ataPublicKey = new PublicKey(ASSOCIATED_TOKEN_PROGRAM_ADDRESS);

    const tokenName = "TestToken";
    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("token-2022-token"),
            wallet.publicKey.toBytes(),
            Buffer.from(tokenName),
        ],
        program.programId
    );
    const [payerATA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            wallet.publicKey.toBytes(),
            token2022PublicKey.toBytes(),
            mint.toBytes(),
        ],
        token2022PublicKey
    );
    const receiver = anchor.web3.Keypair.generate();
    const [receiverATA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            receiver.publicKey.toBytes(),
            token2022PublicKey.toBytes(),
            mint.toBytes(),
        ],
        ataPublicKey
    );
    it("Create Token-2022 token!", async () => {
        //await connection.requestAirdrop(receiver.publicKey, 1000000000);
        //await connection.requestAirdrop(wallet.publicKey, 1000000000);
        const tx = new anchor.web3.Transaction();
        const ix = await program.methods
            .createToken(tokenName)
            .accounts({
                signer: wallet.publicKey,
                tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
            })
            .instruction();
        tx.add(ix);
        const sig = await sendAndConfirmTransaction(
            program.provider.connection,
            tx,
            [wallet.payer]
        );
        console.log("Your transaction signature", sig);
    });
});
