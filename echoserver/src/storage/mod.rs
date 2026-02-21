
pub trait Storage: Send + Sync {

}

// () is a dummy provider that no-ops everything.
#[cfg(debug_assertions)]
impl Storage for () {

}