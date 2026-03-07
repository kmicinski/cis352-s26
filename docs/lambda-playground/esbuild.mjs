import * as esbuild from 'esbuild';

// Rewrite the WASM import path for the output location (www/app.js)
const rewriteWasmImport = {
    name: 'rewrite-wasm-import',
    setup(build) {
        build.onResolve({ filter: /lambda_viz\.js$/ }, () => ({
            path: './pkg/lambda_viz.js',
            external: true,
        }));
    },
};

await esbuild.build({
    entryPoints: ['www/src/app.ts'],
    bundle: true,
    outfile: 'www/app.js',
    format: 'esm',
    target: 'es2022',
    sourcemap: true,
    minify: false,
    plugins: [rewriteWasmImport],
});
