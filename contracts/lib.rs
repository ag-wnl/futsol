use anchor_lang::prelude::*;

declare_id!("HzxAxnbb4tgkzeCuLVNTk2pQpdq4RjBewVBwz3SeEEU1");

#[program]
pub mod bets {
    use super::*;

    /// Initializes the betting account with two teams.
    pub fn initialize(ctx: Context<Initialize>, team_a: String, team_b: String) -> Result<()> {
        let betting_account = &mut ctx.accounts.betting_account;

        // Validate input lengths to prevent excessive storage use
        require!(team_a.len() <= 32 && team_b.len() <= 32, ErrorCode::InvalidTeamNameLength);

        betting_account.team_a = team_a;
        betting_account.team_b = team_b;
        betting_account.total_bets_a = 0;
        betting_account.total_bets_b = 0;
        betting_account.winner = None;

        Ok(())
    }

    /// Allows a user to place a bet on a team with a specified amount.
    pub fn place_bet(ctx: Context<PlaceBet>, team: String, amount: u64) -> Result<()> {
        let betting_account = &mut ctx.accounts.betting_account;
        let user_bet_account = &mut ctx.accounts.user_bet_account;

        // Ensure the user is betting on a valid team
        require!(
            team == betting_account.team_a || team == betting_account.team_b,
            ErrorCode::InvalidTeam
        );

        // Transfer funds to escrow
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.escrow_account.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.escrow_account.to_account_info(),
            ],
        )?;

        // Update the total bets for the selected team
        if team == betting_account.team_a {
            betting_account.total_bets_a = betting_account
                .total_bets_a
                .checked_add(amount)
                .ok_or(ErrorCode::AmountOverflow)?;
        } else {
            betting_account.total_bets_b = betting_account
                .total_bets_b
                .checked_add(amount)
                .ok_or(ErrorCode::AmountOverflow)?;
        }

        // Update user's bet account
        user_bet_account.user = *ctx.accounts.user.key;
        user_bet_account.team = team;
        user_bet_account.amount = user_bet_account
            .amount
            .checked_add(amount)
            .ok_or(ErrorCode::AmountOverflow)?;

        Ok(())
    }

    /// Announces the winning team and calculates odds.
    pub fn announce_winner(ctx: Context<AnnounceWinner>, winner: String) -> Result<()> {
        let betting_account = &mut ctx.accounts.betting_account;

        // Ensure winner is valid
        require!(
            winner == betting_account.team_a || winner == betting_account.team_b,
            ErrorCode::InvalidTeam
        );

        // Record the winner
        betting_account.winner = Some(winner);

        Ok(())
    }

    /// Distributes payouts to all users who bet on the winning team.
    pub fn distribute_payout(ctx: Context<DistributePayout>) -> Result<()> {
        let betting_account = &ctx.accounts.betting_account;
        let escrow_account = &mut ctx.accounts.escrow_account;
        let user_bet_account = &mut ctx.accounts.user_bet_account;
        let winner = betting_account.winner.clone().ok_or(ErrorCode::WinnerNotAnnounced)?;

        // Ensure this user bet on the winning team
        if user_bet_account.team != winner {
            return Ok(()); // Skip losers
        }

        // Calculate odds
        let total_pool = betting_account.total_bets_a + betting_account.total_bets_b;
        let winning_pool = if winner == betting_account.team_a {
            betting_account.total_bets_a
        } else {
            betting_account.total_bets_b
        };

        let odds = total_pool as f64 / winning_pool as f64;
        let payout = (user_bet_account.amount as f64 * odds) as u64;

        // Transfer payout to the user
        require!(escrow_account.amount >= payout, ErrorCode::InsufficientFunds);
        **escrow_account.to_account_info().lamports.borrow_mut() -= payout;
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += payout;

        Ok(())
    }
}

#[account]
pub struct BettingAccount {
    pub team_a: String,       // Name of team A
    pub team_b: String,       // Name of team B
    pub total_bets_a: u64,    // Total bets for team A
    pub total_bets_b: u64,    // Total bets for team B
    pub winner: Option<String>, // The winning team (None until decided)
}

#[account]
pub struct UserBetAccount {
    pub user: Pubkey,         // Address of the user who placed the bet
    pub team: String,         // Team the user bet on
    pub amount: u64,          // Total bet amount placed by the user
}

#[account]
pub struct EscrowAccount {
    pub amount: u64,          // Total funds held in escrow
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 8 + 32)]
    pub betting_account: Account<'info, BettingAccount>,
    #[account(init, payer = user, space = 8 + 8)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub betting_account: Account<'info, BettingAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(init_if_needed, payer = user, space = 8 + 32 + 32 + 8)]
    pub user_bet_account: Account<'info, UserBetAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AnnounceWinner<'info> {
    #[account(mut)]
    pub betting_account: Account<'info, BettingAccount>,
}

#[derive(Accounts)]
pub struct DistributePayout<'info> {
    #[account(mut)]
    pub betting_account: Account<'info, BettingAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub user_bet_account: Account<'info, UserBetAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid team specified.")]
    InvalidTeam,

    #[msg("Invalid team name length. Maximum 32 characters allowed.")]
    InvalidTeamNameLength,

    #[msg("Bet amount overflowed.")]
    AmountOverflow,

    #[msg("Winner not announced.")]
    WinnerNotAnnounced,

    #[msg("Insufficient funds in escrow.")]
    InsufficientFunds,
}
