use std::io::BufRead;
use std::io::Seek;

pub struct ChildProcess {
    pub config: x20::Configuration,
    pub binary: x20::Binary,
    offset: u64,
    log_file: String,
    binary_file: String,
    child: Option<std::process::Child>,
}

impl ChildProcess {
    pub fn new(base_dir: String, config: x20::Configuration, binary: x20::Binary) -> Self {
        let log_file = format!("{}/logs/{}", base_dir, config.get_binary_name());
        let binary_file = format!("{}/bin/{}", base_dir, config.get_binary_name());

        ChildProcess {
            config: config,
            binary: binary,
            offset: 0,
            log_file: log_file,
            binary_file: binary_file,
            child: None,
        }
    }

    pub fn start(&mut self) {
        if !self.binary.get_docker_img().is_empty() {
            return self.start_docker();
        }

        self.start_executable();
    }

    pub fn start_docker(&mut self) {
        // First, stop any existing containers
        let mut c = std::process::Command::new("docker");
        c.arg("stop");
        c.arg(self.config.get_name());
        c.output().unwrap();

        // Delete the container
        let mut c = std::process::Command::new("docker");
        c.arg("rm");
        c.arg(self.config.get_name());
        c.output().unwrap();

        // Start a new one
        let mut c = std::process::Command::new("docker");
        let f = std::fs::File::create(&self.log_file).unwrap();
        let f2 = std::fs::File::create(&self.log_file).unwrap();
        c.stdout(f);
        c.stderr(f2);
        c.arg("run");
        c.arg(format!("--name={}", self.config.get_name()));
        c.arg(format!("--net=host"));
        c.arg(format!(
            "{}@{}",
            self.binary.get_docker_img(),
            self.binary.get_docker_img_tag()
        ));
        for arg in self.config.get_docker_arguments() {
            c.arg(arg);
        }
        println!("start docker image: {}", self.binary.get_docker_img());
        self.child = Some(c.spawn().unwrap());
        println!("✔️ started `{}`", self.config.get_name());
    }

    pub fn start_executable(&mut self) {
        let f = std::fs::File::create(&self.log_file).unwrap();
        let f2 = std::fs::File::create(&self.log_file).unwrap();
        println!("start bin file: {}", self.binary_file);
        let mut c = std::process::Command::new(&self.binary_file);
        c.stdout(f);
        c.stderr(f2);
        for arg in self.config.get_arguments() {
            c.arg(format!("--{}={}", arg.get_name(), arg.get_value()));
        }
        self.child = Some(c.spawn().unwrap());
        println!("✔️ started `{}`", self.config.get_name());
    }

    pub fn run_to_completion(&mut self) -> bool {
        let output = match std::process::Command::new(&self.binary_file).output() {
            Ok(o) => o,
            Err(e) => {
                eprintln!(
                    "❌failed to start `{}`:\n\n {:?}",
                    self.config.get_binary_name(),
                    e
                );
                return false;
            }
        };

        let output_stderr = std::str::from_utf8(&output.stderr)
            .unwrap()
            .trim()
            .to_owned();
        if !output.status.success() {
            eprintln!(
                "❌process `{}` failed:\n\n {}",
                self.config.get_binary_name(),
                output_stderr
            );
            return false;
        }

        true
    }

    pub fn tail_logs(&mut self) {
        let mut f = std::fs::File::open(&self.log_file).unwrap();
        f.seek(std::io::SeekFrom::Start(self.offset)).unwrap();
        let mut reader = std::io::BufReader::new(f);
        let mut buf = String::new();
        while let Ok(length) = reader.read_line(&mut buf) {
            if length == 0 {
                break;
            }
            println!("[{}] {}", self.config.get_name(), buf.trim());
            buf.clear();
            self.offset += length as u64;
        }
    }

    pub fn check_alive(&mut self) -> bool {
        if let Some(child) = self.child.as_mut() {
            if let Some(exit_status) = child.try_wait().unwrap() {
                println!(
                    "❌process `{}` terminated with exit status {:?}",
                    self.config.get_name(),
                    exit_status
                );
                println!("❌shutting everything down",);
                return false;
            }
        }
        true
    }

    pub fn kill(&mut self) {
        if let Some(child) = self.child.as_mut() {
            match child.kill() {
                Ok(_) => (),
                Err(e) => eprintln!("failed to kill child: {:?}", e),
            }
        }
    }
}
