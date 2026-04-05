
#[cfg(test)]
mod test {
    use super::{*, prelude::v1::*};

    const MANIFEST_DIR: &Path = Path::new(env!("CARGO_MANIFEST_DIR"));

    #[tokio::test]
    async fn example_1() {
        use crate::langs::RustModuleExt;
        let module = module::Module::binary()
            .identifier("example_1")
            .name("example 1")
            .description("an example for test abuild")
            .version("0.0.1")
            .dir(MANIFEST_DIR.join("tests/example_1"))
            .source_dir("src")
            .output_dir("output")
            .target_dir("output/target")
            .cache_dir("output/cache")
            .main("main.rs")
            .rust_edition(langs::rust::Edition::_2024)
            .rustc(langs::rust::ComplierSelector::Host)
            .cargo(langs::rust::CargoSelector::Host);

        // (prelude) build::ModuleHostBuilderExt
        let builder = module.host_builder().unwarp();
        builder.build().await.unwarp();

        // (prelude) run::ModuleHostRunnerExt
        let runner = module.host_runner().unwarp();
        runner.run().await.unwarp(); // run ${target_dir}/host/example_1
    }
}

