const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const withMDX = require("@next/mdx")({
  extension: /\.mdx?$/,
  options: {
    remarkPlugins: [],
    rehypePlugins: [],
    // If you use `MDXProvider`, uncomment the following line.
    providerImportSource: "@mdx-js/react",
  },
});

const nextConfig = withMDX({
  reactStrictMode: true,
  webpack: (config, { isServer }) => {
    config.experiments = {
      layers: true,
      asyncWebAssembly: true,
      topLevelAwait: true,
    };
    config.output.webassemblyModuleFilename =
      (isServer ? "../" : "") + "static/wasm/webassembly.wasm";
    config.plugins.push(
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "../"),
      })
    );

    return config;
  },
});

module.exports = nextConfig;
