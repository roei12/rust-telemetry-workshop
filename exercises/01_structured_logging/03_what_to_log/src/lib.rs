mod logger;

use std::time::Instant;

pub use logger::TestLogger;

/// Given a list of order numbers, compute the total price.
///
/// # Exercise
///
/// Add log statements to `get_total` and `get_order_details`, our two units of work, to capture
/// the data points we discussed:
/// - the start and end of each unit of work
/// - the duration of each unit of work
/// - the outcome of each unit of work
///
/// Refer to the test files for the expected output format.
pub fn get_total(order_numbers: &[u64]) -> Result<u64, anyhow::Error> {
    log::info!("START - process total price");
    let starting_time = Instant::now();
    let mut sum = 0u64;

    for order in order_numbers {
        match get_order_details(*order) {
            Ok(details) => sum += details.price,
            Err(e) => {
                log::error!(
                    "END - process total price - ERROR - {}ms",
                    starting_time.elapsed().as_millis()
                );
                anyhow::bail!(e);
            }
        }
    }

    log::info!(
        "END - process total price - SUCCESS - {}ms",
        starting_time.elapsed().as_millis()
    );
    Ok(sum)
}

pub struct OrderDetails {
    pub order_number: u64,
    pub price: u64,
}

/// A dummy function to simulate what would normally be a database query.
fn get_order_details(order_number: u64) -> Result<OrderDetails, anyhow::Error> {
    log::info!("START - retrieve order");
    let starting_time = Instant::now();
    let prices = vec![999, 1089, 1029];

    let result = if order_number % 4 == 0 {
        log::error!(
            "END - retrieve order - ERROR - {}ms",
            starting_time.elapsed().as_millis()
        );
        Err(anyhow::anyhow!("Failed to talk to the database"))
    } else {
        let price = prices[order_number as usize % prices.len()];
        log::info!(
            "END - retrieve order - SUCCESS - {}ms",
            starting_time.elapsed().as_millis()
        );
        Ok(OrderDetails {
            order_number,
            price,
        })
    };

    result
}
