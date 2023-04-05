use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::native_token::sol_to_lamports;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); //update this


#[account]
pub struct Pot {  // The JackPot PDA
    pub name: String,
    pub owner: Pubkey,
    pub balance: u64,
    pub winning_number: i64  // Clock returns i64
}

// #[account]
// pub struct Guess {  // The user's guess
//     pub number: i64 // Converted from u8 to match `Pot.winning_number`
// }

#[derive(Accounts)]
pub struct Create<'info> {
    // Create JackPot PDA
    #[account(init, payer=user, space=5000, seeds=[b"jackpot", user.key().as_ref()], bump)]
    pub pot: Account<'info, Pot>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Session<'info> {
    #[account(mut)]
    pub pot: Account<'info, Pot>,
    // pub guess: Account<'info, Guess>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[program]
pub mod gg {
    use super::*;

    // init
    pub fn create(ctx: Context<Create>, name: String) -> ProgramResult {
        let pot = &mut ctx.accounts.pot;
        let clock = Clock::get()?;  // random generator [workaround]
       
        pot.name = name;
        pot.owner = *ctx.accounts.user.key;
        pot.balance = 0;
        pot.winning_number = clock.unix_timestamp % 100;  // Set range as desired
        Ok(())
    }

    // Make a guess
    pub fn deposit(ctx: Context<Session>) -> ProgramResult {
        // Handle entry deposit
        let entry_fee = sol_to_lamports(0.1);  // Set as desired
        let txn = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.pot.key(),
            entry_fee
        );

        anchor_lang::solana_program::program::invoke(
            &txn,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.pot.to_account_info()
            ]
        )?;
        (&mut ctx.accounts.pot).balance += entry_fee;  // Update balance
        // (&mut ctx.accounts.guess).number = i64::from(number);  // Save user's guess
        Ok(())
    }

    // Win or lose!
    pub fn guess(ctx: Context<Session>, number: u8) -> ProgramResult {
        let pot = &mut ctx.accounts.pot;
        let user = &mut ctx.accounts.user;
        let guess = i64::from(number);

        // Check owner
        if pot.owner != user.key() {
            return Err(ProgramError::IncorrectProgramId);
        }

        if guess == pot.winning_number {
            // Perform the withdraw
            **pot.to_account_info().try_borrow_mut_lamports()? -= pot.balance;
            **user.to_account_info().try_borrow_mut_lamports()? += pot.balance;
        }
        Ok(())
    }
}
