import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';

export function swap_rpc(from: string, target: string) {
  const regex = new RegExp(`import.{0,500} from ["'].{0,500}\/(${from})\/.{0,500}["']`);
  return {
    name: 'swap_rpc',
    transform(src: string, id: string) {
      const match = regex.exec(src);

      if (match !== null && match[1] !== undefined) {
        const [allLine, part] = match;
        const newImport = allLine.replace(`/${from}/`, `/${target}/`);

        return {
          code: src.replace(allLine, newImport),
          map: null,
        }
      }
    },
  }
}

export default defineConfig({
  plugins: [swap_rpc("test_rpc", "test_rpc_2"), sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
  }
});
