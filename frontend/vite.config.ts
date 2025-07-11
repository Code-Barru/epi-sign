import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
        proxy: {
            '/api': {
                target: 'http://localhost:3000', // Ajustez selon votre backend
                changeOrigin: true,
            }
        },
        allowedHosts: ["cfcd-82-65-111-80.ngrok-free.app"]
    }
});
