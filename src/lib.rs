use anchor_lang::prelude::*;

declare_id!("6gj1isTLAMAXNh18C2XEwbyVFxbhEorz8Hu9qt4Wp2hn");

#[program]
pub mod venta {
    use super::*;

    //CREAR VENTA
    pub fn crear_venta(
        context: Context<NuevaVenta>,
        nombre_vendedor: String,
    ) -> Result<()> {
        let owner_id = context.accounts.owner.key();

        let producto: Vec<Producto> = Vec::new();

        context.accounts.venta.set_inner(Venta {
            owner: owner_id,
            nombre_vendedor,
            producto,
        });

        msg!(
            "Venta creada exitosamente. Owner ID: {}",
            owner_id
        );

        Ok(())
    }
    // AGREGAR PRODUCTO /

    pub fn agregar_producto(
        context: Context<NuevoProducto>,
        nombre_producto: String,
        cliente: String,
        cantidad: u16,
        precio_unitario: f64,
        total: f64,
        fecha_venta: String,
    ) -> Result<()> {
        require!(
            context.accounts.venta.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let producto = Producto {
            nombre_producto: nombre_producto.clone(),
            cliente,
            cantidad,
            precio_unitario,
            total,
            estado_venta: true,
            fecha_venta,
        };

        context.accounts.venta.producto.push(producto);

        msg!(
            "Producto {} agregado correctamente",
            nombre_producto
        );

        Ok(())
    }

    //VER PRODUCTO 

    pub fn ver_producto(
        context: Context<NuevoProducto>
    ) -> Result<()> {
        require!(
            context.accounts.venta.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista actual de productos: {:#?}",
            context.accounts.venta.producto
        );

        Ok(())
    }

    // ELIMINAR PRODUCTO 

    pub fn eliminar_producto(
        context: Context<NuevoProducto>,
        nombre_producto: String,
    ) -> Result<()> {
        require!(
            context.accounts.venta.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.venta.producto;

        let pos = productos
            .iter()
            .position(|x| x.nombre_producto == nombre_producto)
            .ok_or(Errores::ProductoNoExiste)?;

        productos.remove(pos);

        msg!(
            "Producto {} eliminado correctamente",
            nombre_producto
        );

        Ok(())
    }
    //ALTERAR ESTADO 

    pub fn alterar_estado(
        context: Context<NuevoProducto>,
        nombre_producto: String,
    ) -> Result<()> {
        require!(
            context.accounts.venta.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.venta.producto;

        for producto in productos.iter_mut() {
            if producto.nombre_producto == nombre_producto {
                producto.estado_venta = !producto.estado_venta;

                msg!(
                    "El producto {} ahora tiene estado: {}",
                    nombre_producto,
                    producto.estado_venta
                );

                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }
}

// ERRORES 

#[error_code]
pub enum Errores {
    #[msg("Error: No eres el propietario de la venta que deseas modificar")]
    NoEresElOwner,

    #[msg("Error: El producto proporcionado no existe")]
    ProductoNoExiste,
}

//CUENTA VENTA 

#[account]
#[derive(InitSpace)]
pub struct Venta {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre_vendedor: String,

    #[max_len(10)]
    pub producto: Vec<Producto>,
}

//STRUCT PRODUCTO 

#[derive(
    InitSpace,
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    PartialEq,
    Debug
)]
pub struct Producto {
    #[max_len(60)]
    pub nombre_producto: String,

    #[max_len(60)]
    pub cliente: String,

    pub cantidad: u16,
    pub precio_unitario: f64,
    pub total: f64,
    pub estado_venta: bool,

    #[max_len(30)]
    pub fecha_venta: String,
}

//CONTEXTO NUEVA VENTA 

#[derive(Accounts)]
pub struct NuevaVenta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Venta::INIT_SPACE,
        seeds = [b"venta", owner.key().as_ref()],
        bump
    )]
    pub venta: Account<'info, Venta>,

    pub system_program: Program<'info, System>,
}

//CONTEXTO NUEVO PRODUCTO

#[derive(Accounts)]
pub struct NuevoProducto<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub venta: Account<'info, Venta>,
}
