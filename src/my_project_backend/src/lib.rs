use::std::cell::RefCell;

thread_local! {
    static MSG: RefCell<String> = RefCell::new(String::new());
}

#[ic_cdk::update]
fn setMsg(newMsg: String) {
    MSG.with(|message| {
        *message.borrow_mut() = newMsg;
    })
}

#[ic_cdk::query]
fn getMsg() -> String {
    MSG.with(|msg| msg.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}