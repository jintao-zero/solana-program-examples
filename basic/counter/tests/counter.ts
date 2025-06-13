import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { assert } from "chai";

describe("counter", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.counter as Program<Counter>;
    let counterAccount: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    let user: anchor.web3.Keypair = anchor.getProvider().wallet.payer;
    let connection = anchor.getProvider().connection;
    console.log("Program ID:", program.programId.toString());
    console.log("User Public Key:", user.publicKey.toString());
    console.log(
        "Counter Account Public Key:",
        counterAccount.publicKey.toString()
    );
    it("Is initialized!", async () => {
        // Add your test here.
        const tx = await program.methods
            .initializeCounter()
            .accounts({
                user: user.publicKey,
                counter: counterAccount.publicKey,
            })
            .signers([counterAccount])
            .rpc({ commitment: "confirmed" });
        console.log("Your transaction signature", tx);
        connection.confirmTransaction(tx, "confirmed");
        connection
            .getTransaction(tx, {
                commitment: "confirmed",
            })
            .then((transaction) => {
                if (transaction) {
                    console.log("Transaction confirmed:", transaction);
                } else {
                    console.log("Transaction not found");
                }
            });

        const account = await program.account.counterAccount.fetch(
            counterAccount.publicKey
        );
        console.log(
            "Counter initialized with count:",
            account.count.toString()
        );
    });
    it("Increment Counter", async () => {
        await program.methods
            .incrementCounter()
            .accounts({ counter: counterAccount.publicKey })
            .rpc();

        const currentCount = await program.account.counterAccount.fetch(
            counterAccount.publicKey
        );

        //assert(currentCount.count.toNumber() === 1, "Expected  count to be 1");
    });
});
