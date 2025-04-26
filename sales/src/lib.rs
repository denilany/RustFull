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

    pub fn insert_item(&mut self, store: &Store, product_name: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| name == &product_name) {
            self.items.push(product.clone());
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = prices.len();
        if n < 3 {
            // no discount
            let mut no_discount = prices.clone();
            no_discount.sort_by(|a, b| a.partial_cmp(b).unwrap());
            self.receipt = no_discount.clone();
            return no_discount;
        }

        let num_discounts = n / 3; // how many items are free
        let discount_prices: f32 = prices.iter().take(num_discounts).sum();
        let total: f32 = prices.iter().sum();

        let discount_ratio = (total - discount_prices) / total;

        // Apply discount ratio to all items
        let mut adjusted_prices: Vec<f32> = prices
            .iter()
            .map(|price| (price * discount_ratio * 100.0).round() / 100.0)
            .collect();

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}
