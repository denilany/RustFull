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
        if self.items.is_empty() {
            self.receipt = Vec::new();
            return Vec::new();
        }

        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();

        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut discount_prices = Vec::new();
        
        let free_items_count = prices.len() / 3;

        if free_items_count > 0 {
            let net_price = prices.iter().sum::<f32>();

            let mut free_items_value = 0.0;
            for i in 0..free_items_count {
                free_items_value += prices[i];
            }

            let net_after_discount = net_price - free_items_value;

            // Discount factor
            let discount_factor = net_after_discount / net_price;

            // Apply discount to all items
            for &price in &prices {
                let discounted = (price * discount_factor * 100.0).round() / 100.0;
                discount_prices.push(discounted);
            }
        } else {
            discount_prices = prices;
        }

        self.receipt = discount_prices.clone();
        discount_prices
    }
}