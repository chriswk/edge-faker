use chrono::Utc;
use clap::Parser;
use fake::faker::name::en::{FirstName, Name};
use fake::faker::number::en::Digit;
use fake::faker::{internet, lorem};
use fake::locales::EN;
use fake::Fake;
use rand::rngs::ThreadRng;
use rand::{random, thread_rng, Rng};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use unleash_types::client_features::{ClientFeature, ClientFeatures, Constraint, Strategy};

#[derive(Debug, Clone, Parser)]
pub struct CliArgs {
    #[arg(short, long, env, default_value_t = 10000)]
    pub features_count: usize,
    #[arg(short, long, env, default_value_t = 20)]
    pub max_strategies_per_feature: usize,
    #[arg(short, long, env)]
    pub output: String,
}

fn main() {
    let args = CliArgs::parse();

    if args.features_count > 0 {
        let features = generate_features(args.features_count, args.max_strategies_per_feature);
        let output =
            File::create(args.output.clone()).expect(&format!("Failed to create {}", args.output));
        let mut writer = BufWriter::new(output);
        serde_json::to_writer(&mut writer, &features).expect("Failed to write generated data");
    }
}

fn generate_features(feature_count: usize, max_strategies_per_feature: usize) -> ClientFeatures {
    let mut features = Vec::with_capacity(feature_count);
    let mut rng = thread_rng();
    for _ in 0..feature_count {
        features.push(ClientFeature {
            name: FirstName().fake(),
            feature_type: Some("release".into()),
            description: lorem::en::Sentence(3..5).fake(),
            created_at: Some(Utc::now() - chrono::Duration::days(rng.gen_range(0..365))),
            last_seen_at: None,
            enabled: rng.gen_bool(0.9),
            stale: Some(false),
            impression_data: Some(false),
            project: Some("default".into()),
            strategies: generate_strategies(max_strategies_per_feature),
            variants: None,
            dependencies: None,
        });
    }
    ClientFeatures {
        version: 2,
        features,
        segments: None,
        query: None,
    }
}

fn generate_strategies(max_count: usize) -> Option<Vec<Strategy>> {
    let mut rng = thread_rng();
    let strategy_count = rng.gen_range(0..max_count);
    if strategy_count == 0 {
        return None;
    }
    let mut strategies = Vec::with_capacity(strategy_count);
    for _ in 0..strategy_count {
        match rng.gen_range(0..100) {
            0..90 => strategies.push(flexible_rollout(rng.clone())),
            _ => strategies.push(default()),
        }
    }
    Some(strategies)
}

fn flexible_rollout(mut rng: ThreadRng) -> Strategy {
    let mut parameters: HashMap<String, String> = HashMap::new();
    parameters.insert("rollout".into(), rng.gen_range(0..100).to_string());
    Strategy {
        name: "flexibleRollout".into(),
        parameters: Some(parameters),
        sort_order: Some(rng.gen_range(0..10000)),
        segments: None,
        variants: None,
        constraints: None,
    }
}

fn default() -> Strategy {
    Strategy {
        name: "default".into(),
        parameters: None,
        sort_order: Some(0),
        segments: None,
        variants: None,
        constraints: None,
    }
}
