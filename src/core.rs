trait State {
    type Action;
    type ReverseAction;

    fn complete() -> bool;
    fn agents_count() -> u32;
    fn can_apply(agent_id: u32, action: Self::Action) -> bool;
    fn apply(agent_id: u32, action: Self::Action) -> Self::ReverseAction;
    fn unapply(agent_id: u32, reverse_action: Self::ReverseAction);
}

trait Problem {
    type State: State;
    type Error;

    fn load_state(data: Vec<u8>) -> Self::State;
    fn save_solution(solution: Vec<<<Self as crate::core::Problem>::State as State>::Action>) -> Vec<u8>;
    fn score_solution(solution: Vec<<<Self as crate::core::Problem>::State as State>::Action>) -> Result<u64, Self::Error>;
}

trait Strategy {
    type State: State;

    fn name() -> String;
    fn run(state: Self::State, callback: dyn Fn(Vec<<<Self as crate::core::Strategy>::State as State>::Action>) -> ());
}
