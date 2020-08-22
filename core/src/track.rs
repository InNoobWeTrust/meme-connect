use crate::{block::Block, direction::Direction};

type StepIdx = usize;

#[derive(Debug, PartialEq)]
enum ExploreFlag {
    /// Not yet explored
    Wild,
    /// Explored, no need to look at it again
    Explored,
    /// Meet goal at current node
    Goal,
}

#[derive(Debug)]
struct TrackStep {
    /// Position in game board
    pos: Block,
    /// Indexes of children
    children: Vec<StepIdx>,
    /// Index of parent, point to self for root node
    parent: StepIdx,
    /// Spread direction from parent, not applicable for root node
    direction: Option<Direction>,
    /// Turn count from root node to this node
    turns: u8,
    /// Status of current node
    flag: ExploreFlag,
}

#[derive(Debug)]
pub struct Track {
    /// The start block
    start: Block,
    /// The goal block to reach
    goal: Block,
    /// Store the exploration information
    search_tree: Vec<TrackStep>,
    /// Current exploration progress in search tree
    current: StepIdx,
}

impl Track {
    pub fn new(start: Block, goal: Block) -> Self {
        Self {
            start,
            goal,
            search_tree: vec![TrackStep {
                pos: start,
                children: Vec::new(),
                parent: 0,
                direction: None,
                turns: 0,
                flag: ExploreFlag::Wild,
            }],
            current: 0,
        }
    }

    fn current_step(&self) -> &TrackStep {
        self.search_tree.get(self.current).unwrap()
    }

    fn current_step_mut(&mut self) -> &mut TrackStep {
        self.search_tree.get_mut(self.current).unwrap()
    }

    fn traversed(&self) -> Vec<Block> {
        self.search_tree.iter().map(|step| step.pos).collect()
    }

    pub fn goal_found(&self) -> bool {
        self.current_step().flag == ExploreFlag::Goal
    }

    /// Trace back steps from current step to the start
    pub fn backtrace(&self) -> Vec<Block> {
        let mut trace = Vec::new();
        let mut tracer = self.current_step();
        while tracer.pos != self.start {
            trace.push(tracer.pos);
            tracer = self.search_tree.get(tracer.parent).unwrap();
        }
        trace.push(self.start);
        trace
    }

    fn explore<F>(&self, validate: F) -> Vec<TrackStep>
    where
        F: Fn(&Block) -> bool,
    {
        let current_step = self.current_step();
        if current_step.flag != ExploreFlag::Wild {
            panic!("Should not be explored again");
        }
        current_step
            .pos
            .neighbours()
            .iter()
            .filter(|&(direction, _)| {
                // Don't step back
                current_step
                    .direction
                    .map_or(true, |curdir| !direction.is_opposite(curdir))
            })
            .filter(|&(_, pos)| !self.traversed().contains(pos)) // Only traverse once
            .filter(|&(_, pos)| *pos == self.goal || validate(pos)) // External validation for valid position
            .map(|&(direction, pos)| {
                let turns = if current_step.direction.map_or(false, |dir| direction != dir) {
                    current_step.turns + 1
                } else {
                    current_step.turns
                };
                let parent = self.current;
                // If goal is met, set goal status. Else, set wild
                let flag = if pos == self.goal {
                    ExploreFlag::Goal
                } else {
                    ExploreFlag::Wild
                };
                TrackStep {
                    pos,
                    children: Vec::new(),
                    parent,
                    direction: Some(direction),
                    turns,
                    flag,
                }
            })
            .filter(|step| step.turns <= 2) // As a rule of the game, max 2 turns
            .collect()
    }

    fn route_chooser<'a, T>(&self, routes: T) -> Option<StepIdx>
    where
        T: Iterator<Item = &'a StepIdx>,
    {
        // Smallest distance to goal is the choice amongst explorable routes
        routes.copied().min_by_key(|&idx| {
            self.search_tree
                .get(idx)
                .unwrap()
                .pos
                .distance_sqr(&self.goal)
        })
    }

    /// Search current step and take decision to move for next step.
    /// Return `true` if still need to continue, `false` if goal reached.
    /// `validate`: filter valid steps for exploration
    pub fn search<F>(&mut self, validate: F) -> bool
    where
        F: Fn(&Block) -> bool,
    {
        let next_idx = self.search_tree.len();
        // Explore children of current node and add to search tree
        let mut children = self.explore(validate);
        let mut current_step = self.current_step_mut();
        current_step
            .children
            .append(&mut (next_idx..next_idx + children.len()).collect());
        // Update flag of current node before modifying "self"
        current_step.flag = ExploreFlag::Explored;
        self.search_tree.append(&mut children);

        // Check if there is any goal in children
        if let Some((idx, _)) = self
            .search_tree
            .iter()
            .enumerate()
            .skip(next_idx)
            .find(|&(_, step)| step.flag == ExploreFlag::Goal)
        {
            // Jump to goal
            self.current = idx;
            // Return early
            return false;
        }

        // Try other wild nodes for better chance to meet goal
        let wild_nodes = self
            .search_tree
            .iter()
            .enumerate()
            .filter(|&(_, step)| step.flag == ExploreFlag::Wild)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        // Explore next wild node
        if let Some(next) = self.route_chooser(wild_nodes.iter()) {
            self.current = next;
            true
        } else {
            false
        }
    }
}
