use chrono::Local;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::{io::Write, time::Duration};

use energy_trade_matching::energy_trade_matching::*;

fn one_asset_per_mg(c: &mut Criterion) {
	let (mut requests, mut offers) = generate_requests_and_offers(944, 944);
	let (grid_request, grid_offer) = generate_grid_request_and_offer();

	c.bench_function("One asset for each of the 1,888 proposed MG", |b| {
		b.iter(|| {
			generate_trades(
				black_box(&mut requests),
				black_box(&mut offers),
				black_box(&grid_request),
				black_box(&grid_offer),
			)
		})
	});

	let reqs_json = serde_json::to_string(&requests).unwrap();
	let mut file = std::fs::File::create(format!(
		"requests-{}.json",
		Local::now().format("%Y-%m-%d@%H:%M:%S").to_string()
	))
	.unwrap();
	file.write_all(reqs_json.as_bytes()).unwrap();

	let offs_json = serde_json::to_string(&offers).unwrap();
	let mut file = std::fs::File::create(format!(
		"offers-{}.json",
		Local::now().format("%Y-%m-%d@%H:%M:%S").to_string()
	))
	.unwrap();
	file.write_all(offs_json.as_bytes()).unwrap();
}

fn ten_assets_per_mg(c: &mut Criterion) {
	let (mut requests, mut offers) = generate_requests_and_offers(9_440, 9_440);
	let (grid_request, grid_offer) = generate_grid_request_and_offer();

	c.bench_function("Ten assets for each of the 18,880 proposed MG", |b| {
		b.iter(|| {
			generate_trades(
				black_box(&mut requests),
				black_box(&mut offers),
				black_box(&grid_request),
				black_box(&grid_offer),
			)
		})
	});
}

fn block_transaction_limit_of_9_850(c: &mut Criterion) {
	let (mut requests, mut offers) = generate_requests_and_offers(4_925, 4_925);
	let (grid_request, grid_offer) = generate_grid_request_and_offer();

	c.bench_function("Block transaction limit (9,850 energy assets)", |b| {
		b.iter(|| {
			generate_trades(
				black_box(&mut requests),
				black_box(&mut offers),
				black_box(&grid_request),
				black_box(&grid_offer),
			)
		})
	});
}

fn large_overestimation(c: &mut Criterion) {
	let (mut requests, mut offers) = generate_requests_and_offers(2_000_000, 2_000_000);
	let (grid_request, grid_offer) = generate_grid_request_and_offer();

	c.bench_function("Large over-approximation (4,000,00 energy assets)", |b| {
		b.iter(|| {
			generate_trades(
				black_box(&mut requests),
				black_box(&mut offers),
				black_box(&grid_request),
				black_box(&grid_offer),
			)
		})
	});
}

criterion_group! {
	name = benches;
	config = Criterion::default().measurement_time(Duration::new(60, 0)).sample_size(30).confidence_level(0.98);
	targets = one_asset_per_mg, ten_assets_per_mg, block_transaction_limit_of_9_850, large_overestimation
}

criterion_main!(benches);

fn generate_requests_and_offers(
	req_count: u32,
	off_count: u32,
) -> (Vec<EnergyRequest>, Vec<EnergyOffer>) {
	let min: u16 = 200;
	let max: u16 = 1000;

	let mut rng = rand::thread_rng();
	let mut requests: Vec<EnergyRequest> = (0..req_count)
		.map(|_| energy_request_factory(rng.gen_range(min..max), rng.gen_range(1.8..3.5)))
		.collect();

	let mut offers: Vec<EnergyOffer> = (0..off_count)
		.map(|_| energy_offer_factory(rng.gen_range(min..max), rng.gen_range(2.0..4.0)))
		.collect();

	(requests, offers)
}

fn generate_grid_request_and_offer() -> (EnergyRequest, EnergyOffer) {
	let grid_request = EnergyRequest { amount: 20, price: 1.5, buyer: String::from("Grid_Buyer") };
	let grid_offer = EnergyOffer { amount: 20, price: 2.6, seller: String::from("Grid_Seller") };
	(grid_request, grid_offer)
}

fn energy_request_factory(amount: u16, price: f32) -> EnergyRequest {
	EnergyRequest { amount, price, buyer: String::from("mg_buyer") }
}

fn energy_offer_factory(amount: u16, price: f32) -> EnergyOffer {
	EnergyOffer { amount, price, seller: String::from("mg_seller") }
}
