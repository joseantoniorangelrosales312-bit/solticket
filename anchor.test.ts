import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solticket } from "../target/types/solticket";

describe("solticket", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solticket as Program<Solticket>;

  it("Crea un evento", async () => {
    const event = anchor.web3.Keypair.generate();

    await program.methods
      .createEvent("Solana Hackathon")
      .accounts({
        event: event.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([event])
      .rpc();

    console.log("Evento creado:", event.publicKey.toString());
  });
});
