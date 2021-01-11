use crate::domain;
use crate::domain::Event;
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
  fn execute(&self, orders: Vec<domain::Order>, event: Event) -> anyhow::Result<()> {
    let duration = match event.duration {
      Some(x) => Duration::from_secs(x),
      None => Duration::from_secs(0),
    };

    for order in orders {
      let event1 = event.clone();
      thread::spawn(move || {
        let command = match event1.arg {
          Some(arg) => format!("{} {}", order.command, arg),
          None => order.command,
        };

        let mut args: Vec<&str> = command.split_whitespace().collect();
        if args.len() <= 0 {
          return;
        }

        info!(
          "Will execute '{}' after {} secs",
          command,
          duration.as_secs()
        );

        thread::sleep(duration);
        let cmd = args.remove(0);
        let result = Command::new(cmd).args(args).output();
        if let Ok(output) = result {
          if output.status.success() {
            info!(
              "'{}' was done successfully: {}",
              command,
              std::str::from_utf8(&output.stdout).unwrap_or("")
            );
          } else {
            error!(
              "'{}' was failed: {}",
              command,
              std::str::from_utf8(&output.stderr).unwrap_or("")
            );
          }
        } else {
          error!("'{}' was failed", command);
        }
      });
    }
    Ok(())
  }
}
