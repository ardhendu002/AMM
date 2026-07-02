pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;
declare_id!("4ezuim4CP3uyumVRbQnDcy1UVKs4iAHcnrk7LvM2haXk");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool::handler(ctx)
    }
}
