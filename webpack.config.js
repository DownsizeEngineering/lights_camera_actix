const path = require('path');

module.exports = {
    mode: "production",
    entry: path.resolve(__dirname, 'client', 'src', 'index.js'),
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname, 'client', 'public')
    },
    resolve: {
        alias: {
            svelte: path.resolve('node_modules', 'svelte')
        },
        extensions: ['.mjs', '.js', '.svelte'],
        mainFields: ['svelte', 'browser', 'module', 'main']
    },
    module: {
        rules: [
            {
                test: /\.(html|svelte)$/,
                exclude: /node_modules/,
                use: 'svelte-loader'
            }
        ],
    },
}