// Importar dependencias
const web3 = require("@solana/web3.js");

// Función principal
async function main(pg) {
    try {
        // Mostrar dirección del wallet
        const myAddress = pg.wallet.publicKey.toString();
        console.log("📌 Mi dirección:", myAddress);

        // Obtener balance
        const balanceLamports = await pg.connection.getBalance(pg.wallet.publicKey);
        const balanceSol = balanceLamports / web3.LAMPORTS_PER_SOL;

        console.log(`💰 Mi balance: ${balanceSol} SOL`);

    } catch (error) {
        console.error("❌ Error al consultar la información:", error);
    }
}

// Exportar la función para usarla en otros módulos
module.exports = { main };
