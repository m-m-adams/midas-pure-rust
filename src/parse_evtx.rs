use evtx::err::EvtxError;
use evtx::{EvtxParser, ParserSettings, SerializedEvtxRecord};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
pub struct Login {
    pub workstation: String,
    pub user: String,
    pub time: i64,
}

impl Login {
    pub fn to_scoreable(&self) -> ((u64, u64), u64) {
        (
            (
                Login::my_hash(&self.user),
                Login::my_hash(&self.workstation),
            ),
            self.time as u64,
        )
    }
    fn my_hash<T>(obj: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        hasher.finish()
    }
}

pub fn read(evtx: &str) -> Vec<Login> {
    // Change this to a path of your .evtx sample.
    let fp = PathBuf::from(evtx);
    let settings = ParserSettings::default().num_threads(0);
    EvtxParser::from_path(fp)
        .unwrap()
        .with_configuration(settings)
        .records_json_value()
        .filter_map(rec_to_login)
        .collect()
}

fn rec_to_login(
    event: std::result::Result<SerializedEvtxRecord<serde_json::value::Value>, EvtxError>,
) -> Option<Login> {
    let r = event.unwrap();
    let time = r.timestamp.timestamp();
    if let Some(user) = r.data["Event"]["EventData"]["TargetUserName"].as_str() {
        if user.ends_with("$") {
            return None;
        }
        if let Some(ws) = r.data["Event"]["EventData"]["WorkstationName"].as_str() {
            return Some(Login {
                workstation: ws.to_string(),
                time: time,
                user: user.to_string(),
            });
        }
    }
    None
}
