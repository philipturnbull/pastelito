#![feature(ascii_char)]
// We're generating the `model.bin` file that is used in `src/model.rs`. At this
// point, the crate hasn't been built yet but we need access to the definitions.
// We directly include the file here instead.
#[path = "src/data.rs"]
#[allow(unused)]
mod data;

use data::{Feature, Model, Scores, Tag, WeightRange};
use fxhash::FxHashMap;
use serde_json::Value;
use speedy::Writable as _;
use std::path::PathBuf;
use std::str::FromStr as _;
use std::{env, fs::File, io::BufReader, path::Path};

fn input_file(filename: &str) -> BufReader<File> {
    let path = PathBuf::from("prose/internal/model").join(filename);
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

fn input_json(filename: &str) -> Value {
    serde_json::from_reader(input_file(filename)).unwrap()
}

fn output_file_path(filename: &str) -> PathBuf {
    Path::new(&env::var("OUT_DIR").unwrap()).join(filename)
}

fn dserialize_weight_range(value: serde_json::Value, weights: &mut Vec<(Tag, f32)>) -> WeightRange {
    let start = weights.len();

    weights.extend(value.as_object().unwrap().into_iter().map(|(k, v)| {
        let tag = Tag::from_str(k.as_str()).unwrap();
        let weight = v.as_f64().unwrap() as f32;
        (tag, weight)
    }));
    let end = weights.len();

    WeightRange::new(start, end)
}

fn deserialize_weights<O>(mut on_weight_mapping: O) -> Vec<(Tag, f32)>
where
    O: FnMut(String, WeightRange),
{
    let mut weights = Vec::new();

    match input_json("weights.json") {
        Value::Object(map) => {
            map.into_iter().for_each(|(k, v)| {
                let range = dserialize_weight_range(v, &mut weights);
                on_weight_mapping(k, range);
            });
        }
        _ => panic!("Invalid JSON"),
    }

    weights
}

fn read_static_tags() -> FxHashMap<String, Tag> {
    let reader = input_file("tags.json");
    let json: Value = serde_json::from_reader(reader).unwrap();

    let mut tags: Vec<(String, Tag)> = match json {
        Value::Object(tags) => tags
            .into_iter()
            .map(|(k, v)| match v {
                Value::String(v) => (k, Tag::from_str(v.as_str()).unwrap()),
                _ => panic!("tag value is not a string, got {:?}", v),
            })
            .collect(),

        _ => panic!("top-level tag object is not an object, got {:?}", json),
    };

    tags.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());

    FxHashMap::from_iter(tags)
}

fn generate_model() {
    let static_tags = read_static_tags();

    let mut weight_mapping: FxHashMap<Feature, WeightRange> = FxHashMap::default();

    let weights = deserialize_weights(|key, weights| {
        weight_mapping.insert(key.into(), weights);
    });

    let mut initial_scores = Scores::default();
    let bias_range = weight_mapping.get(&Feature::Bias).unwrap();
    let bias_weights = &weights[bias_range.as_range()];
    for (tag, weight) in bias_weights {
        initial_scores.update(*tag, *weight);
    }

    let model = Model::new(static_tags, weights, weight_mapping, initial_scores);

    model.write_to_file(output_file_path("model.bin")).unwrap();
}

fn main() {
    generate_model();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=prose");
}
