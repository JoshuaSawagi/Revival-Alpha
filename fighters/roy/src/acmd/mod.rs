use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod special_s4;
mod throws;
mod other;

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    //special_s4::install(agent);
    throws::install(agent);
    other::install(agent);
}