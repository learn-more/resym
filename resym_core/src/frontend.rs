use anyhow::Result;

use crate::{backend::PDBSlot, diffing::DiffedType};

pub type TypeList = Vec<(String, pdb::TypeIndex)>;

pub enum FrontendCommand {
    LoadPDBResult(Result<PDBSlot>),
    UpdateFilteredTypes(TypeList),
    ReconstructTypeResult(Result<String>),
    DiffTypeResult(Result<DiffedType>),
}

pub trait FrontendController {
    fn send_command(&self, command: FrontendCommand) -> Result<()>;
}
