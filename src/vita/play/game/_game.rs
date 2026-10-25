// devela::vita::play::game::game

///
pub trait Game {
    ///
    type Role;
    ///
    type Position;
    ///
    type Action;
    ///
    type Judgment;

    ///
    fn legal_actions(pos: &Self::Position) -> impl Iterator<Item = Self::Action>;
    ///
    fn apply(pos: &Self::Position, action: Self::Action) -> Self::Position;
    ///
    fn judgment(pos: &Self::Position) -> Option<Self::Judgment>;
}
