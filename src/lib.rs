use kovi::{AllMsgEvent, PluginBuilder as p, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = p::get_runtime_bot();
    p::on_group_msg(move |e| on_group_msg(e, bot.clone()));
}

async fn on_group_msg(e: Arc<AllMsgEvent>, bot: Arc<RuntimeBot>) {
    let text = match e.borrow_text() {
        Some(v) => v,
        None => return,
    };

    if !text.starts_with("我要头衔") {
        return;
    }

    let title = text.trim_start_matches("我要头衔").trim().to_string();

    bot.set_group_special_title(e.group_id.unwrap(), e.user_id, &title);
}
