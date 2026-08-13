use indoc::indoc;
use predicates::prelude::*;
use std::error::Error;

mod common;
use common::OutputStream;
use common::run_codeowners;

/// A nested `.codeowner` naming an unregistered team, under one naming a real team:
/// ownership falls through to the ancestor, so nothing else reports the bad name.
#[test]
fn test_validate_reports_directory_codeowner_with_invalid_team() -> Result<(), Box<dyn Error>> {
    run_codeowners(
        "invalid-directory-codeowner",
        &["validate"],
        false,
        OutputStream::Stdout,
        predicate::str::contains(indoc! {"
            Found invalid team annotations
            - app/services/nested/.codeowner is referencing an invalid team - 'Web3'
        "}),
    )?;

    Ok(())
}
