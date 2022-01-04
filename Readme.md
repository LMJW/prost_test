# test project of how to use prost 

This is a simple repo that show you a basic setup of how to use prost to compile the protobuf file, and encode and decode the protobuf binaries.

## Detail steps

1. setup the `build.rs` like in this repo. You can find more details in the [guide](https://github.com/tokio-rs/prost).

2. once the the protobuf got compiled properly, you will be able to import the file. You can see the lib.rs to see how to setup the module to import it.

3. you can import the trait "prost::Message" when you need to use the encode/decode methods. See the test in the lib.rs for more details.