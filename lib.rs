use anchor_lang::prelude::*;

declare_id!("BVqaTBMKW1rTf9whnYRacoZYnhVXabW6qDtkYGqvnP9H");

#[program]
pub mod solticket {
    use super::*;

    pub fn create_event(ctx: Context<CreateEvent>, name: String) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.name = name;
        event.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn create_ticket(ctx: Context<CreateTicket>) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.owner = *ctx.accounts.owner.key;
        ticket.used = false;
        Ok(())
    }

    pub fn validate_ticket(ctx: Context<ValidateTicket>) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.used = true;
        Ok(())
    }
}

#[account]
pub struct Event {
    pub name: String,
    pub authority: Pubkey,
}

#[account]
pub struct Ticket {
    pub owner: Pubkey,
    pub used: bool,
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = authority, space = 8 + 64)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTicket<'info> {
    #[account(init, payer = owner, space = 8 + 40)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ValidateTicket<'info> {
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
}
