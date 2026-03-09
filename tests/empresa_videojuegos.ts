// No imports needed: web3, anchor, pg and more are globally available

describe("Empresa de Videojuegos", () => {

  it("crear empresa", async () => {

    const empresaPda = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("empresa"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    )[0];

    const nombre = "YaelGame Studios";

    const txHash = await pg.program.methods
      .crearEmpresa(nombre)
      .accounts({
        owner: pg.wallet.publicKey,
        empresa: empresaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Usa 'solana confirm -v ${txHash}' para ver los logs`);

    await pg.connection.confirmTransaction(txHash);

    const empresa = await pg.program.account.empresa.fetch(empresaPda);

    console.log("Datos on-chain de la empresa:", empresa);

  });

});
