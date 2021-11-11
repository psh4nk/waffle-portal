const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("Starting the test...")

  // Provider must be set and updated to communicate with frontend
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Waffleportal;

  // Create account keypair
  const baseAccount = anchor.web3.Keypair.generate();

  let tx = await program.rpc.startThingsOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("Your transaction signature: ", tx);

  // Fetch data from the account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF Count: ', account.totalGifs.toString())

  // add_gif call
  await program.rpc.addGif("https://media.giphy.com/media/z9lZMI5UDdI08/giphy.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF count: ', account.totalGifs.toString())

  // Access account's gif_list
  console.log('GIF List: ', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();