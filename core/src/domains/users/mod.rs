//! # Users

/// Service
pub mod service;

/// Model
pub mod model;


/// GraphQL Mutations
pub mod mutations;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");
