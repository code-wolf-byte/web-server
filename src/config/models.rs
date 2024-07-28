
enum service_type{
    REVERSE_PROXY,
    STATIC,
}
struct service{
    service: service_type,
    entry: String
}
struct block {
    host: String,
    services: Vec<service>
}