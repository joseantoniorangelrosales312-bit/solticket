import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("solticket", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solticket as Program;

  it("Create Event", async () => {
    const event = anchor.web3.Keypair.generate();

    await program.methods
      .createEvent("Solana Hackathon")
      .accounts({
        event: event.publicKey,
        authority: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([event])
      .rpc();

    console.log("Evento creado:", event.publicKey.toString());
  });
});
