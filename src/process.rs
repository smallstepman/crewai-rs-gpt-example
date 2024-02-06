use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Process {
    #[strum(serialize = "sequential")]
    Sequential,
    #[strum(serialize = "hierarchical")]
    Hierarchical,
    // TODO: Consensual = "consensual",
}
