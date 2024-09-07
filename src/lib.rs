// https://modis.ornl.gov/data/modis_webservice.html

const BASE_URL: &str = "https://modis.ornl.gov/rst/api/v1/";

pub mod structs;

pub use structs::*;

// https://modis.ornl.gov/rst/api/v1/products
pub async fn products() -> Result<ProductsData, Box<dyn std::error::Error>> {
    // Python
    // response = requests.get('https://modis.ornl.gov/rst/api/v1/products', headers=header)
    // products = json.loads(response.text)['products']

    let response = reqwest::get(format!("{}/products", BASE_URL))
        .await?
        .json::<ProductsData>()
        .await?;

    Ok(response)
}

pub async fn dates(
    product: &str,
    latitude: f64,
    longitude: f64,
) -> Result<DatesWrapper, Box<dyn std::error::Error>> {
    // Python
    // response = requests.get('https://modis.ornl.gov/rst/api/v1/MOD11A2/dates?latitude=39.56499&longitude=-121.55527', headers=header)
    // dates = json.loads(response.text)['dates']

    // modis_dates = [i['modis_date'] for i in dates]
    // calendar_dates = [i['calendar_date'] for i in dates]

    // Print the URL used
    println!(
        "{}/{}/dates?latitude={}&longitude={}",
        BASE_URL, product, latitude, longitude
    );

    let response = reqwest::get(format!(
        "{}/{}/dates?latitude={}&longitude={}",
        BASE_URL, product, latitude, longitude
    ))
    .await?
    .json::<DatesWrapper>()
    .await?;

    Ok(response)
}

// Sites
// view-source:https://modis.ornl.gov/rst/api/v1/sites
pub async fn sites() -> Result<Sites, Box<dyn std::error::Error>> {
    // Python
    // response = requests.get('https://modis.ornl.gov/rst/api/v1/sites', headers=header)
    // sites = json.loads(response.text)['sites']

    let response = reqwest::get(format!("{}/sites", BASE_URL))
        .await?
        .json::<Sites>()
        .await?;

    Ok(response)
}

// Subset
// response = requests.get('https://modis.ornl.gov/rst/api/v1/MOD11A2/subset?
// latitude=39.56499&longitude=-121.55527&//
// band=LST_Day_1km&//
// startDate=A2001001&endDate=A2001001&
// kmAboveBelow=1&kmLeftRight=1', headers=header)
pub async fn subset(
    product: &str,
    latitude: f64,
    longitude: f64,
    band: &str,
    start_date: &str,
    end_date: &str,
    km_above_below: u8,
    km_left_right: u8,
) -> Result<ModisData, Box<dyn std::error::Error>> {
    // Python
    // response = requests.get('https://modis.ornl.gov/rst/api/v1/MOD11A2/subset?latitude=39.56499&longitude=-121.55527&band=LST_Day_1km&startDate=A2001001&endDate=A2001001&kmAboveBelow=1&kmLeftRight=1', headers=header)
    // subset = json.loads(response.text)

    let response = reqwest::get(format!("{}/{}/subset?latitude={}&longitude={}&band={}&startDate={}&endDate={}&kmAboveBelow={}&kmLeftRight={}", BASE_URL, product, latitude, longitude, band, start_date, end_date, km_above_below, km_left_right))
        .await?
        .json::<ModisData>()
        .await?;

    println!("{:?}", response);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dates() {
        dates(ProductType::MOD11A2.into(), 39.56499, -121.55527)
            .await
            .expect("Failed to fetch dates");
    }

    #[tokio::test]
    async fn test_products() {
        products().await.expect("Failed to fetch products");
    }

    #[tokio::test]
    async fn test_sites() {
        let a = sites().await.expect("Failed to fetch sites");
        assert!(a.sites.len() > 0);
    }

    #[tokio::test]
    async fn test_subset() {
        subset(
            ProductType::MOD11A2.into(),
            39.56499,
            -121.55527,
            "LST_Day_1km",
            "A2001001",
            "A2001001",
            1,
            1,
        )
        .await
        .expect("Failed to fetch subset");
        panic!();
    }
}
