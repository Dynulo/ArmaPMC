use std::collections::HashMap;

pub fn internal_get_balance(player: u64) -> Result<i64, String> {
    if crate::TOKEN.read().unwrap().is_empty() {
        return Err(String::from("Empty token"));
    }
    match reqwest::blocking::Client::new()
        .get(&format!("{}/v2/bank/balance/{}", *crate::HOST, player))
        .header("x-dynulo-guild-token", &*crate::TOKEN.read().unwrap())
        .send()
        .unwrap()
        .json::<i64>()
    {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

pub fn internal_save(player: u64, nickname: String) -> Result<(), String> {
    if crate::TOKEN.read().unwrap().is_empty() {
        return Err(String::from("Empty token"));
    }
    let mut map = HashMap::new();
    map.insert("nickname", nickname);
    match reqwest::blocking::Client::new()
        .post(&format!("{}/v2/players/{}/nickname", *crate::HOST, player))
        .header("x-dynulo-guild-token", &*crate::TOKEN.read().unwrap())
        .json(&map)
        .send()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}