pub mod link;
pub mod paragraph;

use crate::models::components::link::Link;
use crate::models::components::paragraph::Paragraph;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum Component {
    Link(Link),
    Paragraph(Paragraph),
}
