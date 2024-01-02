mod a_star;
pub(crate) use a_star::a_star_search;

mod dijkstra;
pub(crate) use dijkstra::dijkstra_search;

use crate::path::{Cost, Path};

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub(crate) struct HeuristicElement<Id>(pub Id, pub Cost, pub Cost);
impl<Id: Eq> PartialOrd for HeuristicElement<Id> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl<Id: Eq> Ord for HeuristicElement<Id> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.2.cmp(&self.2)
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct Element<Id>(pub Id, pub Cost);
impl<Id: Eq> PartialOrd for Element<Id> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl<Id: Eq> Ord for Element<Id> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.1.cmp(&self.1)
    }
}
