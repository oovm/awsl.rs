use sdl_pest::{
    Assoc::{Left, Right},
    Operator, PrecClimber, Rule,
};
use std::lazy::SyncLazy;

#[rustfmt::skip]
pub static PREC_CLIMBER: SyncLazy<PrecClimber<Rule>> = SyncLazy::new(|| {
    use Rule::*;
    //TODO: use macro
    PrecClimber::new(vec![
        Operator::new(Set, Left),
        Operator::new(Logical, Left),
        Operator::new(Additive, Left),
        Operator::new(Power, Right),
        Operator::new(Dot, Left)
    ])
});
