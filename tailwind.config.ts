import containerQueries from '@tailwindcss/container-queries';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import type { Config } from 'tailwindcss';
import colors from 'tailwindcss/colors';
import defaultTheme from 'tailwindcss/defaultTheme';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			colors: {
				gray: colors.slate,
				primary: colors.sky,
				secondary: colors.teal,
				tertiary: colors.indigo,
				contrast: colors.amber,
			},
		},
		fontFamily: {
			sans: ['Roboto', ...defaultTheme.fontFamily.sans],
			serif: ['Comfortaa', ...defaultTheme.fontFamily.serif],
			mono: ['Source Code Pro', ...defaultTheme.fontFamily.mono],
		},
		letterSpacing: {
			tighter: '0.00ch',
			tight: '0.05ch',
			normal: '0.10ch',
			wide: '0.15',
			wider: '.20em',
			widest: '.25em',
		},
	},

	plugins: [typography, forms, containerQueries],
} as Config;
