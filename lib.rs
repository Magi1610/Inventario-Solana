use anchor_lang::prelude::*;

declare_id!("5kSDb8ngK7Y3Jx4hbDmyBpeF4hMKi28ksU9vJqp9Ykq");

#[program]
pub mod personaje_rpg {
    use super::*;

    pub fn crear_juego(ctx: Context<NuevoJuego>, nombre_juego: String) -> Result<()> {
        require!(!nombre_juego.trim().is_empty(), Errores::CampoVacio);

        ctx.accounts.juego.set_inner(Juego {
            owner: ctx.accounts.owner.key(),
            nombre_juego,
            personajes: Vec::new(),
        });

        Ok(())
    }

    pub fn agregar_personaje(
        ctx: Context<ModificarJuego>,
        nombre: String,
        clase: Clase,
    ) -> Result<()> {
        require!(!nombre.trim().is_empty(), Errores::CampoVacio);

        let juego = &mut ctx.accounts.juego;

        require!(
            juego.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );
        require!(juego.personajes.len() < 5, Errores::EspacioInsuficiente);

        let (vida, mana, fuerza, defensa, velocidad, inteligencia) = match clase {
            Clase::Guerrero => (150, 30, 20, 15, 10, 5),
            Clase::Mago => (80, 120, 5, 5, 10, 25),
            Clase::Arquero => (100, 50, 15, 10, 20, 10),
            Clase::Asesino => (90, 40, 25, 8, 25, 8),
        };

        juego.personajes.push(Personaje {
            nombre,
            nivel: 1,
            experiencia: 0,
            clase,
            vida,
            mana,
            fuerza,
            defensa,
            velocidad,
            inteligencia,
        });

        Ok(())
    }

    pub fn ver_personajes(ctx: Context<ModificarJuego>) -> Result<()> {
        let juego = &ctx.accounts.juego;

        require!(
            juego.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Personajes: {:#?}", juego.personajes);

        Ok(())
    }

    pub fn ganar_experiencia(ctx: Context<ModificarJuego>, nombre: String, xp: u64) -> Result<()> {
        let juego = &mut ctx.accounts.juego;

        require!(
            juego.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        if let Some(p) = juego.personajes.iter_mut().find(|p| p.nombre == nombre) {
            p.experiencia += xp;

            while p.experiencia >= 100 {
                p.experiencia -= 100;
                p.nivel += 1;
                p.fuerza += 5;
                p.defensa += 3;
                p.vida += 20;
            }

            Ok(())
        } else {
            Err(Errores::PersonajeNoEncontrado.into())
        }
    }

    pub fn eliminar_personaje(ctx: Context<ModificarJuego>, nombre: String) -> Result<()> {
        let juego = &mut ctx.accounts.juego;

        require!(
            juego.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        if let Some(pos) = juego.personajes.iter().position(|p| p.nombre == nombre) {
            juego.personajes.remove(pos);
            Ok(())
        } else {
            Err(Errores::PersonajeNoEncontrado.into())
        }
    }
}

#[error_code]
pub enum Errores {
    #[msg("Campo vacio")]
    CampoVacio,
    #[msg("No eres el owner")]
    NoEresElOwner,
    #[msg("Personaje no encontrado")]
    PersonajeNoEncontrado,
    #[msg("Limite de personajes alcanzado")]
    EspacioInsuficiente,
}

#[account]
#[derive(InitSpace)]
pub struct Juego {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre_juego: String,

    #[max_len(10)]
    pub personajes: Vec<Personaje>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Personaje {
    #[max_len(60)]
    pub nombre: String,
    pub nivel: u16,
    pub experiencia: u64,
    pub clase: Clase,
    pub vida: u16,
    pub mana: u16,
    pub fuerza: u16,
    pub defensa: u16,
    pub velocidad: u16,
    pub inteligencia: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub enum Clase {
    Guerrero,
    Mago,
    Arquero,
    Asesino,
}

#[derive(Accounts)]
pub struct NuevoJuego<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Juego::INIT_SPACE,
        seeds = [b"juego", owner.key().as_ref()],
        bump
    )]
    pub juego: Account<'info, Juego>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarJuego<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub juego: Account<'info, Juego>,
}
