trait Action {}
trait ReverseAction {}

trait State<A: Action, R: ReverseAction> {
    fn apply(actions: Vec<A>) -> Vec<R>;
    fn apply_one(agent_id: u32, action: A) -> R;
    fn unapply(reverse_actions: Vec<R>);
    fn unapply_one(agent_id: u32, reverse_action: R);
}

trait Problem<S: State<A, R>, A: Action, R: ReverseAction> {
    fn load_state(data: Vec<u8>) -> S;
    fn save_solution(solution: Vec<A>) -> Vec<u8>;
    fn score_solution(solution: Vec<A>) -> u64;
}

trait Strategy<S: State<A, R>, A: Action, R: ReverseAction> {
    fn name() -> String;
    fn run(state: S, callback: dyn Fn(Vec<A>) -> ());
}
