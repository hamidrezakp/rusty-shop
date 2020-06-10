use crate::models::*;
use crate::schema;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;

pub enum DBResult {
    DuplicateUsername,
    Other,
}

pub fn get_all_products(connection: &SqliteConnection, limit: i64) -> Vec<Product> {
    use schema::products::dsl::*;

    products
        .limit(limit)
        .load::<Product>(connection)
        .expect("Error loading products")
}

pub fn get_product(connection: &SqliteConnection, in_id: i32) -> Product {
    use schema::products::dsl::*;

    products
        .filter(id.eq(in_id))
        .first::<Product>(connection)
        .expect("Error loading product")
}

pub fn get_all_customers(connection: &SqliteConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users
        .filter(access.eq(3))
        .load::<User>(connection)
        .expect("Error loading users")
}

pub fn get_all_orders(connection: &SqliteConnection) -> Vec<OrderWithPrice> {
    use schema::orders::dsl::*;

    let all_orders = orders
        .load::<Order>(connection)
        .expect("Error loading orders");

    all_orders
        .iter()
        .map(|item| OrderWithPrice::from_order(item, get_payment(connection, item.id)))
        .collect::<Vec<_>>()
}

pub fn get_payment(connection: &SqliteConnection, in_order_id: i32) -> f32 {
    use schema::payments::dsl::*;

    payments
        .select(amount)
        .filter(order_id.eq(in_order_id))
        .first(connection)
        .unwrap()
}

pub fn insert_user(connection: &SqliteConnection, in_user: FormUser) -> Result<(), DBResult> {
    use schema::users::dsl::*;

    let result = insert_into(users).values(&in_user).execute(connection);

    match result {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(UniqueViolation, _)) => {
            Err(DBResult::DuplicateUsername)
        }
        _ => Err(DBResult::Other),
    }
}

pub fn insert_and_get_payment(
    connection: &SqliteConnection,
    payment_amount: f32,
    order: i32,
    user: i32,
) -> Option<i32> {
    use schema::payments::dsl::*;

    let result = insert_into(payments)
        .values((
            datetime.eq(Utc::now().naive_utc()),
            amount.eq(payment_amount),
            order_id.eq(order),
            user_id.eq(user),
        ))
        .execute(connection);

    if let Ok(1) = result {
        Some(
            payments
                .select(id)
                .order(id.desc())
                .first::<i32>(connection)
                .expect("Error loading Payments id"),
        )
    } else {
        None
    }
}

pub fn insert_and_get_order(
    connection: &SqliteConnection,
    in_address: &String,
    phonenumber: &String,
) -> Option<i32> {
    use schema::orders::dsl::*;

    let result = insert_into(orders)
        .values((
            datetime.eq(Utc::now().naive_utc()),
            address.eq(in_address),
            phone.eq(phonenumber),
        ))
        .execute(connection);

    if let Ok(1) = result {
        Some(
            orders
                .select(id)
                .order(id.desc())
                .first::<i32>(connection)
                .expect("Error loading Orders id"),
        )
    } else {
        None
    }
}

pub fn get_products_price(connection: &SqliteConnection, product_pairs: &Vec<(i32, i32)>) -> f32 {
    use schema::products::dsl::*;

    let product_ids: Vec<i32> = product_pairs.iter().map(|pair| pair.0).collect();

    let mut products_prices = products
        .select((id, price))
        .filter(id.eq_any(product_ids.clone()))
        .load::<(i32, f32)>(connection)
        .expect("Error loading Products");

    products_prices.sort_by(|a, b| a.0.cmp(&b.0));

    product_pairs
        .iter()
        .zip(products_prices)
        .map(|item| (item.0).1 as f32 * (item.1).1)
        .sum::<f32>()
}

pub fn insert_products_with_order(
    connection: &SqliteConnection,
    in_products: &Vec<(i32, i32)>,
    in_order_id: i32,
) {
    use schema::ordered_products::dsl::*;

    let ordered_products_pair = in_products
        .iter()
        .map(|item| {
            (
                product_id.eq(item.0),
                order_id.eq(in_order_id),
                quantity.eq(item.1),
            )
        })
        .collect::<Vec<_>>();

    insert_into(ordered_products)
        .values(ordered_products_pair)
        .execute(connection)
        .unwrap();
}
