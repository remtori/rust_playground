const { config } = require('@swc/core/spack');

module.exports = config({
    entry: {
        main: __dirname + '/src/main.ts',
    },
    output: {
        path: __dirname + '/dist',
    },
    options: {
        jsc: {
            parser: {
                syntax: 'typescript'
            },
            target: 'es2017',
            loose: true,
            keepClassNames: false
        },
        sourceMaps: true,
        minify: true
    }
});
