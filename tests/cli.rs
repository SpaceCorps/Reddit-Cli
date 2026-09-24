//! In-process TCP mock tests for `reddit`.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

#[derive(Clone, Debug)]
struct Recorded {
    method: String,
    path: String,
    #[allow(dead_code)]
    headers: Vec<(String, String)>,
    body: Option<Value>,
}

type Route = (&'static str, &'static str, u16, Value);

struct Mock {
    url: String,
    log: Arc<Mutex<Vec<Recorded>>>,
}

impl Mock {
    fn start(routes: Vec<Route>) -> Mock {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/v2/", listener.local_addr().unwrap());
        let log = Arc::new(Mutex::new(Vec::new()));
        let log2 = log.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { continue };
                let routes = routes.clone();
                let log = log2.clone();
                std::thread::spawn(move || {
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap_or(0) == 0 {
                        return;
                    }
                    let mut parts = line.split_whitespace();
                    let method = parts.next().unwrap_or("").to_string();
                    let raw_path = parts.next().unwrap_or("");
                    let path = raw_path.trim_start_matches("/v2/").to_string();
                    let mut headers = Vec::new();
                    let mut len = 0usize;
                    loop {
                        let mut h = String::new();
                        reader.read_line(&mut h).unwrap();
                        let h = h.trim_end();
                        if h.is_empty() {
                            break;
                        }
                        if let Some((k, v)) = h.split_once(':') {
                            let (k, v) = (k.trim().to_lowercase(), v.trim().to_string());
                            if k == "content-length" {
                                len = v.parse().unwrap_or(0);
                            }
                            headers.push((k, v));
                        }
                    }
                    let mut buf = vec![0; len];
                    reader.read_exact(&mut buf).unwrap();
                    let body = (len > 0).then(|| serde_json::from_slice(&buf).unwrap());
                    log.lock().unwrap().push(Recorded { method: method.clone(), path: path.clone(), headers, body });

                    let matched = routes.iter().find(|(m, p, _, _)| {
                        if *m != method {
                            return false;
                        }
                        if *p == path {
                            return true;
                        }
                        // Prefix match for query parameters like `users/me?token=...`
                        if p.contains('?') {
                            let base_p = p.split('?').next().unwrap();
                            let base_path = path.split('?').next().unwrap();
                            base_p == base_path
                        } else {
                            false
                        }
                    });

                    let (status, resp) = matched
                        .map(|(_, _, s, b)| (*s, b.clone()))
                        .unwrap_or((404, json!({"code": "not_found", "message": "no route"})));
                    let text = if status == 204 { String::new() } else { resp.to_string() };
                    let _ = write!(
                        stream,
                        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{text}",
                        text.len()
                    );
                });
            }
        });
        Mock { url, log }
    }

    fn requests(&self) -> Vec<Recorded> {
        self.log.lock().unwrap().clone()
    }

    fn last(&self, method: &str) -> Recorded {
        self.requests().into_iter().rev().find(|r| r.method == method).expect("no such request")
    }
}

struct Env {
    dir: PathBuf,
    api: String,
}

impl Env {
    fn new(mock: &Mock) -> Env {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "reddit-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        Env { dir, api: mock.url.clone() }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_reddit"))
            .args(args)
            .env("REDDIT_CONFIG_DIR", &self.dir)
            .env("REDDIT_SECRET_STORE", "plaintext")
            .env("REDDIT_ALLOW_PLAINTEXT_STORE", "1")
            .env("REDDIT_API_URL", &self.api)
            .env_remove("APIFY_TOKEN")
            .env_remove("REDDIT_API_KEY")
            .output()
            .unwrap()
    }

    fn json(&self, args: &[&str]) -> (i32, Value, Value) {
        let mut all = args.to_vec();
        all.push("--json");
        let out = self.run(&all);
        let parse = |b: &[u8]| {
            let s = String::from_utf8_lossy(b);
            let s = s
                .lines()
                .filter(|l| !l.starts_with("warning:") && !l.starts_with("Searching") && !l.starts_with("Scraping"))
                .collect::<Vec<_>>()
                .join("\n");
            serde_json::from_str(&s).unwrap_or(Value::Null)
        };
        (out.status.code().unwrap_or(-1), parse(&out.stdout), parse(&out.stderr))
    }

    fn with_account(self) -> Env {
        let (code, out, err) = self.json(&["accounts", "add", "work", "--api-key", "apify_token_123"]);
        assert_eq!(code, 0, "{err}");
        assert_eq!(out["status"], "added");
        self
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

fn me_route() -> Route {
    (
        "GET",
        "users/me?token=any",
        200,
        json!({
            "data": {
                "id": "usr_test123",
                "username": "redditdev",
                "email": "dev@example.com"
            }
        }),
    )
}

fn scraper_route() -> Route {
    (
        "POST",
        "acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items?token=any",
        200,
        json!([
            {
                "id": "t3_12345",
                "title": "Rust Edition 2024 is awesome",
                "community": "r/rust",
                "author": "ferris",
                "score": 100
            }
        ]),
    )
}

#[test]
fn agent_readme() {
    let mock = Mock::start(vec![]);
    let env = Env::new(&mock);

    let out = env.run(&["agent-readme"]);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("# reddit - agent operating manual"));

    let (code, json_out, _) = env.json(&["agent-readme"]);
    assert_eq!(code, 0);
    assert_eq!(json_out["tool"], "reddit");
    assert_eq!(json_out["apiVersion"], "1.0.0");
    assert!(json_out["rules"].is_array());
    assert_eq!(json_out["exitCodes"]["0"], "ok");
}

#[test]
fn parse_error_envelope() {
    let mock = Mock::start(vec![]);
    let env = Env::new(&mock);

    let (code, _, err) = env.json(&["nonexistent-command"]);
    assert_eq!(code, 6);
    assert_eq!(err["code"], "invalid_input");
}

#[test]
fn accounts_lifecycle() {
    let mock = Mock::start(vec![me_route()]);
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["accounts", "list"]);
    assert_eq!(code, 0);
    assert_eq!(out["count"], 1);
    assert_eq!(out["accounts"][0]["identity"], "redditdev");
    assert_eq!(out["accounts"][0]["keyStatus"], "stored");

    let (_, out, _) = env.json(&["accounts", "list", "--check"]);
    assert_eq!(out["accounts"][0]["keyStatus"], "valid");

    let (code, _, err) = env.json(&["accounts", "add", "WORK", "--api-key", "new_token"]);
    assert_eq!(code, 6);
    assert!(err["remediation"].as_str().unwrap().contains("--force"));

    let (code, out, _) = env.json(&["accounts", "test", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out["keyStatus"], "valid");
    assert_eq!(out["identity"], "redditdev");

    let (code, out, _) = env.json(&["accounts", "remove", "work", "--yes"]);
    assert_eq!(code, 0);
    assert_eq!(out["status"], "removed");

    let (_, out, _) = env.json(&["accounts", "list"]);
    assert_eq!(out["count"], 0);
}

#[test]
fn search_with_api_key_flag() {
    let mock = Mock::start(vec![scraper_route()]);
    let env = Env::new(&mock);

    let (code, out, _) =
        env.json(&["search", "rust", "--api-key", "direct_token_abc", "--community", "rust", "--max-items", "5"]);
    assert_eq!(code, 0);
    assert!(out.is_array());
    assert_eq!(out[0]["title"], "Rust Edition 2024 is awesome");

    let last_post = mock.last("POST");
    assert!(last_post.path.starts_with("acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items"));
    let body = last_post.body.unwrap();
    assert_eq!(body["searches"][0], "rust");
    assert_eq!(body["searchCommunityName"], "rust");
    assert_eq!(body["maxItems"], 5);
}

#[test]
fn search_with_account() {
    let mock = Mock::start(vec![me_route(), scraper_route()]);
    let env = Env::new(&mock).with_account();

    let (code, out, _) = env.json(&["search", "rust", "-a", "work"]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["community"], "r/rust");
}

#[test]
fn scrape_post_url() {
    let mock = Mock::start(vec![scraper_route()]);
    let env = Env::new(&mock);

    let (code, out, _) = env.json(&[
        "scrape",
        "https://www.reddit.com/r/rust/comments/123/hello/",
        "--api-key",
        "tok_123",
        "--skip-comments",
    ]);
    assert_eq!(code, 0);
    assert_eq!(out[0]["author"], "ferris");

    let last_post = mock.last("POST");
    let body = last_post.body.unwrap();
    assert_eq!(body["startUrls"][0]["url"], "https://www.reddit.com/r/rust/comments/123/hello/");
    assert_eq!(body["skipComments"], true);
}

#[test]
fn auth_failure_handling() {
    let mock = Mock::start(vec![(
        "POST",
        "acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items?token=any",
        401,
        json!({"error": {"message": "Invalid token"}}),
    )]);
    let env = Env::new(&mock);

    let (code, _, err) = env.json(&["search", "query", "--api-key", "bad_token"]);
    assert_eq!(code, 3);
    assert_eq!(err["code"], "auth_required");
}
