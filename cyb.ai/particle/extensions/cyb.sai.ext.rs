/*
    This file provides smart particle processing, custom soul-resolver logic,
    content moderation, and dynamic cyber domain features for your particle page on cyb.ai.
    All business logic and exclusions comply with community standards and privacy frameworks.
*/

pub async fn moon_domain_resolver() {
    // Get nickname of domain resolver at the moment
    let nickname = cyb::context.user.nickname;

    let rng = rand::WyRand::new();
    let rand_int = rng.int_range(0, 999999);

    return content_result(format!("Hello, {}, your lucky number is {} 🎉", nickname, rand_int));
    // To substitute with a CID for IPFS-hosted app:
    // return cid_result("QmcqikiVZJLmum6QRDH7kmLSUuvoPvNiDnCKY4A5nuRw17")
}

pub async fn ask_companion(cid, content_type, content) {
    // Display plain text or links for companion page
    let mut links = vec![meta_text("similar: ")];
    let mut rows = vec![links.clone()];

    // Search 5 closest particles by local embedding
    let similar_results = cyb::search_by_embedding(content, 5).await;
    for v in &similar_results {
        links.push(meta_link(format!("/oracle/ask/{}", v.cid), &v.text));
    }

    if links.len() <= 1 {
        links = vec![meta_text("no similar particles found")];
    }

    let secrets = &cyb::context.secrets;
    if let Some(api_key) = secrets.get("open_ai_key") {
        let messages = [
            json!({
                "role": "system",
                "content": "You should give description or summary of any content. answer should not exceed 32 words"
            }),
            json!({
                "role": "user",
                "content": content
            })
        ];

        let inference = cyb::open_ai_completions(&messages, api_key, json!({"model": "gpt-3.5-turbo"})).await;
        rows.push(vec![meta_text(format!("inference: {}", inference))]);
    }

    content_result(rows)
}

pub async fn personal_processor(cid, content_type, mut content) {
    // Only process text content
    if content_type != "text" {
        return pass();
    }

    // <citizen_name>.moon domain resolver
    if content.ends_with(".moon") {
        let items: Vec<_> = content.split('.').collect();
        let username = items[0];
        let ext = items[1];

        if username.len() <= 14 && ext == "moon" {
            let passport = cyb::get_passport_by_nickname(username).await;
            let particle_cid = passport["extension"]["particle"];

            cyb::log(format!("Resolve {} domain from passport particle '{}'", username, particle_cid));
            let result = cyb::eval_script_from_ipfs(particle_cid, "moon_domain_resolver", &[]).await;
            return result;
        }
    }

    // Example: content exclusion
    let buzz_word = "пиздопроебанное хуеплетство";
    if content.contains(buzz_word) {
        cyb::log(format!("Hide {} item because of '{}' in the content", cid, buzz_word));
        return hide();
    }

    // Example: content modification (add heart to "cyber")
    let highlight_text = "cyber ";
    let highlight_with = "❤ ";
    if content.contains(highlight_text) {
        cyb::log(format!("Update {} content, highlight {}{}", cid, highlight_text, highlight_with));
        content = content.replace(highlight_text, &format!("{}{}", highlight_text, highlight_with));
        return content_result(content);
    }

    // Example: token price resolver, replaces <token_name>@NOW
    if content.contains("@NOW") {
        let left_part = content.split("@NOW").next().unwrap();
        let token_name = left_part.split(' ').rev().next().unwrap();
        let vs_currency = "usd";

        // External API call for current token price
        let json = http::get(
            &format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}", token_name, vs_currency)
        )
        .await?
        .json()
        .await?;
        return content_result(content.replace(
            &format!("{}@NOW", token_name),
            &format!("Current {} price is {} {}", token_name, json[token_name][vs_currency], vs_currency),
        ));
    }

    // Filter vulgar language
    content = content.replace("хуй", "🌽").replace("хуя", "🌽").replace("хуе", "🌽");
    content_result(content)
}
