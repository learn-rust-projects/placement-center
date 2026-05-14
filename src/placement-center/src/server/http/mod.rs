pub mod index;
pub mod server;
pub(crate) fn v1_path(path: &str) -> String {
    format!("/v1{path}")
}

pub(crate) fn path_create(path: &str) -> String {
    format!("{path}/create")
}

pub(crate) fn path_update(path: &str) -> String {
    format!("{path}/update")
}

pub(crate) fn path_delete(path: &str) -> String {
    format!("{path}/delete")
}

pub(crate) fn path_list(path: &str) -> String {
    format!("{path}/list")
}
