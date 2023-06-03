pub mod energy_trade_matching {
    use std::cmp::Ordering;
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, Div};
    use serde::{Serialize, Deserialize};

    pub fn generate_trades(energy_requests: &mut Vec<EnergyRequest>,
                           energy_offers: &mut Vec<EnergyOffer>,
                           grid_request: &EnergyRequest,
                           grid_offer: &EnergyOffer)
                           -> Vec<Trade> {
        let mut trades = Vec::<Trade>::new();

        // Sort requests and offers amd turn to iters
        energy_requests.sort_by(|e1, e2| e1.price.partial_cmp(&e2.price).unwrap());
        energy_offers.sort_by(|e1, e2| e1.price.partial_cmp(&e2.price).unwrap());
        let mut request_iter = energy_requests.iter();
        let mut offer_iter = energy_offers.iter();

        // Get first request and offer
        let mut req = request_iter.next();
        let mut off = offer_iter.next();

        // Create variables to store temporary surplus requests and offers
        let mut surplus_req: EnergyRequest;
        let mut surplus_off: EnergyOffer;

        // Match trades
        while req.is_some() && off.is_some() {
            match (req, off) {
                (Some(r), Some(o)) => {
                    let t;

                    if r.price >= o.price {
                        let average_price = r.price.add(o.price).div(2.0);

                        // Create match for request and offer, and handle potential surplus
                        match r.amount.partial_cmp(&o.amount).expect("Both values are numbers") {
                            Ordering::Less => {
                                // The offered amount exceeds whats requested
                                t = Trade { amount: r.amount, price: average_price, buyer: r.buyer.clone(), seller: o.seller.clone() };
                                req = request_iter.next();

                                surplus_off = EnergyOffer { amount: o.amount.clone() - r.amount.clone(), price: o.price.clone(), seller: o.seller.clone() };
                                off = Some(&surplus_off);
                            }
                            Ordering::Equal => {
                                // The same amount is requested and offered
                                t = Trade { amount: o.amount, price: average_price, buyer: r.buyer.clone(), seller: o.seller.clone() };
                                req = request_iter.next();
                                off = offer_iter.next();
                            }
                            Ordering::Greater => {
                                // The requested amount exceeds whats offered
                                t = Trade { amount: o.amount, price: average_price, buyer: r.buyer.clone(), seller: o.seller.clone() };
                                off = offer_iter.next();

                                surplus_req = EnergyRequest { amount: r.amount.clone() - o.amount.clone(), price: r.price.clone(), buyer: r.buyer.clone() };
                                req = Some(&surplus_req);
                            }
                        }
                    } else {
                        t = Trade { amount: r.amount, price: grid_offer.price, buyer: r.buyer.clone(), seller: grid_offer.seller.clone() };
                        req = request_iter.next();
                    }
                    trades.push(t);
                }
                _ => break
            }
        }

        // Match remaining requests with grid
        while req.is_some() {
            let t = Trade { amount: req.unwrap().amount, price: grid_offer.price, buyer: req.unwrap().buyer.clone(), seller: grid_offer.seller.clone() };
            req = request_iter.next();
            trades.push(t);
        }

        // Match remaining offers with grid
        while off.is_some() {
            let t = Trade { amount: off.unwrap().amount, price: grid_request.price, buyer: grid_request.buyer.clone(), seller: off.unwrap().seller.clone() };
            off = offer_iter.next();
            trades.push(t);
        }

        trades
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EnergyRequest {
        pub amount: u16,
        pub price: f32,
        pub buyer: String,
    }

    impl Clone for EnergyRequest {
        fn clone(&self) -> Self {
            EnergyRequest {
                amount: self.amount.clone(),
                price: self.price.clone(),
                buyer: self.buyer.clone(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EnergyOffer {
        pub amount: u16,
        pub price: f32,
        pub seller: String,
    }

    impl Clone for EnergyOffer {
        fn clone(&self) -> Self {
            EnergyOffer {
                amount: self.amount.clone(),
                price: self.price.clone(),
                seller: self.seller.clone(),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Trade {
        pub amount: u16,
        pub price: f32,
        pub buyer: String,
        pub seller: String,
    }

    impl Clone for Trade {
        fn clone(&self) -> Self {
            Trade {
                amount: self.amount.clone(),
                price: self.price.clone(),
                buyer: self.buyer.clone(),
                seller: self.seller.clone(),
            }
        }
    }

    impl Hash for Trade {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.amount.hash(state);
            self.price.to_be_bytes().hash(state);
            self.buyer.hash(state);
            self.seller.hash(state);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn empty_list_returns_no_trades() {
            let mut requests = Vec::<EnergyRequest>::new();
            let mut offers = Vec::<EnergyOffer>::new();
            let grid_request = EnergyRequest { amount: 10, price: 2.2, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.2, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            assert_eq!(trades.len(), 0);
        }

        #[test]
        fn empty_offers_returns_request_trades() {
            let mut requests = Vec::<EnergyRequest>::from([
                EnergyRequest { amount: 10, price: 1.9, buyer: "buyer_1".into() },
                EnergyRequest { amount: 10, price: 2.0, buyer: "buyer_2".into() },
                EnergyRequest { amount: 10, price: 2.3, buyer: "buyer_3".into() }
            ]);
            let mut offers = Vec::<EnergyOffer>::new();
            let grid_request = EnergyRequest { amount: 10, price: 2.0, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.0, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            assert_eq!(trades.len(), 3);
        }

        #[test]
        fn empty_requests_returns_offer_trades() {
            let mut requests = Vec::<EnergyRequest>::new();
            let mut offers = Vec::<EnergyOffer>::from([
                EnergyOffer { amount: 10, price: 1.0, seller: "seller_1".into() },
                EnergyOffer { amount: 10, price: 2.0, seller: "seller_2".into() },
                EnergyOffer { amount: 10, price: 2.3, seller: "seller_3".into() }
            ]);
            let grid_request = EnergyRequest { amount: 10, price: 2.0, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.0, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            assert_eq!(trades.len(), 3);
        }

        #[test]
        fn requests_and_offers_are_matched_to_trades() {
            let mut requests = Vec::<EnergyRequest>::from([
                EnergyRequest { amount: 10, price: 1.9, buyer: "buyer_1".into() },
                EnergyRequest { amount: 10, price: 2.0, buyer: "buyer_2".into() },
                EnergyRequest { amount: 10, price: 2.3, buyer: "buyer_3".into() }
            ]);
            let mut offers = Vec::<EnergyOffer>::from([
                EnergyOffer { amount: 10, price: 1.9, seller: "seller_1".into() },
                EnergyOffer { amount: 10, price: 2.0, seller: "seller_2".into() },
                EnergyOffer { amount: 10, price: 2.3, seller: "seller_3".into() }
            ]);
            let grid_request = EnergyRequest { amount: 10, price: 2.0, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.0, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            assert_eq!(trades.len(), 3);
            assert!(trades.iter().all(|t| t.buyer.ne("grid") && t.seller.ne("grid")));
        }

        #[test]
        fn cheap_request_is_sold_to_the_grid_and_everything_else_is_peer_to_peer() {
            let mut requests = Vec::<EnergyRequest>::from([
                EnergyRequest { amount: 10, price: 1.9, buyer: "buyer_1".into() },
                EnergyRequest { amount: 10, price: 2.0, buyer: "buyer_2".into() },
                EnergyRequest { amount: 10, price: 2.1, buyer: "buyer_3".into() },
                EnergyRequest { amount: 10, price: 2.4, buyer: "buyer_4".into() },
                EnergyRequest { amount: 10, price: 2.8, buyer: "buyer_5".into() }
            ]);
            let mut offers = Vec::<EnergyOffer>::from([
                EnergyOffer { amount: 10, price: 2.0, seller: "seller_1".into() },
                EnergyOffer { amount: 10, price: 2.1, seller: "seller_2".into() },
                EnergyOffer { amount: 10, price: 2.3, seller: "seller_3".into() },
                EnergyOffer { amount: 5, price: 2.3, seller: "seller_4".into() },
                EnergyOffer { amount: 5, price: 2.7, seller: "seller_5".into() }
            ]);

            let grid_request = EnergyRequest { amount: 10, price: 2.0, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.0, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            // Assert that six trades are formed, only one involving the grid, and the grid is a seller
            assert_eq!(trades.len(), 6);
            assert_eq!(trades.iter().filter(|&t| t.buyer.eq("grid")).count(), 0);
            assert_eq!(trades.iter().filter(|&t| t.seller.eq("grid")).count(), 1);
        }

        #[test]
        fn offered_and_traded_amounts_match_including_grid() {
            let mut requests = Vec::<EnergyRequest>::from([
                EnergyRequest { amount: 10, price: 1.9, buyer: "buyer_1".into() },
                EnergyRequest { amount: 10, price: 2.0, buyer: "buyer_2".into() },
                EnergyRequest { amount: 10, price: 2.1, buyer: "buyer_3".into() },
                EnergyRequest { amount: 10, price: 2.4, buyer: "buyer_4".into() },
                EnergyRequest { amount: 10, price: 2.8, buyer: "buyer_5".into() }
            ]);
            let mut offers = Vec::<EnergyOffer>::from([
                EnergyOffer { amount: 10, price: 2.0, seller: "seller_1".into() },
                EnergyOffer { amount: 10, price: 2.1, seller: "seller_2".into() },
                EnergyOffer { amount: 10, price: 2.3, seller: "seller_3".into() },
                EnergyOffer { amount: 5, price: 2.3, seller: "seller_4".into() },
                EnergyOffer { amount: 5, price: 2.7, seller: "seller_5".into() }
            ]);

            let grid_request = EnergyRequest { amount: 10, price: 2.0, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.0, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            trades.iter().for_each(|t| println!("{}", t.amount));

            assert_eq!(offers.iter().map(|o| o.amount).sum::<u16>() + 10, trades.iter().map(|t| t.amount).sum::<u16>());
        }

        #[test]
        fn all_buyers_and_sellers_are_included() {
            let mut requests = Vec::<EnergyRequest>::from([
                EnergyRequest { amount: 10, price: 1.9, buyer: "buyer_1".into() },
                EnergyRequest { amount: 10, price: 2.0, buyer: "buyer_2".into() },
                EnergyRequest { amount: 10, price: 2.1, buyer: "buyer_3".into() },
                EnergyRequest { amount: 10, price: 2.4, buyer: "buyer_4".into() },
                EnergyRequest { amount: 10, price: 2.8, buyer: "buyer_5".into() }
            ]);
            let mut offers = Vec::<EnergyOffer>::from([
                EnergyOffer { amount: 10, price: 2.0, seller: "seller_1".into() },
                EnergyOffer { amount: 10, price: 2.1, seller: "seller_2".into() },
                EnergyOffer { amount: 10, price: 2.3, seller: "seller_3".into() },
                EnergyOffer { amount: 5, price: 2.3, seller: "seller_4".into() },
                EnergyOffer { amount: 5, price: 2.7, seller: "seller_5".into() }
            ]);

            let grid_request = EnergyRequest { amount: 10, price: 2.0, buyer: "grid".into() };
            let grid_offer = EnergyOffer { amount: 10, price: 2.0, seller: "grid".into() };

            let trades = generate_trades(&mut requests, &mut offers, &grid_request, &grid_offer);

            // Check that all buyers are included in a trade
            let buyers: Vec<&String> = requests.iter().map(|r| &r.buyer).collect();
            let included_buyers: Vec<&String> = trades.iter().map(|t| &t.buyer).collect();
            assert!(buyers.iter().all(|&b| included_buyers.iter().any(|&inc_b| b == inc_b)));

            // Check that all sellers are included in a trade
            let sellers: Vec<&String> = offers.iter().map(|o| &o.seller).collect();
            let included_sellers: Vec<&String> = trades.iter().map(|t| &t.seller).collect();
            assert!(sellers.iter().all(|&s| included_sellers.iter().any(|&inc_s| s == inc_s)));
        }
    }
}