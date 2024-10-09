use fxhash::FxHashMap;
use pastelito_data::{Model, Scores, WeightRange, POS};
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

fn dserialize_weight_range(value: serde_json::Value, weights: &mut Vec<(POS, f32)>) -> WeightRange {
    let start = weights.len();

    weights.extend(value.as_object().unwrap().into_iter().map(|(k, v)| {
        let pos = POS::from_str(k.as_str()).unwrap();
        let weight = v.as_f64().unwrap() as f32;
        (pos, weight)
    }));
    let end = weights.len();

    WeightRange::new(start, end)
}

fn deserialize_weights<O>(mut on_weight_mapping: O) -> Vec<(POS, f32)>
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

fn read_static_tags() -> FxHashMap<String, POS> {
    let reader = input_file("tags.json");
    let json: Value = serde_json::from_reader(reader).unwrap();

    let mut tags: Vec<(String, POS)> = match json {
        Value::Object(tags) => tags
            .into_iter()
            .map(|(k, v)| match v {
                Value::String(v) => (k, POS::from_str(v.as_str()).unwrap()),
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

    let mut weight_mapping: FxHashMap<String, WeightRange> = FxHashMap::default();

    let weights = deserialize_weights(|key, weights| {
        weight_mapping.insert(key, weights);
    });

    let mut initial_scores = Scores::default();
    let bias_range = weight_mapping.get("bias").unwrap();
    let bias_weights = &weights[bias_range.as_range()];
    for (pos, weight) in bias_weights {
        initial_scores.update(*pos, *weight);
    }

    let model = Model::new(static_tags, weights, weight_mapping, initial_scores);

    model.write_to_file(output_file_path("model.bin")).unwrap();
}

fn main() {
    generate_model();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=prose");
}
