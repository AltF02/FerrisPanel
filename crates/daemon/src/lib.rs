use programs::models::server::Server;

pub fn test() {
    let mut preset = Server::new("./presets/minecraft-paper.yml".to_string());
    preset.parse();
    preset.run_setup();

    let res = serde_yaml::to_string(&preset).unwrap();
    println!("{}", res);
}
