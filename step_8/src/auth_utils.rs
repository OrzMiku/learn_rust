// store the submodules in parent module's folder in also a good practice.
// models.rs is stored in the auth_utils folder.
pub mod models;

// You can use relative paths to refer to the parent module
pub fn login(creds : models::Credentials) {
    // ...Authenticate the user

    // You can also use super to refer to the parent module
    // super::database::get_user();

    // Or you can use the full path
    // crate:: is an absolute path that starts from the crate root
    crate::database::get_user();
}

fn logout() {
    // ...Logout the user
}
