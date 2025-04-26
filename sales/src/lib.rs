#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, product_name: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| name == &product_name) {
            self.items.push(product.clone());
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();

        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut final_prices = Vec::new();

        let mut i = 0;
        while i < prices.len() {
            let end = usize::min(i + 3, prices.len());
            let chunk = &prices[i..end];

            if chunk.len() == 3 {
                let cheapest = chunk.iter().cloned().fold(f32::INFINITY, f32::min);
                let total: f32 = chunk.iter().sum();
                let discount_ratio = (total - cheapest) / total;

                let mut adjusted_chunk: Vec<f32> = chunk
                    .iter()
                    .map(|price| (price * discount_ratio * 100.0).round() / 100.0)
                    .collect();

                final_prices.append(&mut adjusted_chunk);
            } else {
                final_prices.append(&mut chunk.to_vec());
            }

            i += 3;
        }

        final_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = final_prices.clone();
        final_prices
    }
}