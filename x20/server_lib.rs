extern crate grpc;
extern crate largetable_client;
extern crate x20_grpc_rust as x20;

use largetable_client::LargeTableClient;

const BINARY_VERSIONS: &'static str = "x20::binary_versions";
const BINARIES: &'static str = "x20::binaries";

#[derive(Clone)]
pub struct X20ServiceHandler<C: LargeTableClient> {
    database: C,
}

impl<C: LargeTableClient + Clone> X20ServiceHandler<C> {
    pub fn new(db: C) -> Self {
        Self { database: db }
    }

    fn get_binaries(&self) -> x20::GetBinariesResponse {
        let bin_iter = largetable_client::LargeTableScopedIterator::new(
            &self.database,
            String::from(BINARIES),
            String::from(""),
            String::from(""),
            String::from(""),
            0,
        );
        let mut response = x20::GetBinariesResponse::new();
        for (_, bin) in bin_iter {
            response.mut_binaries().push(bin);
        }
        response
    }

    fn publish_binary(&self, mut req: x20::PublishBinaryRequest) -> x20::PublishBinaryResponse {
        let name = req.get_binary().get_name().to_owned();

        if req.get_binary().get_name().is_empty() {
            eprintln!("cannot publish empty binary name");
            return x20::PublishBinaryResponse::new();
        }
        let version = self
            .database
            .reserve_id(BINARY_VERSIONS, req.get_binary().get_name());

        let mut binary = req.take_binary();
        binary.set_version(version);

        self.database.write_proto(BINARIES, &name, 0, &binary);

        x20::PublishBinaryResponse::new()
    }
}

impl<C: LargeTableClient + Clone> x20::X20Service for X20ServiceHandler<C> {
    fn get_binaries(
        &self,
        _: grpc::RequestOptions,
        _req: x20::GetBinariesRequest,
    ) -> grpc::SingleResponse<x20::GetBinariesResponse> {
        grpc::SingleResponse::completed(self.get_binaries())
    }

    fn publish_binary(
        &self,
        _: grpc::RequestOptions,
        req: x20::PublishBinaryRequest,
    ) -> grpc::SingleResponse<x20::PublishBinaryResponse> {
        grpc::SingleResponse::completed(self.publish_binary(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate largetable_test;

    fn create_test_handler() -> X20ServiceHandler<largetable_test::LargeTableMockClient> {
        let db = largetable_test::LargeTableMockClient::new();
        X20ServiceHandler::new(db)
    }

    #[test]
    fn test_publish() {
        let handler = create_test_handler();
        let mut req = x20::PublishBinaryRequest::new();
        req.mut_binary().set_name(String::from("vim"));
        req.mut_binary().set_url(String::from("http://google.com"));
        req.mut_binary().set_target(String::from("//vim:vim"));

        handler.publish_binary(req);

        // Should be able to read that back
        let response = handler.get_binaries();
        assert_eq!(response.get_binaries().len(), 1);
        assert_eq!(response.get_binaries()[0].get_name(), "vim");
    }
}
