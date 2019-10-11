use failure::Error;

pub trait Instruction {
    fn size(&self) -> Result<usize, Error>;
    fn get_cycles(&self) -> Result<usize, Error>;
}
