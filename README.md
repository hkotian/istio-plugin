## Sharding plugin for streamer

POC to see how sharding would work when we use envoy.

### To run this locally

Make sure you have your services running locally
Update the `envoy.yaml` to have the service ports listed correctly. If needed add more cluster endpoints.

Build the plugin using 
```shell
cargo build --target wasm32-wasip1 --release
```

Run envoy locally with this plugin
```shell
docker-compose up
```