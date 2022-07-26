use diesel::prelude::*;
use super::model::{Product, NewProduct};
use crate::schema::products;

pub fn get_product_by_id(id: i32, conn: &PgConnection) -> QueryResult<Product> { 
    products::table.filter(products::id.eq(id)).first(conn)
}

pub fn get_all_products(conn: &PgConnection) -> QueryResult<Vec<Product>> { 
    products::table.load(conn)
}
/// Get Products Created By This USer
pub fn get_all_products_by_id(id: i32, conn: &PgConnection) -> QueryResult<Vec<Product>> { 
    products::table.filter(products::created_by.eq(id)).load(conn)
}

pub fn create_product(new_product: NewProduct, conn: &PgConnection) -> QueryResult<Product> { 
    diesel::insert_into(products::table)
        .values(new_product)
        .get_result::<Product>(conn)
}

pub fn filter_by_category(category: String, conn: &PgConnection) -> QueryResult<Vec<Product>> { 
    products::table.filter(products::category.eq(category)).load(conn)
}

pub fn filter_by_tags(tags: String, conn: &PgConnection) -> QueryResult<Vec<Product>> { 
    products::table.filter(products::tags.eq(tags)).load(conn)
}

pub fn update_product(
    product_id: i32, 
    user_id: i32, 
    new_product: NewProduct, 
    conn: &PgConnection
) -> QueryResult<Product> { 
    //  Select Table and filter by user_id and find the product_id 
    diesel::update(
        products::table.filter(products::created_by.eq(user_id))
        .find(product_id)
    )
    .set(new_product)
    .get_result::<Product>(conn)
}

pub fn delete_product(product_id: i32, conn: &PgConnection) -> QueryResult<bool> { 
    diesel::delete(products::table)
        .filter(products::id.eq(product_id))
        .execute(conn)?;
    Ok(true)
}