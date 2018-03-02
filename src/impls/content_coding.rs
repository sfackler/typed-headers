token! {
    ContentCoding, InvalidContentCoding => {
        BROTLI => "br" => [],
        GZIP => "gzip" => ["x-gzip"],
        DEFLATE => "deflate" => [],
        COMPRESS => "compress" => ["x-compress"],
        IDENTITY => "identity" => [],
    }
}
