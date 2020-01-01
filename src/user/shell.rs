use crate::{print, user, kernel};
use heapless::{String, FnvIndexSet, Vec};
use heapless::consts::*;

#[repr(u8)]
pub enum ExitCode {
    CommandSuccessful = 0,
    CommandUnknown    = 1,
    CommandError      = 2,
    ShellExit         = 255,
}

pub struct Shell {
    cmd: String<U256>,
    history: FnvIndexSet<String<U256>, U256>,
    history_index: usize,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            cmd: String::new(),
            history: FnvIndexSet::new(),
            history_index: 0,
        }
    }

    pub fn run(&mut self) -> user::shell::ExitCode {
        self.print_prompt();
        loop {
            let c = kernel::console::get_char();
            match c {
                '\0' => {
                    continue;
                }
                '\n' => {
                    print!("\n");
                    if self.cmd.len() > 0 {
                        // Remove first command from history if full
                        if self.history.len() == self.history.capacity() {
                            let first = self.history.iter().next().unwrap().clone();
                            self.history.remove(&first);
                        }

                        // Add or move command to history at the end
                        let cmd = self.cmd.clone();
                        self.history.remove(&cmd);
                        if self.history.insert(cmd).is_ok() {
                            self.history_index = self.history.len();
                        }

                        let line = self.cmd.clone();
                        match self.exec(&line) {
                            ExitCode::CommandSuccessful => {},
                            ExitCode::ShellExit => { return ExitCode::CommandSuccessful },
                            _ => { print!("?\n") },
                        }
                        self.cmd.clear();
                    }
                    self.print_prompt();
                },
                '\x08' => { // Backspace
                    if self.cmd.len() > 0 {
                        self.cmd.pop();
                        print!("\x08");
                    }
                },
                '↑' => { // Arrow up
                    if self.history.len() > 0 {
                        if self.history_index > 0 {
                            self.history_index -= 1;
                        }
                        if let Some(cmd) = self.history.iter().nth(self.history_index) {
                            let n = self.cmd.len();
                            for _ in 0..n {
                                print!("\x08");
                            }
                            self.cmd = cmd.clone();
                            print!("{}", cmd);
                        }
                    }
                },
                '↓' => { // Arrow down
                    if self.history.len() > 0 {
                        if self.history_index < self.history.len() - 1 {
                            self.history_index += 1;
                        }
                        if let Some(cmd) = self.history.iter().nth(self.history_index) {
                            let n = self.cmd.len();
                            for _ in 0..n {
                                print!("\x08");
                            }
                            self.cmd = cmd.clone();
                            print!("{}", self.cmd);
                        }
                    }
                },
                c => {
                    if c.is_ascii_graphic() || c.is_ascii_whitespace() {
                        if self.cmd.push(c).is_ok() {
                            print!("{}", c);
                        }
                    }
                },
            }
        }
    }

    pub fn exec(&self, cmd: &str) -> ExitCode {
        let args: Vec<&str, U256> = cmd.split_whitespace().collect();
        match args[0] {
            "a" | "alias"                       => ExitCode::CommandUnknown,
            "b"                                 => ExitCode::CommandUnknown,
            "c" | "copy" | "cp"                 => ExitCode::CommandUnknown,
            "d" | "del" | "delete" | "rm"       => ExitCode::CommandUnknown,
            "e" | "edit"                        => ExitCode::CommandUnknown,
            "f" | "find"                        => ExitCode::CommandUnknown,
            "g" | "gd" | "go" | "go-dir" | "cd" => ExitCode::CommandUnknown,
            "h" | "help"                        => ExitCode::CommandUnknown,
            "i"                                 => ExitCode::CommandUnknown,
            "j" | "jd" | "jump" | "jump-dir"    => ExitCode::CommandUnknown,
            "k" | "kill"                        => ExitCode::CommandUnknown,
            "l" | "list" | "ls"                 => ExitCode::CommandUnknown,
            "m" | "move" | "mv"                 => user::r#move::main(&args),
            "n"                                 => ExitCode::CommandUnknown,
            "o"                                 => ExitCode::CommandUnknown,
            "p" | "print" | "echo"              => user::print::main(&args),
            "q" | "quit" | "exit"               => ExitCode::ShellExit,
            "r" | "read" | "cat"                => user::read::main(&args),
            "s"                                 => ExitCode::CommandUnknown,
            "t" | "tag"                         => ExitCode::CommandUnknown,
            "u"                                 => ExitCode::CommandUnknown,
            "v"                                 => ExitCode::CommandUnknown,
            "w" | "write"                       => user::write::main(&args),
            "x"                                 => ExitCode::CommandUnknown,
            "y"                                 => ExitCode::CommandUnknown,
            "z"                                 => ExitCode::CommandUnknown,
            "rd" | "read-dir"                   => ExitCode::CommandUnknown,
            "wd" | "write-dir" | "mkdir"        => ExitCode::CommandUnknown,
            "shell"                             => user::shell::main(&args),
            "sleep"                             => user::sleep::main(&args),
            "clear"                             => user::clear::main(&args),
            "login"                             => user::login::main(&args),
            _                                   => ExitCode::CommandUnknown,
        }
    }

    fn print_prompt(&self) {
        print!("\n> ");
    }
}

pub fn main(args: &[&str]) -> ExitCode {
    let mut shell = Shell::new();
    match args.len() {
        1 => {
            return shell.run();
        },
        2 => {
            let pathname = args[1];
            if let Some(file) = kernel::fs::File::open(pathname) {
                for line in file.read().split("\n") {
                    if line.len() > 0 {
                        shell.exec(line);
                    }
                }
                ExitCode::CommandSuccessful
            } else {
                print!("File not found '{}'\n", pathname);
                ExitCode::CommandError
            }
        },
        _ => {
            ExitCode::CommandError
        },
    }
}
