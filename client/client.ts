import { PublicKey } from "@solana/web3.js";

//CONSTANTES 

const nombre_vendedor = "Juan";
const owner = pg.wallet.publicKey;

console.log("Mi address:", owner.toString());

const balance = await pg.connection.getBalance(owner);
console.log(`Mi balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

// PDA VENTA /

function pdaVenta() {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("venta"),
      owner.toBuffer(),
    ],
    pg.PROGRAM_ID
  );
}

///CREAR VENTA 

async function crearVenta(nombre_vendedor) {
  const [pda_venta] = pdaVenta();

  const txHash = await pg.program.methods
    .crearVenta(nombre_vendedor)
    .accounts({
      owner: owner,
      venta: pda_venta,
    })
    .rpc();

  console.log("txHash:", txHash);
}

///AGREGAR PRODUCTO 

async function agregarProducto(
  nombre_producto,
  cliente,
  cantidad,
  precio_unitario,
  total,
  fecha_venta
) {
  const [pda_venta] = pdaVenta();

  const txHash = await pg.program.methods
    .agregarProducto(
      nombre_producto,
      cliente,
      cantidad,
      precio_unitario,
      total,
      fecha_venta
    )
    .accounts({
      owner: owner,
      venta: pda_venta,
    })
    .rpc();

  console.log("txHash:", txHash);
}

////////////////// VER PRODUCTOS ////////////////////

async function verProductos() {
  const [pda_venta] = pdaVenta();

  try {
    const ventaAccount = await pg.program.account.venta.fetch(
      pda_venta
    );

    const numero_productos = ventaAccount.producto.length;

    if (!ventaAccount.producto || numero_productos === 0) {
      console.log("No hay productos registrados");
      return;
    }

    console.log("Cantidad de productos:", numero_productos);

    for (let i = 0; i < numero_productos; i++) {
      const producto = ventaAccount.producto[i];

      console.log(
        `Producto #${i + 1}
        Nombre: ${producto.nombreProducto}
        Cliente: ${producto.cliente}
        Cantidad: ${producto.cantidad}
        Precio Unitario: ${producto.precioUnitario}
        Total: ${producto.total}
        Estado: ${producto.estadoVenta}
        Fecha: ${producto.fechaVenta}`
      );
    }
  } catch (error) {
    console.log("Error:", error);
  }
}

// ELIMINAR PRODUCTO 

async function eliminarProducto(nombre_producto) {
  const [pda_venta] = pdaVenta();

  const txHash = await pg.program.methods
    .eliminarProducto(nombre_producto)
    .accounts({
      owner: owner,
      venta: pda_venta,
    })
    .rpc();

  console.log("txHash:", txHash);
}

// ALTERAR ESTADO 

async function alterarEstado(nombre_producto) {
  const [pda_venta] = pdaVenta();

  const txHash = await pg.program.methods
    .alterarEstado(nombre_producto)
    .accounts({
      owner: owner,
      venta: pda_venta,
    })
    .rpc();

  console.log("txHash:", txHash);
}
