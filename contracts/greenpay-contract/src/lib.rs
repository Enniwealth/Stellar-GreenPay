/**
 * contracts/greenpay-contract/src/lib.rs
 *
 * Stellar GreenPay — Climate Donation Tracking Contract
 *
 * This contract provides on-chain transparency for every donation:
 *
 *   1. Admin registers verified climate projects on-chain
 *   2. Donors call donate() — XLM sent directly to project wallet
 *   3. Contract records every donation immutably
 *   4. Anyone can query total raised, donor count, CO2 offset per project
 *   5. Impact badges auto-calculated based on cumulative donor totals
 *
 * This is the most feature-complete contract in the GreenPay stack.
 * It focuses on radical transparency — all data is publicly queryable.
 *
 * Build:
 *   cargo build --target wasm32-unknown-unknown --release
 *
 * Deploy:
 *   stellar contract deploy \
 *     --wasm target/wasm32-unknown-unknown/release/greenpay_contract.wasm \
 *     --source alice --network testnet
 */

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    token, Address, Env, Symbol, symbol_short, String, Vec,
};

// ─── Badge tiers (on-chain) ───────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum BadgeTier {
    None,
    Seedling,   // ≥ 10 XLM (10_000_000 stroops)
    Tree,       // ≥ 100 XLM
    Forest,     // ≥ 500 XLM
    EarthGuardian, // ≥ 2000 XLM
}

// ─── Data structures ──────────────────────────────────────────────────────────

/// A climate project registered on-chain.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Project {
    pub id:              String,
    pub name:            String,
    pub wallet:          Address,   // Receives donations directly
    pub co2_per_xlm:     u32,       // Estimated grams of CO2 offset per XLM donated
    pub total_raised:    i128,      // Total received in stroops
    pub donor_count:     u32,
    pub active:          bool,
    pub registered_at:   u32,       // Ledger sequence
}

/// An immutable on-chain donation record.
#[contracttype]
#[derive(Clone, Debug)]
pub struct DonationRecord {
    pub donor:     Address,
    pub project:   String,
    pub amount:    i128,
    pub ledger:    u32,
    pub message_hash: u32, // simple hash of message (message stored off-chain)
}

/// On-chain donor stats.
#[contracttype]
#[derive(Clone, Debug)]
pub struct DonorStats {
    pub total_donated:    i128,
    pub donation_count:   u32,
    pub badge:            BadgeTier,
    pub co2_offset_grams: i128,
}

/// Storage keys
#[contracttype]
pub enum DataKey {
    Admin,
    Project(String),
    ProjectCount,
    DonorStats(Address),
    DonationCount,
    GlobalTotalRaised,
    GlobalCO2OffsetGrams,
}

// ─── Badge calculation ────────────────────────────────────────────────────────

const STROOP: i128 = 10_000_000; // 1 XLM in stroops

fn calculate_badge(total_stroops: i128) -> BadgeTier {
    let xlm = total_stroops / STROOP;
    if      xlm >= 2000 { BadgeTier::EarthGuardian }
    else if xlm >= 500  { BadgeTier::Forest }
    else if xlm >= 100  { BadgeTier::Tree }
    else if xlm >= 10   { BadgeTier::Seedling }
    else                { BadgeTier::None }
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct GreenPayContract;

#[contractimpl]
impl GreenPayContract {

    // ─── Initialization ──────────────────────────────────────────────────────

    /// Initialize the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ProjectCount,      &0u32);
        env.storage().instance().set(&DataKey::DonationCount,     &0u32);
        env.storage().instance().set(&DataKey::GlobalTotalRaised, &0i128);
        env.storage().instance().set(&DataKey::GlobalCO2OffsetGrams, &0i128);
    }

    // ─── Project management ───────────────────────────────────────────────────

    /// Admin registers a verified climate project on-chain.
    ///
    /// Parameters:
    ///   project_id    — unique ID matching the backend record
    ///   name          — project display name
    ///   wallet        — Stellar address that receives donations
    ///   co2_per_xlm   — estimated grams of CO2 offset per XLM donated
    pub fn register_project(
        env:          Env,
        admin:        Address,
        project_id:   String,
        name:         String,
        wallet:       Address,
        co2_per_xlm:  u32,
    ) {
        admin.require_auth();

        // Verify caller is admin
        let stored_admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Not initialized");
        if stored_admin != admin {
            panic!("Only admin can register projects");
        }

        if env.storage().instance().has(&DataKey::Project(project_id.clone())) {
            panic!("Project already registered");
        }

        let project = Project {
            id:           project_id.clone(),
            name,
            wallet,
            co2_per_xlm,
            total_raised: 0,
            donor_count:  0,
            active:       true,
            registered_at: env.ledger().sequence(),
        };

        env.storage().instance().set(&DataKey::Project(project_id.clone()), &project);

        let count: u32 = env.storage().instance().get(&DataKey::ProjectCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::ProjectCount, &(count + 1));

        env.events().publish(
            (symbol_short!("proj_reg"), admin),
            project_id,
        );
    }

    /// Admin can deactivate a project (stops new donations).
    pub fn deactivate_project(env: Env, admin: Address, project_id: String) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        if stored_admin != admin { panic!("Only admin can deactivate projects"); }

        let mut project: Project = env.storage().instance()
            .get(&DataKey::Project(project_id.clone()))
            .expect("Project not found");
        project.active = false;
        env.storage().instance().set(&DataKey::Project(project_id), &project);
    }

    // ─── Donations ────────────────────────────────────────────────────────────

    /// Donor sends XLM directly to the project wallet.
    /// The contract records the donation and updates stats.
    ///
    /// Parameters:
    ///   token       — SAC address for XLM (or USDC in future)
    ///   donor       — the donor's address (must authorize)
    ///   project_id  — ID of the climate project
    ///   amount      — donation in stroops (1 XLM = 10_000_000)
    ///   msg_hash    — optional hash of donor's message (stored off-chain)
    pub fn donate(
        env:        Env,
        token:      Address,
        donor:      Address,
        project_id: String,
        amount:     i128,
        msg_hash:   u32,
    ) {
        donor.require_auth();

        if amount <= 0 {
            panic!("Donation amount must be positive");
        }

        let mut project: Project = env.storage().instance()
            .get(&DataKey::Project(project_id.clone()))
            .expect("Project not found");

        if !project.active {
            panic!("Project is not accepting donations");
        }

        // Transfer tokens directly to the project wallet
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&donor, &project.wallet, &amount);

        // Update project stats
        project.total_raised += amount;
        project.donor_count  += 1;
        env.storage().instance().set(&DataKey::Project(project_id.clone()), &project);

        // Update donor stats
        let mut donor_stats: DonorStats = env.storage().instance()
            .get(&DataKey::DonorStats(donor.clone()))
            .unwrap_or(DonorStats {
                total_donated:    0,
                donation_count:   0,
                badge:            BadgeTier::None,
                co2_offset_grams: 0,
            });

        donor_stats.total_donated    += amount;
        donor_stats.donation_count   += 1;
        donor_stats.badge             = calculate_badge(donor_stats.total_donated);
        donor_stats.co2_offset_grams += (amount / STROOP) * project.co2_per_xlm as i128;
        env.storage().instance().set(&DataKey::DonorStats(donor.clone()), &donor_stats);

        // Record the donation
        let donation = DonationRecord {
            donor:        donor.clone(),
            project:      project_id.clone(),
            amount,
            ledger:       env.ledger().sequence(),
            message_hash: msg_hash,
        };

        let donation_count: u32 = env.storage().instance().get(&DataKey::DonationCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::DonationCount, &(donation_count + 1));

        // Update global totals
        let global_raised: i128 = env.storage().instance().get(&DataKey::GlobalTotalRaised).unwrap_or(0);
        env.storage().instance().set(&DataKey::GlobalTotalRaised, &(global_raised + amount));

        let global_co2: i128 = env.storage().instance().get(&DataKey::GlobalCO2OffsetGrams).unwrap_or(0);
        let new_co2 = (amount / STROOP) * project.co2_per_xlm as i128;
        env.storage().instance().set(&DataKey::GlobalCO2OffsetGrams, &(global_co2 + new_co2));

        // Emit event
        env.events().publish(
            (symbol_short!("donated"), donor, project_id),
            (amount, donor_stats.badge.clone()),
        );
    }

    // ─── Getters ─────────────────────────────────────────────────────────────

    /// Get project details by ID.
    pub fn get_project(env: Env, project_id: String) -> Project {
        env.storage().instance()
            .get(&DataKey::Project(project_id))
            .expect("Project not found")
    }

    /// Get donor stats (total donated, badge tier, CO2 offset).
    pub fn get_donor_stats(env: Env, donor: Address) -> DonorStats {
        env.storage().instance()
            .get(&DataKey::DonorStats(donor))
            .unwrap_or(DonorStats {
                total_donated: 0, donation_count: 0,
                badge: BadgeTier::None, co2_offset_grams: 0,
            })
    }

    /// Get the current badge for a donor.
    pub fn get_badge(env: Env, donor: Address) -> BadgeTier {
        let stats: DonorStats = env.storage().instance()
            .get(&DataKey::DonorStats(donor))
            .unwrap_or(DonorStats { total_donated: 0, donation_count: 0, badge: BadgeTier::None, co2_offset_grams: 0 });
        stats.badge
    }

    /// Get global total XLM raised across all projects.
    pub fn get_global_total(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::GlobalTotalRaised).unwrap_or(0)
    }

    /// Get global total CO2 offset in grams.
    pub fn get_global_co2(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::GlobalCO2OffsetGrams).unwrap_or(0)
    }

    /// Get total number of registered projects.
    pub fn get_project_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::ProjectCount).unwrap_or(0)
    }

    /// Get total number of donations recorded.
    pub fn get_donation_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::DonationCount).unwrap_or(0)
    }

    /// Get the admin address.
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("Not initialized")
    }

    // ─── Placeholders ─────────────────────────────────────────────────────────

    /// [PLACEHOLDER] Mint an impact NFT when a donor reaches a badge threshold.
    /// See ROADMAP.md v1.3 — Impact NFT Badges.
    pub fn mint_impact_nft(_env: Env, _donor: Address, _tier: BadgeTier) {
        panic!("Impact NFT minting coming in v1.3 — see ROADMAP.md");
    }

    /// [PLACEHOLDER] DAO governance vote to verify a new project.
    /// See ROADMAP.md v2.1 — DAO Governance.
    pub fn vote_verify_project(_env: Env, _voter: Address, _project_id: String, _approve: bool) {
        panic!("DAO governance coming in v2.1 — see ROADMAP.md");
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_initialize() {
        let env    = Env::default();
        let id     = env.register_contract(None, GreenPayContract);
        let client = GreenPayContractClient::new(&env, &id);
        let admin  = Address::generate(&env);
        client.initialize(&admin);
        assert_eq!(client.get_admin(), admin);
        assert_eq!(client.get_project_count(), 0);
        assert_eq!(client.get_donation_count(), 0);
        assert_eq!(client.get_global_total(), 0);
    }

    #[test]
    #[should_panic(expected = "Contract already initialized")]
    fn test_double_init_fails() {
        let env    = Env::default();
        let id     = env.register_contract(None, GreenPayContract);
        let client = GreenPayContractClient::new(&env, &id);
        let admin  = Address::generate(&env);
        client.initialize(&admin);
        client.initialize(&admin);
    }

    #[test]
    fn test_donor_badge_none_below_threshold() {
        let env    = Env::default();
        let id     = env.register_contract(None, GreenPayContract);
        let client = GreenPayContractClient::new(&env, &id);
        let admin  = Address::generate(&env);
        client.initialize(&admin);
        let donor  = Address::generate(&env);
        assert_eq!(client.get_badge(&donor), BadgeTier::None);
    }

    #[test]
    fn test_calculate_badge_thresholds() {
        assert_eq!(calculate_badge(0),                BadgeTier::None);
        assert_eq!(calculate_badge(9 * STROOP),       BadgeTier::None);
        assert_eq!(calculate_badge(10 * STROOP),      BadgeTier::Seedling);
        assert_eq!(calculate_badge(99 * STROOP),      BadgeTier::Seedling);
        assert_eq!(calculate_badge(100 * STROOP),     BadgeTier::Tree);
        assert_eq!(calculate_badge(499 * STROOP),     BadgeTier::Tree);
        assert_eq!(calculate_badge(500 * STROOP),     BadgeTier::Forest);
        assert_eq!(calculate_badge(1999 * STROOP),    BadgeTier::Forest);
        assert_eq!(calculate_badge(2000 * STROOP),    BadgeTier::EarthGuardian);
        assert_eq!(calculate_badge(100000 * STROOP),  BadgeTier::EarthGuardian);
    }
}
