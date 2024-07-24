// cfg changes how the code will be compiled
#[cfg(not(target_arch = "wasm32"))]
pub mod db_pool;
#[cfg(not(target_arch = "wasm32"))]
pub mod functions;
pub mod schema;

pub mod structs;

#[cfg(test)]
pub mod tests;
