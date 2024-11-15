#[derive(strum::IntoStaticStr)]
pub enum DialogTitle {
    #[strum(serialize = "Operation not allowed")]
    OperationNotAllowed,
}
