use anchor_lang::prelude::*;

declare_id!("2DZc1dTm8QAUKjBtbwAGojBkQJT9TfvjEosKM6d9Cf8o");

#[program]
pub mod waffleportal {
    use super::*;
    pub fn start_things_off(ctx: Context<StartThingsOff>) -> ProgramResult {
        // Get account reference
        let base_account = &mut ctx.accounts.base_account;
        // Init total_gifs to 0
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get reference to the account
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        
        // Build the GIF item struct
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Increment total_gifs
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartThingsOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify the data we want in the AddGif context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Give Solana info about what should be stored in the account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
