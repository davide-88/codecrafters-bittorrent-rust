use anyhow::Result;

pub trait Executor {
    fn execute(&self) -> Result<()>;
}
