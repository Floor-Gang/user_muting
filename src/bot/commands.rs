use serenity::{
    prelude::*,
    framework::standard::{
        Args,
        CommandResult,
        macros::{
            command, group
        }
    },
    model::{
        channel::Message,
        id::UserId
    }
};
use crate::bot::utils::{reply, filter_tag};
use crate::bot::DataBase;
use crate::config::Config;
use regex::Regex;
use chrono::Duration;

#[group()]
#[commands(ping, db_test, prefix, mute)]
pub struct Commands;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    reply(&ctx, &msg, &String::from("Pong!")).await;
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let config = data.get::<Config>().unwrap();

    if let Err(why) = msg.channel_id.send_message(&ctx.http,  |m| {
        m.embed(|embed| {
            embed.title("Prefix");
            embed.description(format!("My prefix is: `{}`", &config.prefix));
            embed.color(0xffa500)
        });
        m

    }).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    };

    Ok(())
}

#[command]
async fn db_test(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<DataBase>().unwrap();

    let rows = db.query("SELECT test FROM test", &[]).await.unwrap();

    reply(&ctx, &msg, &rows[0].get(0)).await;
    Ok(())
}

#[command]
async fn mute(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // if arg is empty reply it to channel
    if args.is_empty() {
        if let Err(why) = msg.channel_id.send_message(&ctx.http,  |m| {
            m.embed(|embed| {
                embed.title("mute");
                embed.description(format!("-mute [userID] [time(optional)] [reason]"));
                embed.color(0xffa500)
            });
            m

        }).await {
            println!("Failed to send message in #{} because\n{:?}",
                     msg.channel_id, why
            );
        };
    }

    // Parse first argument
    let user = args.single::<String>().unwrap_or(String::from(""));

    if user == "" {
        println!("User not provided!");
        return Ok(());
    }

    // get the guild
    if let Some(guild) = msg.guild(&ctx.cache).await {
        // Get the user ID from provided tag
        let uid = UserId(filter_tag(&user.to_owned()));

        if uid == 0 {
            println!("Something horrible happened!");
            return Ok(());
        }

        // Get the member with the given user ID
        let member = {
            match ctx.cache.member(guild.id, uid).await {
                Some(member) => member,
                None => {
                    println!("Couldn't find user with id: {}", uid);
                    return Ok(());
                },
            }
        };


        // Parse second argument
        let mut time = args.single::<String>().unwrap_or(String::from(""));

        if time != "" {
            // var used for timeout duration
            let mut sleep_count = Duration::seconds(0);

            // find the time duration
            let m = Regex::new("[0-9]{1,}").unwrap();
            let time_dur_borders = m.find(&time);

            // check if we found a time duration
            if time_dur_borders.is_some() {
                let time_duration = &time[time_dur_borders.unwrap().start()..
                                          time_dur_borders.unwrap().end()];

                let i64_time_val = time_duration.parse::<i64>().unwrap_or(0);

                // find the time unit
                let tu = Regex::new("[A-Za-z]{1,}").unwrap();
                let time_unit_borders = tu.find(&time);

                // check if we found a time unit and if it is valid
                if time_unit_borders.is_some() &&
                   (time_unit_borders.unwrap().end() == time.len()) {

                    let time_unit = &time[time_unit_borders.unwrap().start()..
                                        time_unit_borders.unwrap().end()];

                    // Calculate the required timeout value
                    match time_unit {
                        "s" | "sec"  => sleep_count = Duration::seconds(i64_time_val),
                        "m" | "min"   => sleep_count = Duration::minutes(i64_time_val),
                        "h" | "hour" => sleep_count = Duration::hours(i64_time_val),
                        "d" | "day" => sleep_count = Duration::days(i64_time_val),
                        "w" | "week" => sleep_count = Duration::weeks(i64_time_val),
                        _ => {
                            println!("Wrong time format!");
                            time = String::from("");
                            args.rewind();
                        }
                    }
                } else {
                    // wrong time format, rewind the arguments
                    println!("Wrong time format provided!");
                    time = String::from("");
                    args.rewind();
                }
            } else {
                // wrong time format, rewind the arguments
                println!("Time was not provided!");
                time = String::from("");
                args.rewind();
            }

        }

        // Parse the input arguments to get the reason variable. If time is not
        // provided reason starts from second argument.
        let mut reason = String::from("");

        if (args.len() > 2) || (time == "") {
            for arg in args.iter::<String>() {
                reason = reason + &format!(" {}", arg.unwrap_or(String::from("")));
            }
            reason = reason.trim().to_string();
        } else {
            println!("Reason not provided!");
            return Ok(());
        }

        // if time not provided we assume user is muted indefinitely
        if time == "" {
            time = String::from("indefinitely");
        }

        // Reply to the channel from where it was sent.
        reply(&ctx, &msg, &String::from(format!("Muted user {} for {}! Reason: {}",
                                        user, time, reason))).await;
    }

    Ok(())
}

//TODO
/*
#[command]
async fn unmute(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        if let Err(why) = msg.channel_id.send_message(&ctx.http,  |m| {
            m.embed(|embed| {
                embed.title("unmute");
                embed.description(format!("-mute [userID]"));
                embed.color(0xffa500)
            });
            m

        }).await {
            println!("Failed to send message in #{} because\n{:?}",
                     msg.channel_id, why
            );
        };
    }
    Ok(())
}*/