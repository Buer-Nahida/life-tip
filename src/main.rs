use notify_rust::Notification;
use std::{error::Error, result, thread, time::Duration};

type Result<T = ()> = result::Result<T, Box<dyn Error>>;

fn main() -> Result {
    loop {
        thread::sleep(Duration::from_secs(30 * 60));
        Notification::new()
            .summary("已经过了三十分钟了")
            .body("休息下吧，比如痛痛快快去上个厕所什么的")
            .appname("Life Tip")
            .timeout(Duration::from_secs(3))
            .show()?;
    }
}
