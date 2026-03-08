import reactHooks from 'eslint-plugin-react-hooks';
import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const babelParser = require('@babel/eslint-parser');

export default [
    {
        files: ["src/**/*.{js,jsx}"],
        languageOptions: {
            parser: babelParser,
            parserOptions: {
                requireConfigFile: false,
                babelOptions: { presets: ["@babel/preset-react"] }
            }
        },
        plugins: { 'react-hooks': reactHooks },
        rules: { ...reactHooks.configs.recommended.rules }
    }
];
