use std::net::UdpSocket;
use std::process::Command;
use tiny_http::{Response, Server};

const HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width,initial-scale=1,user-scalable=no">
<title>nyamote</title>
<style>
*{margin:0;padding:0;box-sizing:border-box;touch-action:none;user-select:none}
body{background:#1a1a2e;color:#eee;font-family:system-ui;height:100vh;display:flex;flex-direction:column}
.pad{flex:1;background:#16213e;border-radius:16px;margin:12px;display:flex;align-items:center;justify-content:center;font-size:24px;color:#555}
.btns{display:flex;gap:12px;padding:0 12px 12px}
.btn{flex:1;padding:24px;background:#0f3460;border-radius:12px;text-align:center;font-size:18px;font-weight:bold}
.btn:active{background:#e94560}
.scroll{display:flex;gap:8px;padding:0 12px 12px}
.scroll .btn{flex:1;padding:16px}
.keys{display:flex;gap:8px;padding:0 12px 12px}
.keys .btn{padding:16px;font-size:14px}
</style>
</head>
<body>
<div class="pad" id="pad">touch to move</div>
<div class="btns">
<div class="btn" id="left">LEFT</div>
<div class="btn" id="right">RIGHT</div>
</div>
<div class="scroll">
<div class="btn" id="su">SCROLL ↑</div>
<div class="btn" id="sd">SCROLL ↓</div>
</div>
<div class="keys">
<div class="btn" id="space">SPACE</div>
<div class="btn" id="esc">ESC</div>
<div class="btn" id="enter">ENTER</div>
</div>
<script>
const s=(u,d)=>fetch('/api',{method:'POST',body:JSON.stringify(d)});
const pad=document.getElementById('pad');
let lx=0,ly=0;
pad.ontouchstart=e=>{e.preventDefault();lx=e.touches[0].clientX;ly=e.touches[0].clientY};
pad.ontouchmove=e=>{
  e.preventDefault();
  const x=e.touches[0].clientX,y=e.touches[0].clientY;
  const dx=Math.round((x-lx)*1.5),dy=Math.round((y-ly)*1.5);
  if(dx||dy)s('/api',{m:[dx,dy]});
  lx=x;ly=y;
};
document.getElementById('left').ontouchstart=e=>{e.preventDefault();s('/api',{c:'l'})};
document.getElementById('right').ontouchstart=e=>{e.preventDefault();s('/api',{c:'r'})};
document.getElementById('su').ontouchstart=e=>{e.preventDefault();s('/api',{s:'u'})};
document.getElementById('sd').ontouchstart=e=>{e.preventDefault();s('/api',{s:'d'})};
document.getElementById('space').ontouchstart=e=>{e.preventDefault();s('/api',{k:'space'})};
document.getElementById('esc').ontouchstart=e=>{e.preventDefault();s('/api',{k:'esc'})};
document.getElementById('enter').ontouchstart=e=>{e.preventDefault();s('/api',{k:'enter'})};
</script>
</body>
</html>"#;

fn get_local_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").ok();
    socket
        .local_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn ydotool(args: &[&str]) {
    let _ = Command::new("ydotool")
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

fn main() {
    let port = std::env::args()
        .nth(1)
        .and_then(|p| p.parse().ok())
        .unwrap_or(8888);

    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).expect("Failed to start server");

    let ip = get_local_ip();
    println!("nyamote running :3");
    println!("open http://{}:{} on your phone", ip, port);

    for mut req in server.incoming_requests() {
        let path = req.url().to_string();

        if path == "/" {
            let resp = Response::from_string(HTML).with_header(
                "Content-Type: text/html".parse::<tiny_http::Header>().unwrap(),
            );
            let _ = req.respond(resp);
        } else if path == "/api" {
            let mut body = String::new();
            let _ = req.as_reader().read_to_string(&mut body);

            if body.contains("\"m\"") {
                if let (Some(start), Some(end)) = (body.find('['), body.find(']')) {
                    let coords: Vec<i32> = body[start + 1..end]
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if coords.len() == 2 && (coords[0] != 0 || coords[1] != 0) {
                        ydotool(&[
                            "mousemove",
                            "--",
                            &coords[0].to_string(),
                            &coords[1].to_string(),
                        ]);
                    }
                }
            } else if body.contains("\"c\":\"l\"") {
                ydotool(&["click", "0xC0"]);
            } else if body.contains("\"c\":\"r\"") {
                ydotool(&["click", "0xC1"]);
            } else if body.contains("\"s\":\"u\"") {
                ydotool(&["mousemove", "--wheel", "-y", "-3"]);
            } else if body.contains("\"s\":\"d\"") {
                ydotool(&["mousemove", "--wheel", "-y", "3"]);
            } else if body.contains("\"k\":\"space\"") {
                ydotool(&["key", "57:1", "57:0"]);
            } else if body.contains("\"k\":\"esc\"") {
                ydotool(&["key", "1:1", "1:0"]);
            } else if body.contains("\"k\":\"enter\"") {
                ydotool(&["key", "28:1", "28:0"]);
            }

            let resp = Response::from_string("ok");
            let _ = req.respond(resp);
        } else {
            let resp = Response::from_string("404").with_status_code(404);
            let _ = req.respond(resp);
        }
    }
}
