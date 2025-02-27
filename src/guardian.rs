use std::process::{Command, Child};
use std::time::{Duration, Instant};
use std::thread;
use std::error::Error;
use log::{info, error};
use crate::config::{Config, ProcessConfig};

pub struct Guardian {
    config: Config,
}

impl Guardian {
    pub fn new(config: Config) -> Self {
        println!("当前配置信息: {:?}", config);
        Self { config }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        for process in &self.config.processes {
            self.monitor_process(process)?;
        }
        Ok(())
    }

    fn monitor_process(&self, process_config: &ProcessConfig) -> Result<(), Box<dyn Error>> {
        let mut restart_count = 0;
        
        loop {
            let _start_time = Instant::now();
            
            info!("正在启动进程: {}", process_config.name);
            let mut child = self.spawn_process(process_config)?;

            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        error!("进程 {} 异常退出，退出状态: {}", process_config.name, status);
                    }
                }
                Err(e) => {
                    error!("等待进程时发生错误: {}", e);
                }
            }

            if let Some(max_restarts) = process_config.max_restarts {
                if max_restarts != -1 {
                    restart_count += 1;
                    if restart_count >= max_restarts {
                        error!("进程 {} 已达到最大重启次数", process_config.name);
                        break;
                    }
                }
            }

            thread::sleep(Duration::from_secs(process_config.restart_delay));
        }
        
        Ok(())
    }

    fn spawn_process(&self, process_config: &ProcessConfig) -> Result<Child, std::io::Error> {
        let mut command = Command::new(&process_config.executable_path);
        
        command.args(&process_config.arguments);
        
        if let Some(ref working_dir) = process_config.working_directory {
            command.current_dir(working_dir);
        }
        
        command.spawn()
    }
}