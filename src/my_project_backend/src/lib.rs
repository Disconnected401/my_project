use std::call::RefCall;

thread_local! {
    static MSG: RefCall<String> = RefCall::new(String::new());
}

#[ic_cdk::update]
fn save_msg(msg: String) {
    MSG.with(|static_msg| *static_msg.borrow_mut() = msg);
}

#[ic_cdk::query]
fn get_msg() -> String {
    MSG.with(|static_msg| static_msg.borrow().clone())
}


#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
