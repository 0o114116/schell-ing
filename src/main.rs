use crate::neighborhood::Neighborhood;

mod color;
mod neighborhood;

// TODO: eventually, allow comparison and switching between different neighborhoods
// TODO: eventually, allow for more specific preferences (indifference, mixes, etc.)

// TODO: if an actor can be better off by switching places with another actor who can move without
// TODO: being worse off, they should switch. this should be a toggle and a different one for each
// TODO: group

fn main() {
    // the bools in the array of tuples is basically 'are_preferences_all_or_nothing' for each group
    Neighborhood::random(10, 5).optimize([(3, false), (3, false)]);
    /* let mut hood = Neighborhood(vec![vec![None, Some(Color::White), Some(Color::White)], vec![Some(Color::White), Some(Color::White), Some(Color::White)]]);
    hood.optimize([2,2]) */
}
