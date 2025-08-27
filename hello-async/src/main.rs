use trpl::Html;
use std::env;

async fn page_title(url: &str) -> Option<String> {
    let text = trpl::get(url).await.text().await;

    Html::parse(&text)
        .select_first("title")
        .map(|t| t.inner_html())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} is {title}"),
            None => println!("{url} has no title")
        }
    })


}
