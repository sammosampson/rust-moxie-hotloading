use app_state::*;
use mox::mox;
use moxie::Key;
use illicit::*;
use engine::embedding::*;

pub enum Message {
    AddControl
}

pub trait StateProcessor<TMessage> {
    fn process(&self, message: TMessage) -> Self;
}

impl StateProcessor<Message> for TestState {
    fn process(&self, message: Message) -> Self {
        match message {
            Message::AddControl => {
                Self {
                    control_count: self.control_count + 2,
                    ..*self
                }
            }
        }
    }
}

#[topo::nested]
pub fn app_root() -> RootNode<TestState> {
    increase_control_count();
    mox!(
        <root>
            <test_control />
            <visibilty_test_control />
            <multi_test_controls />
        </root>
    )
}

#[topo::nested]
#[from_env(state: &Key<TestState>)]
fn increase_control_count() {
    state.update(|state| Some(state.process(Message::AddControl)));
}

#[topo::nested]
#[from_env(state: &Key<TestState>)]
fn multi_test_controls() -> Vec<Node> {
    println!("count: {:?}", state.control_count);
    (0..state.control_count).map(|_| {
        mox!(<test_control />)
    }).collect::<Vec<_>>()
}

#[topo::nested]
#[from_env(state: &Key<TestState>)]
fn visibilty_test_control() -> Node {
    if state.test_control == 1 {
        mox!(<test_control />)
    } else {
        mox!(<test_control_2 />)
    }
}

#[topo::nested]
fn test_control() -> Node {
    mox!(
        <vertical_stack>
            <circle radius=10 stroke_width=5 />
        </vertical_stack>
    )
}

#[topo::nested]
fn test_control_2() -> Node {
    mox!(
        <vertical_stack>
            <circle radius=11 stroke_width=6 />
        </vertical_stack>
    )
}