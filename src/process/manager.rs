use std::process::{Command, Child};
use std::sync::{Arc, Mutex};
use crate::process::Process;

struct ProcessManager{
    processes: Arc<Mutex<Vec<Process>>>,
    current_pid: u32,
}


impl ProcessManger{

    fn new(name: String) -> Self{
        ProcessManager{
            processes: Arc::new(Mutex::new(Vec::new())),
            current_pid: 0,
        }
    }

    fn list_processes(&self) -> None {
        let processes = self.processes.lock().unwrap();
        for process in processes.iter(){
            println!("Name: {}, PID: {}, Command: {}", process.name, process.pid, process.command);
        }
    }

fn attach(&self, process: Process) {
        
        
    }

}