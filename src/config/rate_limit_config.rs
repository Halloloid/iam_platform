use std::{sync::Arc, time::Duration};

use axum::body::Body;
use governor::{middleware::StateInformationMiddleware};
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
};

pub fn auth_rate_limiter()
-> GovernorLayer<PeerIpKeyExtractor, StateInformationMiddleware, Body> {
    let config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(10)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let limiter = config.limiter().clone();

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(60));
            limiter.retain_recent();
        }
    });

    GovernorLayer::new(config)
}

pub fn login_rate_limiter() ->GovernorLayer<PeerIpKeyExtractor,StateInformationMiddleware,Body> {
    let config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let limiter = config.limiter().clone();

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(60));
            limiter.retain_recent();
        }
    });

    GovernorLayer::new(config)
}
