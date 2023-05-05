fn main() {
    embed_resource::compile("resources/atxlogo.ico", embed_resource::NONE);
    embed_resource::compile("foo-manifest.rc", embed_resource::NONE);
}
