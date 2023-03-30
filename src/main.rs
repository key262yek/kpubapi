use reqwest;

#[tokio::main]
async fn main() {
    // key : "5a645970696b657939336a48426b7a"

    let res = reqwest::get("http://swopenapi.seoul.go.kr/api/subway/5a645970696b657939336a48426b7a/json/realtimePosition/0/5/1호선")
        .await.unwrap()
        .text().await.unwrap();
    println!("{:?}", res);
}
