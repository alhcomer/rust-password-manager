mod core;
mod core_test;

extern crate clap;

fn main() {
    let password = core::generate_password(10);
    println!("{}", password)
}

// TODO: Allow user to login using email and password
// TODO: Hash user password
// TODO: Provide user with recovery codes
// TODO: Store passwords to encrypted file
// TODO: Allow for ability to decrypt file and see password for a given account
// TODO: See list of passwords with associated accounts/workplaces/etc.
// TODO: Create main event loop
// TODO: After all main tasks, optimise code