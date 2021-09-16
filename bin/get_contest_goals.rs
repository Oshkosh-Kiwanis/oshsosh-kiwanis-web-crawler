
use std::{convert::TryFrom, error::Error};

use chrono::Utc;
use oshkosh_kiwanis_web_crawler::{Contest, ContestData, ContestDataCSV, Contests};

use tokio::time::{interval, Duration};

use nipper::Document;

use log::info;

async fn crawl_site(domain: &str, contest: Contest) -> Result<ContestData, Box<dyn Error>> {
    // navigate to the search page for the contest,
    // this is where we will grab the top tep results
    let url = format!("{}/{}", domain, contest.page);
    info!("getting url; url={:?}", &url);

    // get the webapge html
    let resp  = reqwest::get(&url).await?;
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

    let goal = doc.select("#ContentPlaceHolder_divFundraisingMeter > div.goal > span")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);

    let total_entries = get_entries(&url).await?;

    let champ_day = contest.champ_day;
    let now = Utc::now();

    Ok(ContestData {
        contest,
        raised,
        goal,
        total_entries,
        champ_day,
        timestamp: now.timestamp(),
    })
}

async fn get_entries(contest_url: &str) -> Result<usize, Box<dyn Error>> {
    // get the webapge html
    let entries_url = format!("{}/search", &contest_url);
    let resp  = reqwest::get(entries_url).await?;
    let html = resp.text().await?;

    // now we have to parse that html
    let doc = Document::try_from(&html)?;

    let total_entries = doc.select("#ContentPlaceHolder_divSearchTitle > span.numEntries")
        .text()
        .chars()
        // make sure that we are only dealing with valid numerical
        // representation before trying to parse it
        .filter(|ch| ch.is_digit(10) || *ch == '.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);

    return Ok(total_entries);
}


// lets do some web crawling!
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let domain = "https://www.gogophotocontest.com";

    // Do this every minute!
    let mut interval = interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        info!("tick");

        let mut results: Vec<ContestData> = Vec::new();
        for contest in Contests::get_all() {
            let ret = crawl_site(domain, contest).await?;

            results.push(ret);
        }

        // write the results to a json file
        let serialized = serde_json::to_string(
            &results
        )?;

        let mut csv_wtr = csv::Writer::from_path("contest-goals.csv")?;

        for result in results.iter() {
            let record = ContestDataCSV::from_contest_data(result);
            csv_wtr.serialize(record)?;
        }
        csv_wtr.flush()?;

        std::fs::write("contest-goals.json", serialized)?;
        info!("done");
    }
}
