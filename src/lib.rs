// Importa las librerías principales de Anchor
use anchor_lang::prelude::*;

// Declara el ID único del programa en la blockchain de Solana
declare_id!("GameStore111111111111111111111111111111111111");

// Define el módulo principal del programa
#[program]
pub mod tienda_videojuegos {
    use super::*;

    // Función para crear una nueva tienda
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
        // Obtiene la clave pública del propietario
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        // Inicializa un vector vacío de videojuegos
        let juegos: Vec<Videojuego> = Vec::new();

        // Guarda los datos iniciales en la cuenta tienda
        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            juegos,
        });

        Ok(())
    }

    // Función para agregar un nuevo videojuego
    pub fn agregar_juego(context: Context<NuevoJuego>, titulo: String, plataforma: String, precio: u16) -> Result<()> {
        // Verifica que el dueño sea quien ejecuta la acción
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Crea un nuevo videojuego con los datos recibidos
        let juego = Videojuego {
            titulo,
            plataforma,
            precio,
            disponible: true,
        };

        // Inserta el juego en la lista de la tienda
        context.accounts.tienda.juegos.push(juego);

        Ok(())
    }

    // Función para eliminar un videojuego por título
    pub fn eliminar_juego(context: Context<NuevoJuego>, titulo: String) -> Result<()> {
        // Verifica que el dueño sea quien ejecuta la acción
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene referencia mutable a la lista de juegos
        let juegos = &mut context.accounts.tienda.juegos;

        // Recorre la lista buscando el título
        for i in 0..juegos.len() {
            if juegos[i].titulo == titulo {
                juegos.remove(i);
                msg!("Juego {} eliminado!", titulo);
                return Ok(());
            }
        }

        // Si no encuentra el juego, devuelve error
        Err(Errores::JuegoNoExiste.into())
    }

    // Función para ver todos los videojuegos registrados
    pub fn ver_juegos(context: Context<NuevoJuego>) -> Result<()> {
        // Verifica que el dueño sea quien consulta
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Imprime la lista de juegos en el log
        msg!("Lista de juegos: {:#?}", context.accounts.tienda.juegos);
        Ok(())
    }

    // Función para alternar la disponibilidad de un videojuego
    pub fn alternar_disponibilidad(context: Context<NuevoJuego>, titulo: String) -> Result<()> {
        // Verifica que el dueño sea quien ejecuta
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene referencia mutable a la lista de juegos
        let juegos = &mut context.accounts.tienda.juegos;

        // Busca el juego por título y cambia su estado
        for i in 0..juegos.len() {
            if juegos[i].titulo == titulo {
                let nuevo_estado = !juegos[i].disponible;
                juegos[i].disponible = nuevo_estado;
                msg!("El juego {} ahora está disponible: {}", titulo, nuevo_estado);
                return Ok(());
            }
        }

        // Si no existe, devuelve error
        Err(Errores::JuegoNoExiste.into())
    }

    // Función para contar el total de videojuegos registrados
    pub fn total_juegos(context: Context<NuevoJuego>) -> Result<()> {
        let total = context.accounts.tienda.juegos.len();
        msg!("La tienda tiene {} juegos registrados", total);
        Ok(())
    }
}

// Definición de errores personalizados
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("El juego no existe")]
    JuegoNoExiste,
}

// Definición de la cuenta principal de la tienda
#[account]
#[derive(InitSpace)]
pub struct Tienda {
    owner: Pubkey, // Propietario de la tienda

    #[max_len(60)]
    nombre: String, // Nombre de la tienda

    #[max_len(50)]
    juegos: Vec<Videojuego>, // Lista de videojuegos
}

// Definición de la estructura de cada videojuego
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Videojuego {
    #[max_len(60)]
    titulo: String, // Título del videojuego

    #[max_len(30)]
    plataforma: String, // Plataforma (ejemplo: Xbox, PlayStation, PC)

    precio: u16, // Precio del videojuego

    disponible: bool, // Estado de disponibilidad
}

// Contexto de cuentas para crear una nueva tienda
#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // Propietario que firma la transacción

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda_videojuegos", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>, // Cuenta de la tienda

    pub system_program: Program<'info, System>, // Programa del sistema
}

// Contexto de cuentas para operaciones sobre videojuegos
#[derive(Accounts)]
pub struct NuevoJuego<'info> {
    pub owner: Signer<'info>, // Propietario que firma la transacción

    #[account(mut)]
    pub tienda: Account<'info, Tienda>, // Cuenta de la tienda existente
}
