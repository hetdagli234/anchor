use anchor_lang::prelude::*;

declare_id!("CustomDiscriminator111111111111111111111111");

const CONST_DISC: &'static [u8] = &[55, 66, 77, 88];

const fn get_disc(input: &str) -> &'static [u8] {
    match input.as_bytes() {
        b"wow" => &[4 + 5, 55 / 5],
        _ => unimplemented!(),
    }
}

#[program]
pub mod custom_discriminator {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn int(_ctx: Context<DefaultIx>) -> Result<()> {
        Ok(())
    }

    #[instruction(discriminator = [1, 2, 3, 4])]
    pub fn array(_ctx: Context<DefaultIx>) -> Result<()> {
        Ok(())
    }

    #[instruction(discriminator = b"hi")]
    pub fn byte_str(_ctx: Context<DefaultIx>) -> Result<()> {
        Ok(())
    }

    #[instruction(discriminator = CONST_DISC)]
    pub fn constant(_ctx: Context<DefaultIx>) -> Result<()> {
        Ok(())
    }

    #[instruction(discriminator = get_disc("wow"))]
    pub fn const_fn(_ctx: Context<DefaultIx>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DefaultIx<'info> {
    pub signer: Signer<'info>,
}
