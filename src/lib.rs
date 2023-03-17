use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Price {
    timestamp: Instant,
    value: f64,
}

impl Eq for Price {}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .value
            .partial_cmp(&self.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug, PartialEq,Clone)]
pub struct OHLC {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl OHLC {
    pub fn new(price: &Price) -> OHLC {
        OHLC {
            open: price.value,
            high: price.value,
            low: price.value,
            close: price.value,
        }
    }

    pub fn update(&mut self, price: &Price) {
        self.close = price.value;
        self.high = self.high.max(price.value);
        self.low = self.low.min(price.value);
    }
}

#[derive(Clone)]
pub struct RollingOHLC {
    window: Duration,
    prices: Vec<Price>,
    ohlc: Option<OHLC>,
}

impl RollingOHLC {
    pub fn new(window: Duration) -> RollingOHLC {
        RollingOHLC {
            window,
            prices: vec![],
            ohlc: None,
        }
    }

    pub fn push(&mut self, price: Price) -> Option<&OHLC> {
        let now = Instant::now();
        self.prices.retain(|p| now - p.timestamp <= self.window);
        self.prices.push(price);
        self.prices.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
        let mut heap = BinaryHeap::new();
        let mut ohlc = OHLC::new(&self.prices[0]);
        for price in &self.prices {
            heap.push(price);
            while !heap.is_empty() && now - heap.peek().unwrap().timestamp > self.window {
                heap.pop();
            }
            if let Some(p) = heap.peek() {
                ohlc.update(p);
            }
        }
        self.ohlc = Some(ohlc);
        self.ohlc.as_ref().clone()
    }

    pub fn get(&self) -> Option<&OHLC> {
        self.ohlc.as_ref().clone()
    }
}

