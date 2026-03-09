use anchor_lang::prelude::*;

declare_id!("9xQeWvG816bUx9EPjHmaT23yvVM6Xv2yN5GzJbK8u9p");

#[program]
pub mod empresa_videojuegos {
    use super::*;

    pub fn crear_empresa(context: Context<NuevaEmpresa>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let videojuegos: Vec<Videojuego> = Vec::new();

        context.accounts.empresa.set_inner(Empresa {
            owner: owner_id,
            nombre,
            videojuegos,
        });

        Ok(())
    }

    pub fn agregar_videojuego(
        context: Context<NuevoVideojuego>,
        nombre: String,
        precio: u16,
        plataforma: String
    ) -> Result<()> {

        require!(
            context.accounts.empresa.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let videojuego = Videojuego {
            nombre,
            precio,
            plataforma,
            disponible: true,
        };

        context.accounts.empresa.videojuegos.push(videojuego);

        Ok(())
    }

    pub fn eliminar_videojuego(context: Context<NuevoVideojuego>, nombre: String) -> Result<()> {

        require!(
            context.accounts.empresa.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.empresa.videojuegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                juegos.remove(i);
                msg!("Videojuego {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::JuegoNoExiste.into())
    }

    pub fn ver_videojuegos(context: Context<NuevoVideojuego>) -> Result<()> {

        require!(
            context.accounts.empresa.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista actual de videojuegos: {:#?}",
            context.accounts.empresa.videojuegos
        );

        Ok(())
    }

    pub fn alternar_disponibilidad(context: Context<NuevoVideojuego>, nombre: String) -> Result<()> {

        require!(
            context.accounts.empresa.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.empresa.videojuegos;

        for i in 0..juegos.len() {

            let estado = juegos[i].disponible;

            if juegos[i].nombre == nombre {

                let nuevo_estado = !estado;

                juegos[i].disponible = nuevo_estado;

                msg!(
                    "El videojuego {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::JuegoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {

    #[msg("No eres el propietario de la empresa")]
    NoEresElOwner,

    #[msg("El videojuego no existe")]
    JuegoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Empresa {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(20)]
    videojuegos: Vec<Videojuego>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Videojuego {

    #[max_len(60)]
    nombre: String,

    precio: u16,

    #[max_len(30)]
    plataforma: String,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaEmpresa<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Empresa::INIT_SPACE + 8,
        seeds = [b"empresa", owner.key().as_ref()],
        bump
    )]
    pub empresa: Account<'info, Empresa>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoVideojuego<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub empresa: Account<'info, Empresa>,
  }
