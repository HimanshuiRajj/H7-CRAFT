//! Trim Segment Anywhere removes a picked middle segment without a cutting boundary.
use cadcraft_engine::Session;
use cadcraft_engine::doc::{EntityKind, Handle};
use serde_json::{Value, json};

fn handle(r: &Value) -> String {
    r["handle"].as_str().or_else(|| r["handles"][0].as_str()).unwrap().to_string()
}

fn line(s: &mut Session, a: [f64; 2], b: [f64; 2]) -> String {
    s.execute("line", &json!({"points": [a, b]})).unwrap()["handles"][0].as_str().unwrap().to_string()
}

#[test]
fn trims_arbitrary_middle_of_line_without_cutting_edges() {
    let mut s = Session::new();
    let original = line(&mut s, [0.0, 0.0], [10.0, 0.0]);
    let r = s.execute("trimsegment", &json!({"handle": original, "p1": [3.0, 0.0], "p2": [7.0, 0.0]})).unwrap();
    assert_eq!(r["removed_segment"], true);
    assert_eq!(r["handles"].as_array().unwrap().len(), 2);
    assert!(s.doc().unwrap().entity(Handle::parse_hex(&original).unwrap()).is_none());
    let segments: Vec<_> = s.doc().unwrap().model.iter().filter_map(|e| match &e.kind {
        EntityKind::Line(l) => Some((l.a.xy().x.min(l.b.xy().x), l.a.xy().x.max(l.b.xy().x))),
        _ => None,
    }).collect();
    assert!(segments.iter().any(|(a,b)| (*a - 0.0).abs() < 1e-9 && (*b - 3.0).abs() < 1e-9));
    assert!(segments.iter().any(|(a,b)| (*a - 7.0).abs() < 1e-9 && (*b - 10.0).abs() < 1e-9));
    let trimmed = format!("{:?}", s.doc().unwrap().model.iter().map(|e| e.kind.clone()).collect::<Vec<_>>());
    s.execute("undo", &json!({})).unwrap();
    assert_eq!(s.doc().unwrap().model.len(), 1);
    s.execute("redo", &json!({})).unwrap();
    assert_eq!(format!("{:?}", s.doc().unwrap().model.iter().map(|e| e.kind.clone()).collect::<Vec<_>>()), trimmed);
}

#[test]
fn trim_segment_rejects_points_off_entity_without_mutating_drawing() {
    let mut s = Session::new();
    let original = line(&mut s, [0.0, 0.0], [10.0, 0.0]);
    assert!(s.execute("trimsegment", &json!({"handle": original, "p1": [3.0, 1.0], "p2": [7.0, 0.0]})).is_err());
    assert_eq!(s.doc().unwrap().model.len(), 1);
    assert!(s.doc().unwrap().entity(Handle::parse_hex(&original).unwrap()).is_some());
}

#[test]
fn trim_segment_respects_locked_layers() {
    let mut s = Session::new();
    let original = line(&mut s, [0.0, 0.0], [10.0, 0.0]);
    s.execute("layer.set", &json!({"name": "0", "locked": true})).unwrap();
    assert!(s.execute("trimsegment", &json!({"handle": original, "p1": [3.0, 0.0], "p2": [7.0, 0.0]})).is_err());
    assert_eq!(s.doc().unwrap().model.len(), 1);
}
