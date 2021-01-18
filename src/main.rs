#[macro_use]
extern crate tracing;

use rsrs::RsClient;



use rsq::model::config::Config;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    if let Ok(filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    } else {
        tracing_subscriber::fmt().with_env_filter("info").init();
    }

    let config = match envy::from_env::<Config>() {
        Ok(config) => {
            config
        },
        Err(error) => {
            error!(error = ?error, "Please provide a USER to query in the .env file!");
            panic!("No user provided");
        }
    };
    let username = config.username.to_string();
    let client = RsClient::new(Some(username));
    let quests = client.get_quests(None).await?;
    let completed: Vec<String> = client.get_completed_quests(quests).await.iter().map(|q|q.title.to_string()).collect();

    // TODO: How can we better print lists?
    completed.iter().for_each(|x|{
        println!("{}", x);
    });

    Ok(())
}
