pub mod client {
    #[derive(Debug, borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub enum Request {
        Ping(u64),
    }

    pub fn request_init() -> Request {
        Request::Ping(0)
    }

    pub fn request_next(resp: &crate::program::Response) -> Request {
        match resp {
            crate::program::Response::Pong(i) => Request::Ping(i + 1),
        }
    }
}

pub mod program {
    #[derive(Debug, borsh::BorshSerialize, borsh::BorshDeserialize)]
    pub enum Response {
        Pong(u64),
    }

    pub fn response(req: &crate::client::Request) -> Response {
        match req {
            crate::client::Request::Ping(i) => Response::Pong(i + 1),
        }
    }

    pub fn response_data_size() -> usize {
        use borsh::BorshSerialize; // To access try_to_vec.
        Response::Pong(u64::MAX).try_to_vec().unwrap().len()
    }
}
