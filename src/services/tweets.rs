use crate::entities::Tweet;
use crate::repositories::Tweets;
use crate::views::Home;

pub async fn create_tweets(repo: &impl Tweets, message: &str) {
    let new_tweet = Tweet::create(message);
    repo.store(&new_tweet).await;
}

pub async fn list_tweets(repo: &impl Tweets) -> Home {
    let tweets = repo.list()
        .await
        .into_iter()
        .map(|x| x.into())
        .collect();

    Home {
        tweets,
    }
}

pub async fn delete_tweet(repo: &impl Tweets, id: i32) {
    let tweet = repo.find(id).await;
    if let Some(mut tweet) = tweet {
        tweet.delete();
        repo.store(&tweet).await;
    }
}
