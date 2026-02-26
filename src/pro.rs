use anchor_lang::prelude::*;

declare_id!(H5kSDb8ngK7Y3Jx4hbDmyBpeF4hMKi28ksU9vJqp9YkqHvvnGXYbU37t3H27crgnUj5usUjcMDa6zkbeTru3C6AcHvvnGXYbU37t3H27crgnUj5usUjcMDa6zkbeTru3C6AcHvvnGXYbU37t3H27crgnUj5usUjcMDa6zkbeTru3C6AcHvvnGXYbU37t3H27crgnUj5usUjcMDa6zkbeTru3C6AcH27crgnUj5usUjcMDa6zkbeTru3C6Ac);

#[program]

pub mod Inventario{
    use super::*;

    pub fn crear_Inventario() -> Result<()>{
        //Codigo....
    }
}


#[account]
#[derive(InitSpace)]
pub struct Inventario{
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    productos: Vec<Producto>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Producto{
    #[max_leg(60)]
    nombre: String,

    precio: u128,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevoInventario{
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Inventario::INIT_SPACE + 8,
        seeds = [b"inventario", owner.key().as_ref()]
        bump
    )]
    pub inventario: Account<'info, Inventario>,

    pub system_program: Program<'info, System>,
}

pub struct NuevoProducto{
    pub: owner: Signer<'info>,

    #[account(mut)]
    pub: inventario: Account<'info, Inventario>,
}
