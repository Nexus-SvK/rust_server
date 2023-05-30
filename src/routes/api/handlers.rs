use actix_web::HttpResponse;

pub async fn get_users() -> HttpResponse {
    // Implementation for getting users
    HttpResponse::Ok().body("Get Users")
}

pub async fn create_user() -> HttpResponse {
    // Implementation for creating a user
    HttpResponse::Ok().body("Create User")
}

pub async fn get_products() -> HttpResponse {
    // Implementation for getting products
    HttpResponse::Ok().body("Get Products")
}

pub async fn create_product() -> HttpResponse {
    // Implementation for creating a product
    HttpResponse::Ok().body("Create Product")
}
