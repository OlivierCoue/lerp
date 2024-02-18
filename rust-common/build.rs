extern crate prost_build;

fn main() {
    dotenvy::dotenv().unwrap();
    std::env::set_var("OUT_DIR", "src/proto");
    let proto_gen = std::env::var("PROTO_GEN").expect("env var PROTO_GEN is not set");

    if proto_gen == "true" {
        prost_build::compile_protos(
            &[
                "src/proto/common.proto",
                "src/proto/udp-down.proto",
                "src/proto/udp-up.proto",
                "src/proto/http-auth.proto",
            ],
            &["src/proto"],
        )
        .unwrap();
    }
}
