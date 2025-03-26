import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./turbin3-wallet.json";

const ENDPOINT =
  "https://polished-warmhearted-frog.solana-devnet.quiknode.pro/9bc0c3437243817577c59c3690d3bcde03fe8b6f/";
//const ENDPOINT = "https://api.devnet.solana.com";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
// Create a devnet connection
const connection = new Connection(ENDPOINT);
// Github account
const github = Buffer.from("aquental", "utf8");
// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});
// Create our program
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);

// Execute our enrollment transaction
(async () => {
  try {
    const txhash = await program.methods
      .submit(github)
      .accounts({
        signer: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();
    console.log(`Success! Check out your TX here:
  https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();

//TS=prereq done
//https://explorer.solana.com/tx/3Ji3KRS8yiWnWY46if7mwTB27hfudk6mnBE3kRFNuM3U7qnAyouyDY7H6zWLsd1ZufDt7UUVeRc1J8a1yNCeMU8H?cluster=devnet
