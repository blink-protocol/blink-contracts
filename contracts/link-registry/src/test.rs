extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn lifecycle_and_ownership() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(LinkRegistry, ());
    let client = LinkRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let other = Address::generate(&env);
    let alias = String::from_str(&env, "stellar");
    let first = String::from_str(&env, "https://stellar.org");
    let second = String::from_str(&env, "https://developers.stellar.org");

    let created = client.create(&owner, &alias, &first);
    assert_eq!(created.owner, owner);
    assert_eq!(client.resolve(&alias).unwrap().destination, first);
    assert_eq!(client.try_create(&other, &alias, &second), Err(Ok(Error::AliasTaken)));

    client.update(&owner, &alias, &second);
    assert_eq!(client.resolve(&alias).unwrap().destination, second);
    client.deactivate(&owner, &alias);
    assert_eq!(client.resolve(&alias), None);
}

#[test]
fn rejects_bad_input() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(LinkRegistry, ());
    let client = LinkRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);

    assert_eq!(
        client.try_create(
            &owner,
            &String::from_str(&env, "x"),
            &String::from_str(&env, "https://example.com")
        ),
        Err(Ok(Error::InvalidAlias))
    );
    assert_eq!(
        client.try_create(
            &owner,
            &String::from_str(&env, "Stellar"),
            &String::from_str(&env, "https://example.com")
        ),
        Err(Ok(Error::InvalidAlias))
    );
    assert_eq!(
        client.try_create(
            &owner,
            &String::from_str(&env, "ste_llar"),
            &String::from_str(&env, "https://example.com")
        ),
        Err(Ok(Error::InvalidAlias))
    );
    assert_eq!(
        client.try_create(
            &owner,
            &String::from_str(&env, "stellar"),
            &String::from_str(&env, "javascript:alert(1)")
        ),
        Err(Ok(Error::InvalidDestination))
    );
}

#[test]
fn wrong_owner_is_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(LinkRegistry, ());
    let client = LinkRegistryClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let other = Address::generate(&env);
    let alias = String::from_str(&env, "owned");
    let destination = String::from_str(&env, "https://example.com");

    client.create(&owner, &alias, &destination);
    assert_eq!(
        client.try_update(&other, &alias, &String::from_str(&env, "https://evil.example")),
        Err(Ok(Error::Unauthorized))
    );
    assert_eq!(client.try_deactivate(&other, &alias), Err(Ok(Error::Unauthorized)));
    assert_eq!(client.resolve(&alias).unwrap().destination, destination);
}
