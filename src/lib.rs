use anchor_lang::prelude::*;

declare_id!("GameStore111111111111111111111111111111111111");

#[program]
pub mod tienda_videojuegos {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let juegos: Vec<Videojuego> = Vec::new();

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            juegos,
        });

        Ok(())
    }

    pub fn agregar_juego(context: Context<NuevoJuego>, titulo: String, plataforma: String, precio: u16) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juego = Videojuego {
            titulo,
            plataforma,
            precio,
            disponible: true,
        };

        context.accounts.tienda.juegos.push(juego);

        Ok(())
    }

    pub fn eliminar_juego(context: Context<NuevoJuego>, titulo: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.tienda.juegos;

        for i in 0..juegos.len() {
            if juegos[i].titulo == titulo {
                juegos.remove(i);
                msg!("Juego {} eliminado!", titulo);
                return Ok(());
            }
        }

        Err(Errores::JuegoNoExiste.into())
    }

    pub fn ver_juegos(context: Context<NuevoJuego>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de juegos: {:#?}", context.accounts.tienda.juegos);
        Ok(())
    }

    pub fn alternar_disponibilidad(context: Context<NuevoJuego>, titulo: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.tienda.juegos;

        for i in 0..juegos.len() {
            if juegos[i].titulo == titulo {
                let nuevo_estado = !juegos[i].disponible;
                juegos[i].disponible = nuevo_estado;
                msg!("El juego {} ahora está disponible: {}", titulo, nuevo_estado);
                return Ok(());
            }
        }

        Err(Errores::JuegoNoExiste.into())
    }

    pub fn total_juegos(context: Context<NuevoJuego>) -> Result<()> {
        let total = context.accounts.tienda.juegos.len();
        msg!("La tienda tiene {} juegos registrados", total);
        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("El juego no existe")]
    JuegoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(50)]
    juegos: Vec<Videojuego>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Videojuego {
    #[max_len(60)]
    titulo: String,

    #[max_len(30)]
    plataforma: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda_videojuegos", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoJuego<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
