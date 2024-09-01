use cyndra_service::{Error, Service};

#[derive(Clone)]
pub struct MyService;

#[cyndra_service::async_trait]
impl Service for MyService {
    async fn bind(
        mut self: Box<Self>,
        _addr: std::net::SocketAddr,
    ) -> Result<(), cyndra_service::error::Error> {
        println!("service is binding");
        Ok(())
    }
}

#[cyndra_service::main]
async fn cyndra() -> Result<MyService, Error> {
    Ok(MyService {})
}

// async fn __cyndra_wrapper(
//     _factory: &mut dyn cyndra_service::Factory,
//     runtime: &cyndra_service::Runtime,
//     logger: Box<dyn cyndra_service::log::Log>,
// ) -> Result<Box<dyn Service>, Error> {
//     runtime
//         .spawn_blocking(move || {
//             cyndra_service::log::set_boxed_logger(logger)
//                 .map(|()| {
//                     cyndra_service::log::set_max_level(cyndra_service::log::LevelFilter::Info)
//                 })
//                 .expect("logger set should succeed");
//         })
//         .await
//         .map_err(|e| {
//             if e.is_panic() {
//                 let mes = e
//                     .into_panic()
//                     .downcast_ref::<&str>()
//                     .map(|x| x.to_string())
//                     .unwrap_or_else(|| "<no panic message>".to_string());

//                 cyndra_service::Error::BuildPanic(mes)
//             } else {
//                 cyndra_service::Error::Custom(cyndra_service::error::CustomError::new(e))
//             }
//         })?;

//     runtime
//         .spawn(async {
//             cyndra()
//                 .await
//                 .map(|ok| Box::new(ok) as Box<dyn cyndra_service::Service>)
//         })
//         .await
//         .map_err(|e| {
//             if e.is_panic() {
//                 let mes = e
//                     .into_panic()
//                     .downcast_ref::<&str>()
//                     .map(|x| x.to_string())
//                     .unwrap_or_else(|| "<no panic message>".to_string());

//                 cyndra_service::Error::BuildPanic(mes)
//             } else {
//                 cyndra_service::Error::Custom(cyndra_service::error::CustomError::new(e))
//             }
//         })?
// }

// fn __binder(
//     service: Box<dyn cyndra_service::Service>,
//     addr: std::net::SocketAddr,
//     runtime: &cyndra_service::Runtime,
// ) -> cyndra_service::ServeHandle {
//     runtime.spawn(async move { service.bind(addr).await })
// }

// #[no_mangle]
// pub extern "C" fn _create_service() -> *mut cyndra_service::Bootstrapper {
//     let builder: cyndra_service::StateBuilder<Box<dyn cyndra_service::Service>> =
//         |factory, runtime, logger| Box::pin(__cyndra_wrapper(factory, runtime, logger));

//     let bootstrapper = cyndra_service::Bootstrapper::new(
//         builder,
//         __binder,
//         cyndra_service::Runtime::new().unwrap(),
//     );

//     let boxed = Box::new(bootstrapper);
//     Box::into_raw(boxed)
// }
