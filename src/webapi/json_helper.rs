use serde::{Serialize, Serializer};

use crate::business::{self, shop::ShopProduct};

#[derive(Debug, Serialize)]
struct ShopProductSer<'a> {
  product_id: &'a str,
  coins: i32,
}
impl<'a> From<&'a ShopProduct> for ShopProductSer<'a> {
  fn from(product: &'a ShopProduct) -> Self {
    Self {
      product_id: product.product_id.as_str(),
      coins: product.coins,
    }
  }
}

struct ShopProducts<'a> {
  products: &'a [business::shop::ShopProduct],
}
impl Serialize for ShopProducts<'_> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.collect_seq(self.products.iter().map(ShopProductSer::from))
  }
}

pub fn to_json(
  products: &[business::shop::ShopProduct],
) -> Result<String, serde_json::Error> {
  let products = ShopProducts { products };
  serde_json::to_string(&products)
}
