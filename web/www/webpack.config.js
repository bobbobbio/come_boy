const path = require('path');

module.exports = {
    entry: './src/index.js',
    mode: 'development',
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true
    },
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'public'),
    },
    devServer: {
        allowedHosts: "all",
    },
    watchOptions: {
        poll: 1000,
    },
}
