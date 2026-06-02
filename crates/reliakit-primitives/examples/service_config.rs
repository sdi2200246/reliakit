//! Real-world example: loading and validating a service configuration.
//!
//! This combines three Reliakit crates:
//!
//! - `reliakit-primitives` for typed, constrained fields validated at
//!   construction (`BoundedStr`, `Port`, `Percent`).
//! - `reliakit-secret` for a credential that must never leak through `Debug`,
//!   `Display`, or logs (`SecretString`).
//! - `reliakit-validate` for struct-level rules that span fields or encode
//!   policy beyond what a single field type can express.
//!
//! Run with: `cargo run -p reliakit-primitives --example service_config`

use reliakit_primitives::{BoundedStr, Percent, Port};
use reliakit_secret::{ExposeSecret, SecretString};
use reliakit_validate::{Validate, ValidationError, Violation};

/// A service name is between 3 and 32 characters.
type ServiceName = BoundedStr<3, 32>;

/// A fully typed service configuration.
///
/// Each field carries its own invariant, so by the time a `ServiceConfig`
/// exists, the per-field rules are already guaranteed.
struct ServiceConfig {
    name: ServiceName,
    port: Port,
    error_budget: Percent,
    api_key: SecretString,
}

impl Validate for ServiceConfig {
    type Error = ValidationError;

    /// Rules that span fields or encode policy the field types cannot express
    /// on their own. All violations are collected so the caller sees every
    /// problem at once, not just the first.
    fn validate(&self) -> Result<(), Self::Error> {
        let mut errors = ValidationError::empty();

        if self.api_key.expose_secret().len() < 8 {
            errors.push(Violation::with_field(
                "api_key",
                "must be at least 8 characters",
            ));
        }

        if self.error_budget.get() == 0 {
            errors.push(Violation::with_field(
                "error_budget",
                "must be greater than zero",
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Each field is validated once, at the boundary, as it is constructed.
    let config = ServiceConfig {
        name: ServiceName::new("api-service")?,
        port: Port::new(8080)?,
        error_budget: Percent::new(1)?,
        api_key: SecretString::from_string("rk_live_secret_value"),
    };

    // Struct-level validation for cross-field and policy rules.
    config.validate()?;

    // The secret is never printed, even via Display.
    println!(
        "service '{}' listening on port {}",
        config.name, config.port
    );
    println!("error budget: {}", config.error_budget);
    println!("api key (display): {}", config.api_key); // -> [REDACTED]

    // Explicit, auditable access is required to read the raw secret.
    println!("api key length: {}", config.api_key.expose_secret().len());

    Ok(())
}
