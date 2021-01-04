use warp::Rejection;

pub mod health;
pub mod metrics;
pub mod publish;
pub mod subscribe;

type Responce<T> = std::result::Result<T, Rejection>;
