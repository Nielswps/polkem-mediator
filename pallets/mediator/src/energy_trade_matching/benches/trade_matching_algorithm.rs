use std::io::Write;
use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use chrono::Local;

use energy_trade_matching::energy_trade_matching::*;

fn one_asset_per_mg(c: &mut Criterion) {
    let min_req: u16 = 182;
    let max_req: u16 = 514;
    let mut rng = rand::thread_rng();
    let mut requests: Vec<EnergyRequest> = (0..406).map(|_|
        energy_request_factory(
            rng.gen_range(min_req..max_req),
            rng.gen_range(1.8..3.5)))
        .collect();

    let min_off: u16 = 6_000;
    let max_off: u16 = 25_000;
    let mut offers: Vec<EnergyOffer> = (0..406).map(|_|
        energy_offer_factory(
            rng.gen_range(min_off..max_off),
            rng.gen_range(2.0..4.0)))
        .collect();

    let grid_request = EnergyRequest { amount: 20, price: 1.5, buyer: String::from("Grid_Buyer") };
    let grid_offer = EnergyOffer { amount: 20, price: 2.6, seller: String::from("Grid_Seller") };

    c.bench_function("Trade matching algorithm", |b| b.iter(|| generate_trades(
        black_box(&mut requests),
        black_box(&mut offers),
        black_box(&grid_request),
        black_box(&grid_offer))));


    let reqs_json = serde_json::to_string(&requests).unwrap();
    let mut file = std::fs::File::create(format!("/tmp/requests-{}.json", Local::now().format("%Y-%m-%d@%H:%M:%S").to_string())).unwrap();
    file.write_all(reqs_json.as_bytes()).unwrap();

    let offs_json = serde_json::to_string(&offers).unwrap();
    let mut file = std::fs::File::create(format!("/tmp/offers-{}.json", Local::now().format("%Y-%m-%d@%H:%M:%S").to_string())).unwrap();
    file.write_all(offs_json.as_bytes()).unwrap();
}

fn ten_assets_per_mg(c: &mut Criterion) {
    let min_req: u16 = 182;
    let max_req: u16 = 514;
    let mut rng = rand::thread_rng();
    let mut requests: Vec<EnergyRequest> = (0..4_060).map(|_|
        energy_request_factory(
            rng.gen_range(min_req..max_req),
            rng.gen_range(1.8..3.5)))
        .collect();

    let min_off: u16 = 6_000;
    let max_off: u16 = 25_000;
    let mut offers: Vec<EnergyOffer> = (0..4_060).map(|_|
        energy_offer_factory(
            rng.gen_range(min_off..max_off),
            rng.gen_range(2.0..4.0)))
        .collect();

    let grid_request = EnergyRequest { amount: 20, price: 1.5, buyer: String::from("Grid_Buyer") };
    let grid_offer = EnergyOffer { amount: 20, price: 2.6, seller: String::from("Grid_Seller") };

    c.bench_function("Trade matching algorithm", |b| b.iter(|| generate_trades(
        black_box(&mut requests),
        black_box(&mut offers),
        black_box(&grid_request),
        black_box(&grid_offer))));
}

fn every_household_requests(c: &mut Criterion) {
    let min_req: u16 = 182;
    let max_req: u16 = 514;
    let mut rng = rand::thread_rng();
    let mut requests: Vec<EnergyRequest> = (0..2_800_000).map(|_|
        energy_request_factory(
            rng.gen_range(min_req..max_req),
            rng.gen_range(1.8..3.5)))
        .collect();

    let min_off: u16 = 6_000;
    let max_off: u16 = 25_000;
    let mut offers: Vec<EnergyOffer> = vec![energy_offer_factory(
        rng.gen_range(min_off..max_off),
        rng.gen_range(2.0..4.0))
    ];

    let grid_request = EnergyRequest { amount: 20, price: 1.5, buyer: String::from("Grid_Buyer") };
    let grid_offer = EnergyOffer { amount: 20, price: 2.6, seller: String::from("Grid_Seller") };

    c.bench_function("Trade matching algorithm", |b| b.iter(|| generate_trades(
        black_box(&mut requests),
        black_box(&mut offers),
        black_box(&grid_request),
        black_box(&grid_offer))));
}

fn every_household_offers(c: &mut Criterion) {
    let min_req: u16 = 182;
    let max_req: u16 = 514;
    let mut rng = rand::thread_rng();
    let mut requests: Vec<EnergyRequest> = vec![energy_request_factory(
        rng.gen_range(min_req..max_req),
        rng.gen_range(2.0..4.0))
    ];

    let min_off: u16 = 6_000;
    let max_off: u16 = 25_000;
    let mut offers: Vec<EnergyOffer> = (0..2_800_000).map(|_|
        energy_offer_factory(
            rng.gen_range(min_off..max_off),
            rng.gen_range(2.0..4.0)))
        .collect();

    let grid_request = EnergyRequest { amount: 20, price: 1.5, buyer: String::from("Grid_Buyer") };
    let grid_offer = EnergyOffer { amount: 20, price: 2.6, seller: String::from("Grid_Seller") };

    c.bench_function("Trade matching algorithm", |b| b.iter(|| generate_trades(
        black_box(&mut requests),
        black_box(&mut offers),
        black_box(&grid_request),
        black_box(&grid_offer))));
}

fn large_overestimation_of_assets(c: &mut Criterion) {
    let min_req: u16 = 182;
    let max_req: u16 = 514;
    let mut rng = rand::thread_rng();
    let mut requests: Vec<EnergyRequest> = (0..2_000_000).map(|_|
        energy_request_factory(
            rng.gen_range(min_req..max_req),
            rng.gen_range(1.8..3.5)))
        .collect();

    let min_off: u16 = 6_000;
    let max_off: u16 = 25_000;
    let mut offers: Vec<EnergyOffer> = (0..2_000_000).map(|_|
        energy_offer_factory(
            rng.gen_range(min_off..max_off),
            rng.gen_range(2.0..4.0)))
        .collect();

    let grid_request = EnergyRequest { amount: 20, price: 1.5, buyer: String::from("Grid_Buyer") };
    let grid_offer = EnergyOffer { amount: 20, price: 2.6, seller: String::from("Grid_Seller") };

    c.bench_function("Trade matching algorithm", |b| b.iter(|| generate_trades(
        black_box(&mut requests),
        black_box(&mut offers),
        black_box(&grid_request),
        black_box(&grid_offer))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::new(60, 0)).sample_size(30).confidence_level(0.98);
    targets = one_asset_per_mg, ten_assets_per_mg, every_household_requests, every_household_offers, large_overestimation_of_assets
}
criterion_main!(benches);

fn energy_request_factory(amount: u16, price: f32) -> EnergyRequest {
    EnergyRequest { amount, price, buyer: String::from("mg_buyer") }
}

fn energy_offer_factory(amount: u16, price: f32) -> EnergyOffer {
    EnergyOffer { amount, price, seller: String::from("mg_seller") }
}