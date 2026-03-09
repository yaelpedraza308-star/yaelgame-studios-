// Client

console.log(
  "Dirección del administrador de la empresa:",
  pg.wallet.publicKey.toString()
);

const balance = await pg.connection.getBalance(pg.wallet.publicKey);

console.log(
  `Saldo disponible para registrar videojuegos en la empresa: ${
    balance / web3.LAMPORTS_PER_SOL
  } SOL`
);
