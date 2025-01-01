fn main() {
    let _res = tonic_build::configure().build_server(false).compile_protos(
        &["kachaka-api/protos/kachaka-api.proto"],
        &["kachaka-api/protos"],
    );

    println!("{:?}", _res);
}
