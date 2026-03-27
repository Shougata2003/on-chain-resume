#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, String, Vec, Map,
};

// ─── Data Types ───────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub struct WorkExperience {
    pub company:    String,
    pub role:       String,
    pub start_year: u32,
    pub end_year:   u32,   // 0 = present
    pub description: String,
}

#[contracttype]
#[derive(Clone)]
pub struct Education {
    pub institution: String,
    pub degree:      String,
    pub field:       String,
    pub grad_year:   u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Resume {
    pub owner:       Address,
    pub name:        String,
    pub title:       String,
    pub bio:         String,
    pub skills:      Vec<String>,
    pub experience:  Vec<WorkExperience>,
    pub education:   Vec<Education>,
    pub endorsements: u32,
    pub verified:    bool,
}

// ─── Storage Keys ─────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Resume(Address),
    Endorsed(Address, Address), // (resume_owner, endorser)
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct OnChainResume;

#[contractimpl]
impl OnChainResume {

    /// Create or fully replace a resume for the caller's address.
    pub fn set_resume(
        env:        Env,
        owner:      Address,
        name:       String,
        title:      String,
        bio:        String,
        skills:     Vec<String>,
        experience: Vec<WorkExperience>,
        education:  Vec<Education>,
    ) {
        // Require the owner to have signed the transaction
        owner.require_auth();

        let resume = Resume {
            owner: owner.clone(),
            name,
            title,
            bio,
            skills,
            experience,
            education,
            endorsements: 0,
            verified: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Resume(owner.clone()), &resume);

        env.events().publish(
            (symbol_short!("RESUME"), symbol_short!("SET")),
            owner,
        );
    }

    /// Return the resume for a given address.
    pub fn get_resume(env: Env, owner: Address) -> Option<Resume> {
        env.storage()
            .persistent()
            .get(&DataKey::Resume(owner))
    }

    /// Endorse someone else's resume (one endorsement per unique endorser).
    pub fn endorse(env: Env, endorser: Address, resume_owner: Address) {
        endorser.require_auth();

        // Prevent self-endorsement
        if endorser == resume_owner {
            panic!("cannot endorse yourself");
        }

        let endo_key = DataKey::Endorsed(resume_owner.clone(), endorser.clone());

        // Idempotent: one endorsement per pair
        if env.storage().persistent().has(&endo_key) {
            panic!("already endorsed");
        }

        let mut resume: Resume = env
            .storage()
            .persistent()
            .get(&DataKey::Resume(resume_owner.clone()))
            .expect("resume not found");

        resume.endorsements += 1;

        env.storage()
            .persistent()
            .set(&DataKey::Resume(resume_owner.clone()), &resume);

        env.storage()
            .persistent()
            .set(&endo_key, &true);

        env.events().publish(
            (symbol_short!("ENDORSE"), symbol_short!("ADD")),
            (resume_owner, endorser),
        );
    }

    /// Check whether a specific address has endorsed a resume.
    pub fn has_endorsed(env: Env, endorser: Address, resume_owner: Address) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Endorsed(resume_owner, endorser))
    }

    /// Update only the skills list (owner only).
    pub fn update_skills(env: Env, owner: Address, skills: Vec<String>) {
        owner.require_auth();

        let mut resume: Resume = env
            .storage()
            .persistent()
            .get(&DataKey::Resume(owner.clone()))
            .expect("resume not found");

        resume.skills = skills;

        env.storage()
            .persistent()
            .set(&DataKey::Resume(owner.clone()), &resume);
    }

    /// Append a new work-experience entry (owner only).
    pub fn add_experience(env: Env, owner: Address, entry: WorkExperience) {
        owner.require_auth();

        let mut resume: Resume = env
            .storage()
            .persistent()
            .get(&DataKey::Resume(owner.clone()))
            .expect("resume not found");

        resume.experience.push_back(entry);

        env.storage()
            .persistent()
            .set(&DataKey::Resume(owner.clone()), &resume);
    }

    /// Append a new education entry (owner only).
    pub fn add_education(env: Env, owner: Address, entry: Education) {
        owner.require_auth();

        let mut resume: Resume = env
            .storage()
            .persistent()
            .get(&DataKey::Resume(owner.clone()))
            .expect("resume not found");

        resume.education.push_back(entry);

        env.storage()
            .persistent()
            .set(&DataKey::Resume(owner.clone()), &resume);
    }

    /// Delete caller's own resume.
    pub fn delete_resume(env: Env, owner: Address) {
        owner.require_auth();

        env.storage()
            .persistent()
            .remove(&DataKey::Resume(owner.clone()));

        env.events().publish(
            (symbol_short!("RESUME"), symbol_short!("DEL")),
            owner,
        );
    }
}
