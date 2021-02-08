use backtrace;

use app::prelude::ROLLBAR_CLIENT;

pub fn init() {
  std::panic::set_hook(Box::new(move |panic_info| {
    let backtrace = backtrace::Backtrace::new();
    ROLLBAR_CLIENT.build_report()
      .from_panic(panic_info)
      .with_backtrace(&backtrace)
      .send();
  }));
}
