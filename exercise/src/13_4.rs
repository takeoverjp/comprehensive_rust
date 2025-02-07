pub trait Logger {
  /// 指定された詳細度レベルでメッセージをログに記録します。
  fn log(&self, verbosity: u8, message: &str);
}

struct StdoutLogger;

impl Logger for StdoutLogger {
  fn log(&self, verbosity: u8, message: &str) {
      println!("verbosity={verbosity}: {message}");
  }
}

// TODO: `VerbosityFilter` を定義して実装します。
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StdoutLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
  let logger = VerbosityFilter { max_verbosity: 3, inner: StdoutLogger };
  logger.log(5, "FYI");
  logger.log(2, "Uhoh");
}
