use crate::gateway::Gateway;

pub struct Interactor {
    pub gateway: Box<dyn Gateway>,
}
