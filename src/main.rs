use dotenv::dotenv;
use poise::{
    serenity_prelude::{self as serenity, CreateEmbed},
    CreateReply,
};
use raisehell::{chances_of_hit, how_many_hellraisers, simulate_hellraiser_trigger};
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays probabilities of Hellraisers based on input parameters
#[poise::command(slash_command, prefix_command)]
async fn howmanyhellraisers(
    ctx: Context<'_>,
    gy_size: u32,
    seasons: u32,
    triggers: Option<u32>,
    beacons: Option<u32>,
    flameshapers: Option<u32>,
) -> Result<(), Error> {
    // Unwrap optional values with defaults
    let triggers = triggers.unwrap_or(1);
    let beacons = beacons.unwrap_or(0);
    let flameshapers = flameshapers.unwrap_or(0);

    // Calculate probabilities
    let probabilities = how_many_hellraisers(triggers, gy_size, seasons, beacons, flameshapers);

    // Generate probability fields
    let fields: Vec<(String, String, bool)> = probabilities
        .iter()
        .enumerate()
        .filter(|(_, &prob)| prob != 0.0)
        .map(|(count, &prob)| {
            (
                format!("{} Hellraisers", count + 1), // Field name
                format!("{:.2}%", prob * 100.0),      // Field value as a percentage
                false,                                // Inline field
            )
        })
        .collect();

    // Create the embed with parameters mentioned
    let embed = serenity::CreateEmbed::default()
        .title("Hellraiser Probabilities")
        .description(format!(
            "**Initial Triggers:** {}\n\
            🪦 **Graveyard Size:** {}\n\
            🍂 **Seasons:** {}\n\
            🌟 **Beacons:** {}\n\
            🔥 **Flameshapers:** {}",
            triggers, gy_size, seasons, beacons, flameshapers
        ))
        .fields(fields) // Add probability fields dynamically
        .color(serenity::Colour::DARK_RED);

    // Send the embed
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
/// Simulates a Hellraiser trigger and displays the outcome
#[poise::command(slash_command, prefix_command)]
async fn simulatehellraiser(ctx: Context<'_>, gy_size: u32, hits: u32) -> Result<(), Error> {
    // Call the existing simulation function
    let result = simulate_hellraiser_trigger(hits, gy_size);

    // Create a different embed based on the result
    let embed = if result {
        serenity::CreateEmbed::default()
            .title("Hellraiser Simulation")
            .description("🎯 **You hit!** Hell was raised!")
            .color(serenity::Colour::FOOYOO) // Bright green for a hit
    } else {
        serenity::CreateEmbed::default()
            .title("Hellraiser Simulation")
            .description("😢 **You whiffed.** No hits this time.")
            .color(serenity::Colour::DARK_RED) // Red for a whiff
    };

    // Send the embed
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn chancesofhit(
    ctx: Context<'_>,
    gy_size: u32,
    hits: u32,
    triggers: Option<u32>,
) -> Result<(), Error> {
    let chances = chances_of_hit(hits, gy_size, triggers.unwrap_or(1));
    // Create an embed to display the result
    let embed = serenity::CreateEmbed::default()
        .title("Chances of Hit")
        .description(format!("🎯 **{:.2}%**", chances * 100.0))
        .color(serenity::Colour::DARK_GREEN) // Use a green color to represent the probability
        .fields(vec![
            ("Graveyard Size 🪦".to_string(), gy_size.to_string(), true),
            ("Hits 🎯".to_string(), hits.to_string(), true),
            (
                "Triggers 🔄".to_string(),
                triggers.unwrap_or(1).to_string(),
                true,
            ),
        ]);

    // Send the embed
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![howmanyhellraisers(), simulatehellraiser(), chancesofhit()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
