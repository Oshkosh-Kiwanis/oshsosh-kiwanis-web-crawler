
use std::{convert::TryFrom, error::Error};

use oshkosh_kiwanis_web_crawler::{ContestGoal, Contest, Contests};

use tokio::time::{interval, Duration};

use nipper::Document;

async fn crawl_site(domain: &str, contest: Contest) -> Result<ContestGoal, Box<dyn Error>> {
    // navigate to the search page for the contest,
    // this is where we will grab the top tep results
    let url = format!("{}/{}", domain, contest.page);
    println!("[INFO] getting url; url={:?}", url);

    // get the webapge html
    let resp  = reqwest::get(url).await?;
    let html = resp.text().await?;

    // now we have to parse that html
    let doc = Document::try_from(&html)?;

    let raised = doc.select("#ContentPlaceHolder_divFundraisingMeter > div.raised > span")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);

    let goal= doc.select("#ContentPlaceHolder_divFundraisingMeter > div.goal > span")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);

    Ok(ContestGoal {
        contest,
        raised,
        goal,
        match_day: 0,
    })
}


// lets do some web crawling!
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let domain = "https://www.gogophotocontest.com";

    // Do this every minute!
    let mut interval = interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        println!("[TICK]");

        let mut results: Vec<ContestGoal> = Vec::new();
        for contest in Contests::get_all() {
            let ret = crawl_site(domain, contest).await?;

            results.push(ret);
        }

        // write the results to a json file
        let serialized = serde_json::to_string(
            &results
        )?;

        std::fs::write("contest-goals.json", serialized)?;
        println!("[DONE]");
    }
}
