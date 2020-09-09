use serenity::prelude::Context;
use serenity::model::channel::Message;
use regex::Regex;

pub(crate) async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("Failed to send message in #{} because\n{:?}",
                 msg.channel_id, why
        );
    }
}

// FilterTag takes intakes a ID formatted by the Discord client
// ie <#ID>, <:ID>, or <@ID> and returns ID.
pub(crate) fn filter_tag(input_tag: &String) -> u64 {
    let type_tag = &input_tag[1..2];
    let m;
    let mut tag = "";

	match type_tag {
	    // A channel mention
	    "#" => m = Regex::new("#(.*?)>").unwrap(),
	    // An emoji
	    ":" => m = Regex::new(":(.*?)>").unwrap(),
	// A role or a user mention
	    "@" => {
            if &input_tag[2..3] == "&" {
                m = Regex::new("&(.*?)>").unwrap();
            } else {
                m = Regex::new("!(.*?)>").unwrap();
            }
        },
        _ => return input_tag.parse::<u64>().unwrap_or(0)
	}

    let find_str = m.find(input_tag).unwrap();

	if find_str.start() > 0 {
		// Remove the last character, which is ">"
		tag = &input_tag[..find_str.end()-1];

		// If it's not an emoji, because we need the :emoji:<id>
		if type_tag != ":" {
			// Remove the first character, which is ["!", "&", "#"]
			tag = &tag[find_str.start()+1..];
		} else {
            tag = &tag[1..];
        }
    }

	tag.parse::<u64>().unwrap_or(0)
}