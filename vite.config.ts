import type { UserConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import process, { env } from 'process';

const host: string = process.env['TAURI_DEV_HOST'] ?? 'localhost';

const environment: string =
	process.env['ENVIRONMENT']?.toLowerCase() ?? 'production';
const isProduction: boolean = environment === 'production';

export default {
	mode: environment,
	plugins: [sveltekit()],
	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr:
			host ?
				{
					protocol: 'ws',
					host,
					port: 1421,
				}
			:	undefined,
		watch: {
			ignored: ['**/src-tauri/**'],
		},
	},
	build: {
		minify: isProduction,
		cssMinify: isProduction,
	},
} satisfies UserConfig;
