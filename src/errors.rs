use error_chain::error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        SVG(::svg::parser::Error);
        JSON(::serde_json::Error);
        IO(::std::io::Error);
        SVGTYPE(::svgtypes::Error);
    }
}
