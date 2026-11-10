//! Static-evidence maintenance algorithms. No default user paths or process launch.
pub mod auth;
pub mod batch_repair;
pub mod catalog;
pub mod cleanup;
pub mod config;
pub mod image_compat;
pub mod manager;
pub mod policy;
pub mod process;
pub mod router_capacity;
pub mod router_transition;
pub mod router_verification;
pub mod state_db;
pub mod threads;
pub mod transition_events;
pub mod transition_journal;
