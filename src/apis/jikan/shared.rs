use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use governor::{
            Quota, RateLimiter,
            clock::DefaultClock,
            state::{InMemoryState, NotKeyed},
        };
        use once_cell::sync::Lazy;
        use std::{num::NonZeroU32, sync::Arc};

        pub static RATE_LIMITER_PER_SECOND: Lazy<Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>> =
            Lazy::new(|| {
                let quota = Quota::per_second(NonZeroU32::new(3).unwrap());
                Arc::new(RateLimiter::direct(quota))
            });

        pub static RATE_LIMITER_PER_MINUTE: Lazy<Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>> =
            Lazy::new(|| {
                let quota = Quota::per_minute(NonZeroU32::new(60).unwrap());
                Arc::new(RateLimiter::direct(quota))
            });
    }
}
