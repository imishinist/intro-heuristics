use crate::{Point, State};
use std::thread;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct DrawPoint {
    label: usize,
    x: usize,
    y: usize,
}

impl From<&Point> for DrawPoint {
    fn from(point: &Point) -> Self {
        DrawPoint {
            label: point.id.get(),
            x: point.x,
            y: point.y,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum PayloadType {
    #[serde(rename = "draw")]
    Draw,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    target: usize,
    score: usize,
    r#type: PayloadType,
    points: Vec<DrawPoint>,
}

pub fn draw(state: &State, pos: usize, sleep_dur: Duration) {
    const URL: &str = "http://localhost:8888/json/publish";

    let points = state.points.iter().map(|p| p.into()).collect::<Vec<_>>();

    let score = state.compute_score() as usize;
    let payload = Payload {
        target: pos,
        score,
        r#type: PayloadType::Draw,
        points,
    };

    let res = Client::new()
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send();
    match res {
        Err(e) => log::debug!("{:?}", e),
        Ok(res) => log::info!("{:?}", res),
    }

    thread::sleep(sleep_dur);
}

#[cfg(test)]
mod test {
    use crate::visualizer::{Payload, PayloadType};
    use crate::Point;
    use serde_json::json;

    #[test]
    fn payload_serialize_test() {
        let payload = Payload {
            target: 1,
            score: 10,
            r#type: PayloadType::Draw,
            points: vec![(&Point::new(1, 0, 0)).into()],
        };

        assert_eq!(json!(payload).to_string(), "{\"points\":[{\"label\":1,\"x\":0,\"y\":0}],\"score\":10,\"target\":1,\"type\":\"draw\"}");
    }
}
