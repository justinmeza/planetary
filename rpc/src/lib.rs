pub mod client;
pub mod client_v1;
pub mod server;
pub mod server_v1;
// pub mod server_v1_5;
pub mod server_v2;
pub mod server_v3;

// pub use server::*;
// pub use client::*;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type ProcedureId = i32;
pub type Payload = String;

#[derive(Debug, Clone)]
pub struct Request {
    pub procedure_id: ProcedureId,
    pub payload: Payload,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub payload: Payload,
}

// v3

impl Response {
    fn default() -> Response {
        Response { payload: "OK".to_string() }
    }
}

// trait Middleware {
//     fn handle(&self, request: &Request, next: &dyn Fn(&Request) -> Response) -> Response;
// }

// type NextFn = Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>
//     + Send + Sync>;

// pub trait Handler: Send + Sync {
//     fn handle<'a>(
//         &'a self,
//         request: &Request,
//         next: &'a NextFn,
//         // next: &'a dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>
//     ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;
// }

pub trait Handler: Send + Sync {
    // fn handle(
    //     &self,
    //     request: Request,
    //     node: &HandlerNode,
    // ) -> Pin<Box<dyn Future<Output = Response> + Send>>;
    fn handle<'a>(
        &'a self,
        request: Request,
        node: &'a HandlerNode,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'a>>;
}

pub struct HandlerNode {
    handler: Arc<Mutex<dyn Handler>>,
    next: Option<Arc<HandlerNode>>,
}

impl HandlerNode {
    pub fn new(handler: Arc<Mutex<dyn Handler>>) -> Self {
        HandlerNode {
            handler,
            next: None,
        }
    }

    pub async fn call_next(&self, request: Request) -> Response {
        match &self.next {
            Some(next_node) => next_node.handle(request).await,
            None => Response::default(), // or some other default handling
        }
    }

    // async fn handle(&self, request: Request) -> Response {
    //     match &self.next {
    //         Some(next_node) => {
    //             self.handler.handle(&request, &|req| Box::pin(async move {
    //                 next_node.handle(req).await
    //             })).await
    //         }
    //         None => {
    //             Response::default()
    //         }
    //     }
    // }
    // async fn handle(&self, request: Request) -> Response {
    //     self.handler.handle(&request, &HandlerNode::call_next).await
    // }

    async fn handle(&self, request: Request) -> Response {
        self.handler.lock().await.handle(request, self).await
    }

    // async fn handle(&self, request: Request) -> Response {
    //     match &self.next {
    //         Some(next_node) => {
    //             let next_closure: Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync> = Box::new(move |req| {
    //                 let next_node_clone = next_node.clone();
    //                 Box::pin(async move { next_node_clone.handle(req).await })
    //             });

    //             self.handler.handle(&request, &next_closure).await
    //         }
    //         None => {
    //             Response::default()
    //         }
    //     }
    // }
}

#[derive(Clone)]
pub struct Handlers {
    handler_nodes: Option<Arc<HandlerNode>>,
    // handlers: Vec<Arc<dyn Handler>>,
}

impl Handlers {
    // fn handle_request(&self, request: &Request) -> Response {
    //     let mut response = Response::default();
    //     for middleware in &self.middlewares {
    //         response = middleware.handle(request, &|req| response.clone());
    //     }
    //     response
    // }
    pub fn new() -> Self {
        Handlers {
            handler_nodes: None,
        }
    }

    // fn handle_request(
    //     &self,
    //     request: Request
    // ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
    //     let handlers = self.handlers.clone();
    //     Box::pin(async move {
    //         let mut response = Response::default();
    //         // let next: NextFn = Box::new(|request| Box::pin(async move { Response::default() }));
    //         // let next = |req| Box::pin(async { Response::default() }) as Pin<Box<_>>;

    //         handlers.handle
    //         for handler in handlers {
    //             // response = handler.handle(request, &next).await;
    //             response = handler.handle(&request).await;
    //         }
    //         response
    //     })
    // }

    pub async fn handle_request(&self, request: Request) -> Response {
        match &self.handler_nodes {
            Some(node) => {
                node.handler.lock().await.handle(request, node).await
            }
            None => {
                Response::default()
            }
        }
    }

    // pub fn add_handler(&mut self, handler: Arc<dyn Handler>) {
    //     self.handlers.push(handler);
    // }

    pub fn add_handler(&mut self, handler: Arc<Mutex<dyn Handler>>) {
        let mut node = HandlerNode::new(handler);

        match self.handler_nodes.take() {
            Some(existing) => {
                node.next = Some(existing);
                self.handler_nodes = Some(Arc::new(node));
            }
            None => {
                self.handler_nodes = Some(Arc::new(node));
            }
        }
    }
}
