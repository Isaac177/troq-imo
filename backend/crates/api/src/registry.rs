use axum::Router;
use kernel::kernel::Kernel;
use utoipa::openapi::OpenApi;

macro_rules! register {
    ($($module:ident),* $(,)?) => {
        pub fn public() -> Router<Kernel> {
            Router::new()$(.merge(domain::$module::public_routes::<Kernel>()))*
        }

        pub fn guest() -> Router<Kernel> {
            Router::new()$(.merge(domain::$module::guest_routes::<Kernel>()))*
        }

        pub fn host() -> Router<Kernel> {
            Router::new()$(.merge(domain::$module::host_routes::<Kernel>()))*
        }

        pub fn admin() -> Router<Kernel> {
            Router::new()$(.merge(domain::$module::admin_routes::<Kernel>()))*
        }

        pub fn documents() -> Vec<OpenApi> {
            vec![$(domain::$module::openapi()),*]
        }
    };
}

register! {
    guests,
    reviews,
    notifications,
    pricing,
    billing,
    inventory,
    reservations,
}
