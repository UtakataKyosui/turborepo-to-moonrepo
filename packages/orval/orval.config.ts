import { defineConfig } from 'orval';

export default defineConfig({
    'myservice-file-transfomer': {
        input: './openapi.yaml',
        output: {
            target: './src/generated.ts',
            schemas: './src/schemas',
            mock: true
        },
        hooks: {
            afterAllFilesWrite: "biome check --write ./src"
        }
    }
})