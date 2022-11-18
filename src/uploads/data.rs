use std::process::Command;
use serde::{Serialize, Deserialize};

// 配置信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info{
    pub src: String,
    pub addr: String,
    pub port: String,
}

// 调取配置信息
pub async fn get_info() -> Info{
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    let mut script = File::open(PathBuf::from("./config.yaml")).unwrap();
    let mut conten = String::new();
    script.read_to_string(&mut conten).unwrap();
    let info: Info = serde_yaml::from_str(&conten).unwrap();
    info
}

// 存储视频为m3u8格式
pub fn complete(path: (String, String)){  
    println!("{:?}", path);

    let output = if cfg!(target_os = "windows") {
        // windows系统
        Command::new("cmd").arg("/C")
                .arg("ffmpeg")
                .arg("-i")
                .arg(format!("{}", path.0)) //输入路径
                .args(["-profile:v","main","-level","3.0"])
                .args(["-start_number", "0", "-hls_time", "10", "-hls_list_size", "0"])
                .args(["-f", "hls"])
                .arg(format!("{}", path.1)) //输出路径
                .status()
                // .output()
                .expect("failed to execute process")
    } else {
        // linux系统
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .status()
                // .output()
                .expect("failed to execute process")
    };

    // 输出状态，判断完成或失败
    if output.success(){
        println!("ok")
    }else{
        println!("err")
    }

    // 输出结果
    // if String::from_utf8(output.stdout).unwrap() == ""{
    //     println!("ok");
    // }
}

