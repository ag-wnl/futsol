use anchor_lang::prelude::*;

declare_id!("6o4S2WgWWQrbmLvU7sCWcqqtUh9tD7YKNC69xNYgobHu");

#[program]
pub mod betting_app {
    use super::*;

    pub fn initialize_event(ctx: Context<InitializeEvent>, event_id: u64, description: String) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.event_id = event_id;
        event.description = description;
        event.state = EventState::Open;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64, selection: u8) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        let event = &ctx.accounts.event;

        require!(event.state == EventState::Open, CustomError::EventNotOpen);

        bet.user = ctx.accounts.user.key();
        bet.amount = amount;
        bet.selection = selection;
        bet.event_id = event.event_id;

        // Transfer funds to escrow
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.escrow.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn resolve_event(ctx: Context<ResolveEvent>, outcome: u8) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.outcome = Some(outcome);
        event.state = EventState::Resolved;
        Ok(())
    }

    pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        let event = &ctx.accounts.event;

        require!(event.state == EventState::Resolved, CustomError::EventNotResolved);
        require!(event.outcome == Some(bet.selection), CustomError::IncorrectPrediction);

        // Calculate payout
        let payout = bet.amount * 2; // Example multiplier

        **ctx.accounts.escrow.try_borrow_mut_lamports()? -= payout;
        **ctx.accounts.user.try_borrow_mut_lamports()? += payout;

        Ok(())
    }
}

// Event account structure
#[account]
pub struct Event {
    pub event_id: u64,
    pub description: String,
    pub state: EventState,
    pub outcome: Option<u8>, // 1 for Team A win, 2 for Team B win, etc.
}

// Bet account structure
#[account]
pub struct Bet {
    pub user: Pubkey,
    pub event_id: u64,
    pub amount: u64,
    pub selection: u8,
}

// Event states
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum EventState {
    Open,
    Resolved,
}

// Errors
#[error_code]
pub enum CustomError {
    #[msg("Event is not open for betting.")]
    EventNotOpen,
    #[msg("Event has not been resolved.")]
    EventNotResolved,
    #[msg("Incorrect prediction.")]
    IncorrectPrediction,
}


#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(init, payer = admin, space = 8 + 256)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub escrow: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
}

#[derive(Accounts)]
pub struct ClaimPayout<'info> {
    #[account(mut, has_one = user)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub escrow: SystemAccount<'info>,
}
