use super::Module;

pub(crate) fn create_header() -> Option<Module> {
    let title = format!("{} on {}", whoami::distro(), whoami::hostname());

    let body = format!("The current user is {}.", whoami::username());

    Some(Module::new(title, body))
}
