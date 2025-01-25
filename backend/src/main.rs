use rand::Rng;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Post {
    user_id: i32,
    id: i32,
    title: String,
    body: String,
}

fn random_numbers(len: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen_range(0..1000)).collect()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let randoms = random_numbers(5);
    println!("Random numbers: {:?}", randoms);

    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let resp = reqwest::get(url).await?;
    let post: Post = resp.json().await?;
    println!(
        "Fetched post: {} {} {} {}",
        post.user_id, post.id, post.title, post.body
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_random_numbers_length() {
        let nums = random_numbers(10);
        assert_eq!(nums.len(), 10);
    }

    #[tokio::test]
    async fn test_fetch_post() {
        let url = "https://jsonplaceholder.typicode.com/posts/1";
        let resp = reqwest::get(url).await.unwrap();
        let post: Value = resp.json().await.unwrap();
        assert_eq!(post["id"], 1, "Expected post with ID=1");
    }
}
