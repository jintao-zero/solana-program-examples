import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountData } from "../target/types/account_data";
import { Keypair } from "@solana/web3.js";

describe("account-data", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.accountData as Program<AccountData>;

  // Generate a new keypair for the addressInfo account
  const addressInfoAccount = new Keypair();

  it("Create the address info account", async () => {
    console.log(`Payer Address : ${payer.publicKey.toBase58()}`);
    console.log(`Address Info Account : ${addressInfoAccount.publicKey.toBase58()}`);

    // Instruction Ix data
    const addressInfo = {
      name: 'Joe C',
      houseNumber: 123,
      street: 'Main St',
      city: 'Anytown',
    }

    await program.methods.createAddressInfo(addressInfo.name, addressInfo.houseNumber, addressInfo.street, addressInfo.city)
    .accounts({
      addressInfo: addressInfoAccount.publicKey,
      signer: payer.publicKey,
  })
  .signers([addressInfoAccount])
  .rpc();
  });

  it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });
});
