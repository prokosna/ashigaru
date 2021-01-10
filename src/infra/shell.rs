use crate::domain;
use log::*;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct OrderExecutorShell {}

impl OrderExecutorShell {
  pub fn new() -> Self {
    OrderExecutorShell {}
  }
}

impl domain::OrderExecutor for OrderExecutorShell {
  fn execute(&self, orders: Vec<domain::Order>, duration: Duration) -> anyhow::Result<()> {
    for order in orders {
      thread::spawn(move || {
        let mut args: Vec<&str> = order.command.split_whitespace().collect();
        if args.len() <= 0 {
          return;
        }

        info!(
          "Will execute '{}' after {} secs",
          order.command,
          duration.as_secs()
        );

        thread::sleep(duration);
        let cmd = args.remove(0);
        let result = Command::new(cmd).args(args).output();
        if let Ok(output) = result {
          if output.status.success() {
            info!(
              "'{}' was done successfully: {}",
              order.command,
              std::str::from_utf8(&output.stdout).unwrap_or("")
            );
          } else {
            error!(
              "'{}' was failed: {}",
              order.command,
              std::str::from_utf8(&output.stderr).unwrap_or("")
            );
          }
        } else {
          error!("'{}' was failed", order.command);
        }
      });
    }
    Ok(())
  }
}
