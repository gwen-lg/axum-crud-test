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

pub fn to_json(
  products: &[business::shop::ShopProduct],
) -> Result<String, serde_json::Error> {
  let mut products_json = Vec::new();
  let mut products_serializer = serde_json::Serializer::new(&mut products_json);
  products_serializer.collect_seq(products.iter().map(ShopProductSer::from))?;
  let products = String::from_utf8(products_json)
    .expect("invalid utf8 in json serialization");
  Ok(products)
}
