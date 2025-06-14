import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CloseAccount } from "../target/types/close_account";
import { assert } from "chai";

describe("close-account", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.closeAccount as Program<CloseAccount>;
    const payer = provider.wallet as anchor.Wallet;

    const [userAccountAddress] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("USER"), payer.publicKey.toBuffer()],
        program.programId
    );

    it("Create Account", async () => {
        // Add your test here.
        const tx = await program.methods
            .createUser("John Doe")
            .accounts({
                user: payer.publicKey,
            })
            .rpc({
                commitment: "confirmed",
            });
        const userAccount = await program.account.userState.fetch(
            userAccountAddress
        );
        assert.equal(userAccount.name, "John Doe");
        assert.equal(userAccount.user.toBase58(), payer.publicKey.toBase58());
    });
    it("Close Account", async () => {
        // Add your test here.
        const tx = await program.methods
            .closeUser()
            .accounts({
                user: payer.publicKey,
            })
            .rpc({
                commitment: "confirmed",
            });
        const userAccount = await program.account.userState
            .fetch(userAccountAddress)
            .catch(() => null);
        assert.equal(userAccount, null);
    });
});
