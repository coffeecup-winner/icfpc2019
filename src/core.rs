pub trait State {
    type Action;
    type ReverseAction;

    fn info(&self) -> String;
    fn complete(&self) -> bool;
    fn agents_count(&self) -> u32;
    fn can_apply(&self, agent_id: u32, action: Self::Action) -> bool;
    fn apply(&mut self, agent_id: u32, action: Self::Action) -> Self::ReverseAction;
    fn unapply(&mut self, agent_id: u32, reverse_action: Self::ReverseAction) -> Self::Action;
}

pub trait Problem {
    type State: State;
    type Error;

    type StateAction = <Self::State as State>::Action;

    fn load_state(data: Vec<u8>) -> Self::State;
    fn save_solution(solution: Vec<Self::StateAction>) -> Vec<u8>;
    fn score_solution(solution: Vec<Self::StateAction>) -> Result<u64, Self::Error>;
}

pub trait Strategy {
    type State: State;

    fn name() -> String;
    fn run(state: Self::State, callback: dyn Fn(Vec<<<Self as crate::core::Strategy>::State as State>::Action>) -> ());
}
