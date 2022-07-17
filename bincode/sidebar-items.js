window.SIDEBAR_ITEMS = {"enum":[["ErrorKind","The kind of error that can be produced during a serialization or deserialization."]],"fn":[["config","Get a default configuration object."],["deserialize","Deserializes a slice of bytes into an instance of `T` using the default configuration."],["deserialize_from","Deserializes an object directly from a `Read`er using the default configuration."],["deserialize_from_custom","Deserializes an object from a custom `BincodeRead`er using the default configuration. It is highly recommended to use `deserialize_from` unless you need to implement `BincodeRead` for performance reasons."],["options","Get a default configuration object."],["serialize","Serializes a serializable object into a `Vec` of bytes using the default configuration."],["serialize_into","Serializes an object directly into a `Writer` using the default configuration."],["serialized_size","Returns the size that an object would be if serialized using Bincode with the default configuration."]],"mod":[["config","`bincode` uses a Builder-pattern to configure the Serializers and Deserializers in this crate. This means that if you need to customize the behavior of `bincode`, you should create an instance of the `DefaultOptions` struct:"],["de","Deserialize bincode data to a Rust data structure."]],"struct":[["Config","A configuration builder whose options Bincode will use while serializing and deserializing."],["Serializer","An Serializer that encodes values directly into a Writer."]],"type":[["Error","An error that can be produced during (de)serializing."],["Result","The result of a serialization or deserialization operation."]]};